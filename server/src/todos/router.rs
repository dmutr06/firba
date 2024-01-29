use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};

use crate::{
    jwt::Claims,
    models::{Todo, TodoCreationDto, TodoList, TodoListCreationDto, TodoUpdateDto},
    users, AppState,
};

use super::repo;

pub fn router() -> Router<AppState> {
    let router = Router::new()
        .route("/", get(get_todo_lists))
        .route("/:id", get(get_todo_list_info))
        .route("/:id/all", get(get_todo_list))
        .route("/create", post(create_todo_list))
        .route("/:id/add", post(add_todo))
        .route("/update_todo/:id", put(toggle_todo));

    router
}

async fn get_todo_list_info(claims: Claims, Path(id): Path<String>) -> Result<Json<TodoList>, (StatusCode, &'static str)> {
    if !repo::check_todo_list_owner(&id, &claims.user_id).await {
        return Err((StatusCode::NOT_FOUND, "You are not owner of this todo list"));
    }

    Ok(Json(repo::get_todo_list_info(&id).await.unwrap()))
}

async fn get_todo_lists(claims: Claims) -> Result<Json<Vec<TodoList>>, Json<String>> {
    if !users::repo::user_exists_by_id(&claims.user_id).await {
        return Err(Json("User does not exist".to_owned()));
    }

    Ok(Json(repo::get_users_todo_lists(&claims.user_id).await))
}

async fn get_todo_list(claims: Claims, Path(todos_id): Path<String>) -> Result<Json<Vec<Todo>>, (StatusCode, &'static str)> {
    if !repo::check_todo_list_owner(&todos_id, &claims.user_id).await {
        return Err((StatusCode::IM_A_TEAPOT, "You are not owner of this todo list"));
    }

    Ok(Json(repo::get_todo_lists(&todos_id).await))
}

async fn create_todo_list(claims: Claims, Json(body): Json<TodoListCreationDto>) -> (StatusCode, &'static str) {
    if !users::repo::user_exists_by_id(&claims.user_id).await {
        return (StatusCode::NOT_FOUND, "Please, login or register");
    }

    let _ = repo::create_todo_list(&body, &claims.user_id).await;

    (StatusCode::CREATED, "Created a new todo list")
}

async fn add_todo(
    claims: Claims,
    Path(id): Path<String>,
    Json(body): Json<TodoCreationDto>,
) -> (StatusCode, &'static str) {
    if !users::repo::user_exists_by_id(&claims.user_id).await {
        return (StatusCode::NOT_FOUND, "Please, login or register");
    }
        
    if !repo::todo_list_exists(&id).await {
        return (StatusCode::IM_A_TEAPOT, "Todo list with this id does not exist");
    }

    if let Err(why) = repo::create_todo(&body.title, &id).await {
        log::error!("Error creating a todo:\n{:#?}", why);
        return (StatusCode::IM_A_TEAPOT, "Could not create a todo, try again");
    }

    (StatusCode::CREATED, "Created a new todo")
}

async fn toggle_todo(claims: Claims, Path(todo_id): Path<String>, Json(body): Json<TodoUpdateDto>) -> (StatusCode, &'static str) {
    let todo = match repo::get_todo(&todo_id).await {
        Some(todo) => todo,
        None => return (StatusCode::BAD_REQUEST, "Todo with this id does not exist"),
    };

    if !repo::check_todo_list_owner(&todo.related_to, &claims.user_id).await {
        return (StatusCode::IM_A_TEAPOT, "You are not owner of this todo");
    }

    match repo::change_todo_checked(&todo_id, body.checked).await {
        Ok(_) => (StatusCode::OK, "Todo has been updated"),
        Err(why) => {
            log::error!("Error updating a todo:\n{:#?}", why);
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong, try again later")
        }
    }
}

