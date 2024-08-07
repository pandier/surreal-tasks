use eyre::{Context, ContextCompat};
use rocket::{serde::json::Json, State};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::{model::task::PrivateTask, Claims, CreateTask, RouteResult, Task};

#[get("/")]
pub async fn list(
    claims: Claims,
    database: &State<Surreal<Client>>,
) -> RouteResult<Json<Vec<PrivateTask>>> {
    Ok(Json(
        database
            .query("SELECT * FROM task WHERE user = $user;")
            .bind(("user", Thing::from(("user", claims.sub.as_ref()))))
            .await
            .wrap_err("Failed to select tasks from database")?
            .take::<Vec<Task>>(0)?
            .iter()
            .map(|task: &Task| task.clone().into())
            .collect(),
    ))
}

#[get("/<id>")]
pub async fn get(
    id: &str,
    claims: Claims,
    database: &State<Surreal<Client>>,
) -> RouteResult<Option<Json<PrivateTask>>> {
    Ok(database
        .query("SELECT * FROM $task WHERE user = $user;")
        .bind(("task", Thing::from(("task", id))))
        .bind(("user", Thing::from(("user", claims.sub.as_ref()))))
        .await
        .wrap_err("Failed to select task from database")?
        .take::<Option<Task>>(0)?
        .map(|task: Task| Json(task.into())))
}

#[post("/", data = "<create_task>")]
pub async fn create(
    claims: Claims,
    create_task: Json<CreateTask>,
    database: &State<Surreal<Client>>,
) -> RouteResult<Json<PrivateTask>> {
    let task = database
        .query(
            "CREATE task SET
                name = $name,
                user = $user;",
        )
        .bind(("name", &create_task.name))
        .bind(("user", Thing::from(("user", claims.sub.as_ref()))))
        .await?
        .take::<Option<Task>>(0)
        .wrap_err("Failed to create task")?
        .wrap_err("Received None after task creation")?;

    Ok(Json(task.into()))
}

#[delete("/<id>")]
pub async fn delete(
    id: &str,
    claims: Claims,
    database: &State<Surreal<Client>>,
) -> RouteResult<Option<()>> {
    Ok(database
        .query("DELETE $task WHERE user = $user RETURN BEFORE;")
        .bind(("task", Thing::from(("task", id))))
        .bind(("user", Thing::from(("user", claims.sub.as_ref()))))
        .await
        .wrap_err("Failed to delete task from database")?
        .take::<Option<Task>>(0)?
        .map(|_| ()))
}
