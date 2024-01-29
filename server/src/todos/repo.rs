use sqlx::mysql::MySqlQueryResult;

use crate::db;
use crate::models::{Todo, TodoList, TodoListCreationDto};

pub async fn get_todo(id: &str) -> Option<Todo> {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todo WHERE id = ? LIMIT 1")
        .bind(id)
        .fetch_one(db::get())
        .await
    {
        Ok(todo) => Some(todo),
        Err(why) => {
            log::error!("{:?}", why);
            None
        }
    }
}

pub async fn create_todo(
    title: &str,
    related_to: &str,
) -> sqlx::Result<MySqlQueryResult, sqlx::error::Error> {
    sqlx::query(
        "INSERT INTO todo ( id, title, checked, related_to )
         VALUES ( UUID(), ?, false, ? )",
    )
    .bind(title)
    .bind(related_to)
    .execute(db::get())
    .await
}

pub async fn get_todo_list_info(id: &str) -> Option<TodoList> {
    sqlx::query_as::<_, TodoList>("SELECT * FROM todos WHERE id = ?")
        .bind(id)
        .fetch_one(db::get())
        .await
        .ok()
}

pub async fn get_todo_lists(id: &str) -> Vec<Todo> {
    sqlx::query_as::<_, Todo>("SELECT * FROM todo WHERE related_to = ?")
        .bind(id)
        .fetch_all(db::get())
        .await
        .unwrap_or_default()
}

pub async fn create_todo_list(
    todos: &TodoListCreationDto,
    owner: &str
) -> Result<MySqlQueryResult, sqlx::error::Error> {
    sqlx::query(
        "INSERT INTO todos ( id, name, owner ) 
         VALUES ( UUID(), ?, ? )",
    )
    .bind(&todos.name)
    .bind(owner)
    .execute(db::get())
    .await
}

pub async fn todo_list_exists(id: &str) -> bool {
    get_todo_list_info(id).await.is_some()
}

pub async fn get_users_todo_lists(user_id: &str) -> Vec<TodoList> {
    sqlx::query_as::<_, TodoList>("SELECT * FROM todos WHERE owner = ?")
        .bind(user_id)
        .fetch_all(db::get())
        .await
        .unwrap_or_default()
}

pub async fn check_todo_list_owner(todos_id: &str, user_id: &str) -> bool {
    sqlx::query_as::<_, TodoList>("SELECT * FROM todos WHERE owner = ? AND id = ? LIMIT 1").bind(user_id).bind(todos_id).fetch_optional(db::get()).await.is_ok_and(|todos| todos.is_some())
}

pub async fn change_todo_checked(id: &str, checked: bool) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query("UPDATE todo SET checked = ? WHERE id = ?").bind(checked).bind(id).execute(db::get()).await
}

