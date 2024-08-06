use sqlx::{Pool, Sqlite};

pub type DbPool = Pool<Sqlite>;
