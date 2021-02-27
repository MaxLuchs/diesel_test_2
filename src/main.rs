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
    title: String,
}

#[derive(Debug, Queryable)]
struct Participation {
    user_id: i32,
    course_id: i32,
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
    let participations = schema::participations::table.filter(schema::participations::user_id.eq(user1.id)).load::<Participation>(con);
    info!("user participations {:?}", participations);

    let clarks_courses = schema::users::table.filter(schema::users::name.eq("Clark")
    ).inner_join(schema::participations::table).load::<(User, Participation)>(con)?;
    info!("clarks_courses {:?}", clarks_courses);

    let luthers_courses = schema::users::table.filter(schema::users::name.eq("Luther")
    ).inner_join(schema::participations::table.inner_join(schema::courses::table)).load::<(User, (Participation, Course))>(con)?;
    info!("luthers_courses {:?}", luthers_courses);

    let saras_courses = schema::users::table.filter(schema::users::name.eq("Sara")
    ).inner_join(schema::participations::table.inner_join(schema::courses::table)).load::<(User, (Participation, Course))>(con)?;
    info!("saras_courses {:?}", saras_courses);

    let courses = schema::courses::table.get_results::<Course>(con)?;
    info!("courses {:?}", courses);
    return Ok(());
}
