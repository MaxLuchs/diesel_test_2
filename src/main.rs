use dotenv::dotenv;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;

#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate eyre;

use diesel::prelude::*;
use std::error::Error;

mod schema;

#[derive(Queryable, Debug)]
struct User {
    id: i32,
    name: String,
    gender: i32,
}

#[derive(Debug, Queryable)]
struct Course {
    id: i32,
    title: String
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    dotenv().ok().unwrap();
    env_logger::init();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let mngr = ConnectionManager::new(db_url);
    let pool: Pool<ConnectionManager<diesel::pg::PgConnection>> = r2d2::Pool::new(mngr).expect("Conn pool error");
    let con = &pool.get()?;
    let user1 = schema::users::table.find(1).first::<User>(con).optional()?.ok_or(eyre!("user error"))?;
    info!("user {:?}", user1);

    let courses = schema::courses::table.get_results::<Course>(con)?;
    info!("courses {:?}", courses);
    return Ok(());
}
