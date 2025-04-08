pub mod get_pool;
pub mod init_pool;

pub use get_pool::get_pool;
pub use init_pool::init_pool;
use sqlx::PgPool;
use tokio::sync::OnceCell;

pub static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();
