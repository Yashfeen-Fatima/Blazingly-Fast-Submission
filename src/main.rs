use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::get,
    Router,
};
use dotenv::var;
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;
use tracing::info;

use url_shortner::{
    error::AppError,
    routes::establish_app_routes,
    state::{AppContext, AppState},
    tracing::{get_trace_layer, setup_tracing},
};

async fn handle_short_url_redirect(
    State(ctx): State<AppContext>,
    Path(short_code): Path<String>,
) -> Result<Redirect, AppError> {
    let ctx = ctx.clone();
    let ctx = ctx.lock().await;

    let record = sqlx::query!(
        r#"
            SELECT long_url FROM urls WHERE short_code = $1;
        "#,
        short_code
    )
    .fetch_one(&ctx.conn_pool)
    .await
    .map_err(|_| AppError::InvalidShortCode(short_code))?;

    Ok(Redirect::to(&record.long_url))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    setup_tracing();

    let conn_url = var("DATABASE_URL")?;

    let conn_pool = PgPool::connect(&conn_url).await?;
    info!("Connection to database established successfully!");

    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    let app_router = Router::new()
        .layer(get_trace_layer())
        .route("/{short_url}", get(handle_short_url_redirect))
        .nest("/api", establish_app_routes())
        .with_state(AppState::init(conn_pool));

    info!(
        "Server rolling on port {}",
        listener.local_addr().unwrap().port()
    );

    axum::serve(listener, app_router).await?;
    Ok(())
}
