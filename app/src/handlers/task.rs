use crate::config::AppConfig;
use crate::models::task::Task;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use warp::Filter;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub fn routes(
    config: &AppConfig,
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let _config = config.clone();

    // GET /api/tasks - List all tasks
    let get_tasks = {
        let pool = pool.clone();
        warp::path("api")
            .and(warp::path("tasks"))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(move || list_tasks(pool.clone()))
    };

    // POST /api/tasks - Create a new task
    let create_tasks = {
        let pool = pool.clone();
        warp::path("api")
            .and(warp::path("tasks"))
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |req: CreateTaskRequest| create_task(req, pool.clone()))
    };

    // Health check endpoint
    let health = warp::path("health")
        .and(warp::path::end())
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    get_tasks.or(create_tasks).or(health)
}

async fn list_tasks(pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("📋 Fetching all tasks from database...");

    let tasks = sqlx::query_as::<_, Task>(
        "SELECT id, user_id, title, description, completed, created_at, updated_at 
         FROM tasks 
         ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        log::error!("❌ Failed to fetch tasks from database: {}", e);
        warp::reject::custom(crate::error::AppError::DatabaseError(e.to_string()))
    })?;

    log::info!(
        "✅ Successfully fetched {} tasks from database",
        tasks.len()
    );

    let task_responses: Vec<TaskResponse> = tasks
        .into_iter()
        .map(|task| TaskResponse {
            id: task.id,
            user_id: task.user_id,
            title: task.title,
            description: task.description,
            completed: task.completed,
            created_at: task.created_at.to_rfc3339(),
            updated_at: task.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(warp::reply::json(&task_responses))
}

async fn create_task(
    req: CreateTaskRequest,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("📝 Creating new task: {}", req.title);

    // For now, use a dummy user_id. In a real app, this would come from the authenticated user
    let dummy_user_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();

    let inserted_task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (id, user_id, title, description, completed) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING id, user_id, title, description, completed, created_at, updated_at",
    )
    .bind(&task_id)
    .bind(&dummy_user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(false)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        log::error!("❌ Failed to create task in database: {}", e);
        warp::reject::custom(crate::error::AppError::DatabaseError(e.to_string()))
    })?;

    log::info!(
        "✅ Successfully created task {} in database",
        inserted_task.id
    );

    let task_response = TaskResponse {
        id: inserted_task.id,
        user_id: inserted_task.user_id,
        title: inserted_task.title,
        description: inserted_task.description,
        completed: inserted_task.completed,
        created_at: inserted_task.created_at.to_rfc3339(),
        updated_at: inserted_task.updated_at.to_rfc3339(),
    };

    Ok(warp::reply::json(&task_response))
}
