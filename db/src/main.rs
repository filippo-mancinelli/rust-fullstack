mod db;
mod error;
mod handler;

type Result<T> = std::result
#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool).await.expect("database can be initialized");

    let pet = warp::path!("owner" / i32 / "pet");
    let pet_param = warp::path!("owner" / i32 / "pet" / i32);
    
}