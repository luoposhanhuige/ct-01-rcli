// cargo run -- http serve

// cargo add tracing // 在 main 中添加 tracing_subscriber::fmt::init();
// RUST_LOG=info cargo run -- http serve //2024-06-21T15:35:12.305915Z  INFO rcli::process::http_serve: Serving "." on port 8080

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};
// what is tower-http?
// tower-http is a library that provides a middleware for serving files from a directory. It is built on top of the tower service abstraction and is used by Axum to serve static files.

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}
// The reason for wrapping the path inside a HttpServeState struct and passing the whole HttpServeState instance through .with_state instead of passing the path.clone() directly is to provide a more flexible and extensible way to manage and share state across your application.

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr); // 使用 tracing 的 info! 宏记录日志

    // 个人认为用 config 命名更好，因为这里的 state 是一个 config，而不是一个 state
    // in this case, state 把 main 中的 opts.dir 保存在 HttpServeState 中，然后传递给 router，后续交给 file_handler 处理
    let state = HttpServeState { path: path.clone() };
    // let dir_service = ServeDir::new(path)
    //     .append_index_html_on_directories(true)
    //     .precompressed_gzip()
    //     .precompressed_br()
    //     .precompressed_deflate()
    //     .precompressed_zstd();

    let router = Router::new()
        .route("/*path", get(file_handler))
        // .nest_service("/tower", dir_service)
        .nest_service("/tower", ServeDir::new(path))
        .with_state(Arc::new(state));
    // i want to get index.html of the root path by sending a request to the root path, how to set the route?
    // .route("/", get(index_html)) // index_html is a handler function that returns the content of index.html
    // why the ".nest_service("/", dir_service)" does not work for getting index.html of the root path?
    // The reason is that the ServeDir middleware is designed to serve files from a directory, not to serve a specific file. To serve a specific file, you need to create a handler function that reads the file and returns its content.

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

// In Axum, the return type of a handler function must implement the IntoResponse trait. This trait allows Axum to convert the return value into an HTTP response that can be sent to the client.

// Can You Change the Result Type?
// Yes, you can change the result type of the handler function as long as the new type implements the IntoResponse trait. Axum provides implementations of IntoResponse for many common types, including:

// String
// &'static str
// Json<T> (for JSON responses)
// Html<String> (for HTML responses)
// Result<T, E> where T and E both implement IntoResponse

// Implementation of IntoResponse for Tuples
// Axum provides an implementation of IntoResponse for tuples where the first element is a status code and the second element is a response body.
// use axum::response::IntoResponse;
// use axum::http::StatusCode;

// impl IntoResponse for (StatusCode, String) {
//     fn into_response(self) -> axum::response::Response {
//         let (status, body) = self;
//         axum::response::Response::builder()
//             .status(status)
//             .body(axum::body::Body::from(body))
//             .unwrap()
//     }
// }

// The return type (StatusCode, String) is a tuple where:
// StatusCode represents the HTTP status code (e.g., 200 OK, 404 Not Found, 500 Internal Server Error).
// String represents the body of the HTTP response.

// In Axum, the order of the parameters in the handler function does not matter. Axum will correctly extract the parameters based on their types, regardless of the order in which they are declared in the function signature.

// std::path::Path::new(&state.path):
// The Path::new function takes a reference to a Path or a type that can be converted to a Path. PathBuf can be dereferenced to Path, so &PathBuf can be converted to &Path.
// Path::new creates a &Path from the given reference.
// join(path):
// The join method on Path takes another path (or a type that can be converted to a path) and returns a new PathBuf that represents the concatenation of the original path and the given path.
// path in this case is of type String, which can be converted to a Path.

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    //-> impl IntoResponse {}
    // format!("{:?}, {:?}", state, path) // HttpServeState { path: "." }, "Cargo.toml"
    // "." 在 process_http_serve 中传给了 HttpServeState 的 path 字段，并生成了一个 instance 即 state，
    // "Cargo.toml" 是在浏览器中输入的路径，在 process_http_serve 中 .router 中被 "/*path" 解析，然后被 file_handler 参数 Path 类型的 path 字段捕捉。
    // PathBuf to Path: PathBuf implements Deref to Path, allowing &PathBuf to be used where &Path is expected.
    // Path::new: Creates a &Path from a reference to a type that can be converted to OsStr.
    // join Method: Combines two paths and returns a new PathBuf.
    let p = std::path::Path::new(&state.path).join(path); //防止命名冲突，使用完整路径
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found!", p.display()),
        )
    } else {
        // let content = tokio::fs::read_to_string(p).await.unwrap();
        // (StatusCode::OK, content)
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

// cargo nextest run -- test_file_handler
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let path = "Cargo.toml".to_string();
        let (status, content) = file_handler(State(state), Path(path)).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.contains("[package]"));
        // assert!(content.trim().contains("[package]"));
        // what is the difference between contains and trim.contains?
        // contains: Returns true if the given pattern matches a sub-slice of the byte string.
        // trim.contains: Returns true if the given pattern matches a sub-slice of the byte string after trimming leading and trailing whitespaces.
    }
}
