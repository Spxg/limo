use std::path::Path;

use anyhow::{Context, Result};
use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, header},
    middleware::{self, Next},
    response::{Html, Response},
    routing::get,
    serve::ListenerExt,
};
use tokio::{net::TcpListener, task::JoinHandle};
use tower_http::services::{ServeDir, ServeFile};
use url::Url;

use crate::InputType;

pub fn script_router(path: &Path, input_type: InputType) -> Result<Router> {
    let root = path
        .parent()
        .context("JavaScript file has no parent directory")?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("JavaScript filename is not valid UTF-8")?;

    // Use a placeholder base to URL-encode the filename; only the path is used below.
    let mut entry_url = Url::parse("http://localhost/")?;
    entry_url
        .path_segments_mut()
        .map_err(|()| anyhow::anyhow!("failed to create JavaScript entry URL"))?
        .push(name);
    let entry_path = entry_url.path();
    let html = entry_html(entry_path, input_type);

    Ok(Router::new()
        .without_v07_checks()
        .route(
            "/",
            get(move || {
                let html = html.clone();
                async move { Html(html) }
            }),
        )
        .route_service(
            entry_path,
            ServeFile::new_with_mime(path, &"text/javascript".parse()?),
        )
        .fallback_service(ServeDir::new(root))
        .layer(middleware::from_fn(disable_cache)))
}

pub fn source_router(source: String, name: &str, input_type: InputType) -> Router {
    let entry_path = format!("/{name}");
    let html = entry_html(&entry_path, input_type);
    Router::new()
        .route(
            "/",
            get(move || {
                let html = html.clone();
                async move { Html(html) }
            }),
        )
        .route(
            &entry_path,
            get(move || {
                let source = source.clone();
                async move { ([(header::CONTENT_TYPE, "text/javascript")], source) }
            }),
        )
        .fallback_service(ServeDir::new("."))
        .layer(middleware::from_fn(disable_cache))
}

fn entry_html(entry_path: &str, input_type: InputType) -> String {
    let entry_url = entry_path.replace('&', "&amp;");
    let script_attributes = match input_type {
        InputType::Script => "defer",
        InputType::Module => "type=\"module\"",
    };
    format!(
        "<!doctype html><html><head>\
         <script {script_attributes} src=\"{entry_url}\"></script>\
         </head><body></body></html>"
    )
}

async fn disable_cache(mut request: Request, next: Next) -> Response {
    request.headers_mut().remove(header::IF_MODIFIED_SINCE);
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

pub struct LocalServer {
    base_url: String,
    task: JoinHandle<std::io::Result<()>>,
}

impl LocalServer {
    pub async fn start(app: Router, port: u16) -> Result<Self> {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port))
            .await
            .context("failed to start local server")?;
        let address = listener.local_addr()?;
        let base_url = format!("http://{address}");
        let listener = listener.tap_io(|stream| {
            // Send small script responses without waiting for TCP acknowledgements.
            let _ = stream.set_nodelay(true);
        });
        let task = tokio::spawn(async move { axum::serve(listener, app).await });

        Ok(Self { base_url, task })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

impl Drop for LocalServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}
