use std::path::Path;

use anyhow::{Context, Result, ensure};
use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, header},
    middleware::{self, Next},
    response::{Html, Response},
    routing::get,
};
use tokio::{net::TcpListener, task::JoinHandle};
use tower_http::services::ServeDir;
use url::Url;

pub fn script_router(path: &Path) -> Result<Router> {
    ensure!(
        path.is_file(),
        "JavaScript entry is not a file: {}",
        path.display()
    );
    let root = path
        .parent()
        .context("JavaScript file has no parent directory")?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("JavaScript filename is not valid UTF-8")?;

    let mut entry_url = Url::parse("http://localhost/")?;
    entry_url
        .path_segments_mut()
        .map_err(|()| anyhow::anyhow!("failed to create JavaScript entry URL"))?
        .push(name);
    let entry_url = entry_url.path().replace('&', "&amp;");

    let script_attributes =
        if path.extension().and_then(|extension| extension.to_str()) == Some("mjs") {
            "type=\"module\""
        } else {
            "defer"
        };

    let html = format!(
        "<!doctype html><html><head>\
         <script {script_attributes} src=\"{entry_url}\"></script>\
         </head><body></body></html>"
    );

    Ok(Router::new()
        .route(
            "/",
            get(move || {
                let html = html.clone();
                async move { Html(html) }
            }),
        )
        .fallback_service(ServeDir::new(root))
        .layer(middleware::from_fn(disable_cache)))
}

pub fn blank_router() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { Html("<!doctype html><html><head></head><body></body></html>") }),
        )
        .fallback_service(ServeDir::new("."))
        .layer(middleware::from_fn(disable_cache))
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
