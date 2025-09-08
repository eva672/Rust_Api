use crate::config::AppConfig;
use sqlx::PgPool;
use warp::Filter;

pub mod task;

pub fn task_routes(
    config: &AppConfig,
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    task::routes(config, pool)
}
