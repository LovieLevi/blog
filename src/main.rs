use axum::{
    routing,
    http::StatusCode,
    extract,
    Router,
    response::{Html, IntoResponse, Response},
};
use askama::Template;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", routing::get(root))
        .route("/posts/{id}", routing::get(post_template));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn post_template(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let template = PostTemplate { name, post: "Hello, World!".to_string(), id: "1".to_string() };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    name: String,
    post: String,
    id: String,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
