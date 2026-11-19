use std::path::Path;

use anyhow::{Context, Result};
use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, header},
    middleware::{self, Next},
    response::{Html, Response},
    routing::{MethodRouter, any_service, get},
    serve::ListenerExt,
};
use tokio::{net::TcpListener, task::JoinHandle};
use tower_http::services::{ServeDir, ServeFile};
use url::Url;

use crate::InputType;

pub struct LocalServer {
    pub page_url: String,
    pub entry_url: String,
    task: JoinHandle<std::io::Result<()>>,
}

impl LocalServer {
    pub async fn serve_file(path: &Path, input_type: InputType, port: u16) -> Result<Self> {
        let dir = path
            .parent()
            .context("JavaScript file has no parent directory")?;
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .context("JavaScript filename is not valid UTF-8")?;
        let file = ServeFile::new_with_mime(path, &"text/javascript".parse()?);
        Self::start(dir, name, any_service(file), input_type, port).await
    }

    pub async fn serve_source(source: String, input_type: InputType, port: u16) -> Result<Self> {
        let script = get(([(header::CONTENT_TYPE, "text/javascript")], source));
        Self::start(
            Path::new("."),
            "__limo_input__.js",
            script,
            input_type,
            port,
        )
        .await
    }

    async fn start(
        dir: &Path,
        name: &str,
        script: MethodRouter,
        input_type: InputType,
        port: u16,
    ) -> Result<Self> {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port))
            .await
            .context("failed to start local server")?;
        let page_url = format!("http://{}/", listener.local_addr()?);
        let mut entry_url = Url::parse(&page_url)?;
        entry_url
            .path_segments_mut()
            .expect("HTTP URLs have path segments")
            .push(name);
        let page = Html(entry_html(entry_url.path(), input_type));
        let app = Router::new()
            .without_v07_checks()
            .route("/", get(page))
            .route(entry_url.path(), script)
            .fallback_service(ServeDir::new(dir))
            .layer(middleware::from_fn(disable_cache));

        let listener = listener.tap_io(|stream| {
            // Send small script responses without waiting for TCP acknowledgements.
            let _ = stream.set_nodelay(true);
        });
        let task = tokio::spawn(async move { axum::serve(listener, app).await });
        Ok(Self {
            page_url,
            entry_url: entry_url.into(),
            task,
        })
    }
}

impl Drop for LocalServer {
    fn drop(&mut self) {
        self.task.abort();
    }
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
