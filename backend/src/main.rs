use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{config::Config, NoTls};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use uuid::Uuid;

#[tokio::main]
async fn main() {
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    /* let manager = PostgresConnectionManager::new(
        Config {
            host: "127.0.0.1".to_string(),
            // port: 5432,
            user: "cleanhotel".to_string(),
            // password: "postgres".to_string(),
            database: "cleanhotel".to_string(),
        },
        NoTls,
    ) */
    let manager =
        PostgresConnectionManager::new_from_stringlike("host=localhost user=cleanhotel", NoTls)
            .unwrap();

    let pool = Pool::builder().build(manager).await.unwrap();

    let app = Router::new()
        .route(
            "/",
            get(using_connection_pool_extractor).post(using_connection_pool_extractor),
        )
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

async fn using_connection_pool_extractor(
    State(pool): State<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let rows = conn
        .query("select * from hotels", &[])
        .await
        .map_err(internal_error)?;

    Ok(rows
        .into_iter()
        .map(|row| match row.try_get::<_, String>("name") {
            Ok(name) => format!("{}\n", name),
            Err(e) => {
                tracing::error!(error = %e, "failed to get name");
                "unknown".into()
            }
        })
        .collect::<String>())
}

// async fn using_connection_extractor(
//     DatabaseConnection(conn): DatabaseConnection,
// ) -> Result<String, (StatusCode, String)> {
//     let row = conn
//         .query_one("select 1 + 1", &[])
//         .await
//         .map_err(internal_error)?;
//
//     let two: i32 = row.try_get(0).map_err(internal_error)?;
//     Ok(two.to_string())
// }
//
// struct DatabaseConnection(PooledConnection<'static, PostgresConnectionManager<NoTls>>);
//
// #[async_trait]
// impl<S> FromRequestParts<S> for DatabaseConnection
// where
//     ConnectionPool: FromRef<S>,
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, String);
//
//     async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let pool = ConnectionPool::from_ref(state);
//
//         let conn = pool.get_owned().await.map_err(internal_error)?;
//
//         Ok(Self(conn))
//     }
// }
//
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
