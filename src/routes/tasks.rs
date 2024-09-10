use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

pub async fn get_tasks(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<QuriedTasks>>), (StatusCode, Json<ErrorBody>)> {
    let rows = sqlx::query_as!(QuriedTasks, "SELECT id,title,description FROM tasks_table")
        .fetch_all(&db_pool)
        .await
        .map_err(|_| {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody {
                    message: "Error while getting tasks".to_string(),
                }),
            );
        })?;

    return Ok((StatusCode::OK, Json(rows)));
}

pub async fn get_task_by_id(
    State(db_pool): State<PgPool>,
    Path(task_id): Path<i32>,
) -> Result<(StatusCode, Json<QuriedTasks>), (StatusCode, Json<ErrorBody>)> {
    let row = sqlx::query_as!(
        QuriedTasks,
        "SELECT id,title,description FROM tasks_table where id=$1",
        task_id
    )
    .fetch_one(&db_pool)
    .await
    .map_err(|_| {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody {
                message: "Error while getting task".to_string(),
            }),
        );
    })?;
    return Ok((StatusCode::OK, Json(row)));
}

pub async fn delete_task_by_id(
    State(db_pool): State<PgPool>,
    Path(task_id): Path<i32>,
) -> Result<(StatusCode, Json<DeleteJsonRes>), (StatusCode, Json<DeleteJsonRes>)> {
    sqlx::query!("delete from tasks_table where id = $1", task_id)
        .execute(&db_pool)
        .await
        .map_err(|_| {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DeleteJsonRes {
                    message: "Internal Server Errro".to_string(),
                }),
            );
        })?;

    Ok((
        StatusCode::OK,
        Json(DeleteJsonRes {
            message: "Task Deleted SuccessFully".to_string(),
        }),
    ))
}
pub async fn create_task(
    State(db_pool): State<PgPool>,
    axum::extract::Json(payload): axum::extract::Json<ReqBody>,
) -> Result<(StatusCode, Json<QuriedTasks>), (StatusCode, Json<ErrorBody>)> {
    let row =  sqlx::query_as!(
        QuriedTasks,
        "INSERT INTO tasks_table (title, description, updated_at) VALUES ($1, $2, NOW()) returning id,title,description ",
        payload.title,
        payload.description
    )
    .fetch_one(&db_pool)
    .await
    .map_err(|_| {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody {
                message: "Error while creating task".to_string(),
            }),
        );
    })?;

    Ok((StatusCode::OK, Json(row)))
}

pub async fn update_task(
    State(db_pool): State<PgPool>,
    Path(task_id): Path<i32>,
    Json(payload): Json<ReqBody>,
) -> Result<(StatusCode, Json<QuriedTasks>), (StatusCode, Json<ErrorBody>)> {
    let row = sqlx::query_as!(
        QuriedTasks,
        "UPDATE tasks_table SET title = $1, description = $2 WHERE id = $3 returning id,title,description;",
        payload.title,
        payload.description,
        task_id
    )
    .fetch_one(&db_pool)
    .await
    .map_err(|_| {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody {
                message: "Error while creating task".to_string(),
            }),
        );
    })?;

    Ok((StatusCode::OK, Json(row)))
}

#[derive(serde::Serialize)]
pub struct QuriedTasks {
    id: i32,
    title: String,
    description: String,
}

#[derive(serde::Deserialize)]
pub struct ReqBody {
    pub title: String,
    pub description: String,
}

#[derive(serde::Serialize)]
pub struct ErrorBody {
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct DeleteJsonRes {
    pub message: String,
}
