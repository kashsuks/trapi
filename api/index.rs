use dotenvy::dotenv;
use tower::Layer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use trapi::{app::build_app, config::Config, db::connect_db, state::AppState};
use vercel_runtime::{
    axum::VercelLayer,
    run, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    init_tracing();

    let config = Config::from_env();
    let db = connect_db(&config.database_url).await;
    let state = AppState::new(db, config.jwt_secret);
    let app = build_app(state);
    let service = VercelLayer::new().layer(app);

    run(service).await
}

fn init_tracing() {
    let _ = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "trapi=info,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .try_init();
}
