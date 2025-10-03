use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let state = Arc::new(HttpServeState { path: path.clone() });
    let router = Router::new()
        .route("/", get(root_handler))
        .route("/{*path}", get(file_handler))
        .nest_service("/tower", ServeDir::new(path))
        .with_state(Arc::clone(&state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!(
        "Serving {} at http://localhost:{} (listening on {})",
        state.path.display(),
        port,
        addr
    );
    info!("Serving {:?} on {}", state.path, addr);
    axum::serve(listener, router).await?;
    Ok(())
}

async fn root_handler(State(state): State<Arc<HttpServeState>>) -> Response {
    serve_path(state, "").await
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> Response {
    serve_path(state, &path).await
}

async fn serve_path(state: Arc<HttpServeState>, raw_path: &str) -> Response {
    let request_path = raw_path.trim_start_matches('/');
    if std::path::Path::new(request_path)
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return (
            StatusCode::FORBIDDEN,
            "Parent directory traversal is not allowed".to_string(),
        )
            .into_response();
    }

    let target_path = if request_path.is_empty() {
        state.path.clone()
    } else {
        state.path.join(request_path)
    };
    info!("Resolving {:?} to {:?}", request_path, target_path);

    if !target_path.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", target_path.display()),
        )
            .into_response()
    } else if target_path.is_dir() {
        match render_directory_listing(&target_path, request_path).await {
            Ok(html) => (StatusCode::OK, Html(html)).into_response(),
            Err(e) => {
                warn!("Error rendering directory: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    } else {
        match tokio::fs::read(&target_path).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content).into_response()
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}

async fn render_directory_listing(
    dir: &std::path::Path,
    request_path: &str,
) -> anyhow::Result<String> {
    let mut entries = tokio::fs::read_dir(dir).await?;
    let mut items = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let file_type = entry.file_type().await?;
        let mut name = entry.file_name().to_string_lossy().into_owned();
        if file_type.is_dir() {
            name.push('/');
        }
        items.push(name);
    }

    items.sort();

    let mut html = String::with_capacity(512);
    html.push_str("<html><body><h1>Index of ");
    html.push_str(&format!("/{}", request_path));
    html.push_str("</h1><ul>");

    if !request_path.is_empty() {
        let parent = std::path::Path::new(request_path);
        if let Some(parent_path) = parent.parent() {
            let href = parent_path.to_str().unwrap_or("");
            let mut link = href.trim_end_matches('/').to_string();
            if !link.is_empty() {
                link.push('/');
            }
            html.push_str(&format!("<li><a href=\"/{link}\">../</a></li>"));
        } else {
            html.push_str("<li><a href=\"/\">../</a></li>");
        }
    }

    let prefix = request_path.trim_end_matches('/');
    for name in items {
        let mut href = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", prefix, name)
        };
        let display = &name;
        if !href.starts_with('/') {
            href.insert(0, '/');
        }
        html.push_str(&format!("<li><a href=\"{}\">{}</a></li>", href, display));
    }

    html.push_str("</ul></body></html>");
    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let response = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_root_handler_serves_directory() {
        let tmp_dir = tempdir().expect("create temp dir");
        let file_path = tmp_dir.path().join("hello.txt");
        tokio::fs::write(&file_path, "hello")
            .await
            .expect("write file");
        let state = Arc::new(HttpServeState {
            path: tmp_dir.path().to_path_buf(),
        });

        let response = root_handler(State(state)).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
