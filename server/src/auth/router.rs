use axum::{http::StatusCode, routing::post, Json, Router};
use pwhash::bcrypt;

use crate::{
    jwt::{encode, Claims},
    models::{UserRegisterDto, UserLoginDto},
    users, AppState,
};

pub fn router() -> Router<AppState> {
    let router = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    router
}

async fn login(
    Json(body): Json<UserLoginDto>,
) -> Result<Json<String>, (StatusCode, &'static str)> {
    let user = match users::repo::get_user_by_name(&body.name).await {
        Some(user) => user,
        None => return Err((StatusCode::BAD_REQUEST, "User with this name does not exist")),
    };

    if !bcrypt::verify(&body.password, &user.password) {
        return Err((StatusCode::BAD_REQUEST, "Wrong password"));
    }

    let claims = Claims::new(user.name, user.id);

    let token = encode(&claims);

    if token.is_err() {
        return Err((StatusCode::IM_A_TEAPOT, "Something went wrong, sry"));
    }

    Ok(Json(token.unwrap()))
}

async fn register(
    Json(mut user): Json<UserRegisterDto>,
) -> Result<(StatusCode, String), (StatusCode, &'static str)> {
    if users::repo::user_exists_by_name(&user.name).await {
        return Err((
            StatusCode::IM_A_TEAPOT,
            "User with this name already exists",
        ));
    }
    if users::repo::user_exists_by_email(&user.email).await {
        return Err((
            StatusCode::IM_A_TEAPOT,
            "User with this email already exists",
        ));
    }

    user.password = match bcrypt::hash(&user.password) {
        Ok(pass) => pass,
        Err(why) => {
            log::error!("Error hashing a password:\n{:#?}", why);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong, try again later"));
        }
    };

    let user_id = users::repo::create_user(&user).await.ok();
    match user_id {
        Some(id) => {
            let claims = Claims::new(user.name, id);

            let token = encode(&claims);

            if token.is_err() {
                return Err((StatusCode::IM_A_TEAPOT, "Something went wrong, sry"));
            }

            Ok((StatusCode::CREATED, token.unwrap()))
        }
        None => {
            return Err((StatusCode::IM_A_TEAPOT, "Something went wrong, sry"));
        }
    }
}
