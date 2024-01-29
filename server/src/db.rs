use sqlx::MySqlPool;

static mut POOL: Option<MySqlPool> = None;

pub async fn init_db(url: &str) {
    let pool = MySqlPool::connect(url)
        .await
        .expect("Error connecting to db");

    unsafe {
        POOL = Some(pool);
    }
}

pub(crate) fn get() -> &'static MySqlPool {
    unsafe { POOL.as_ref().unwrap() }
}
