mod app;
mod domain;
mod infra;

use tokio::net::TcpListener;

use crate::infra::http::build_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = build_router();

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
