use crate::{
    db,
    models::{User, UserRegisterDto},
};

pub async fn get_user_by_id(id: &str) -> Option<User> {
    sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ? LIMIT 1")
        .bind(id)
        .fetch_one(db::get())
        .await
        .ok()
}

pub async fn get_user_by_name(name: &str) -> Option<User> {
    sqlx::query_as::<_, User>("SELECT * FROM user WHERE name = ? LIMIT 1")
        .bind(name)
        .fetch_one(db::get())
        .await
        .ok()
}

pub async fn get_user_by_email(email: &str) -> Option<User> {
    sqlx::query_as::<_, User>("SELECT * FROM user WHERE email = ? LIMIT 1")
        .bind(email)
        .fetch_one(db::get())
        .await
        .ok()
}

pub async fn user_exists_by_id(id: &str) -> bool {
    get_user_by_id(id).await.is_some()
}

pub async fn user_exists_by_name(name: &str) -> bool {
    get_user_by_name(name).await.is_some()
}

pub async fn user_exists_by_email(email: &str) -> bool {
    get_user_by_email(email).await.is_some()
}

pub async fn create_user(user: &UserRegisterDto) -> Result<String, sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    if let Err(why) = sqlx::query("INSERT INTO user ( id, name, email, password ) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .execute(db::get())
        .await
    {
        log::error!("{:#?}", why);
        return Err(why);
    }

    Ok(id)
}
