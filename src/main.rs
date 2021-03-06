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

#[derive(Queryable, Debug, Identifiable, AsChangeset)]
#[table_name = "users"]
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

#[derive(Debug, Queryable)]
struct UserCourse {
    user: User,
    participation: ParticipationCourse,
}

#[derive(Debug, Queryable)]
struct ParticipationCourse {
    participation: Participation,
    course: Course,
}

use schema::users;

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    name: String,
    gender: i32,
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    dotenv().ok().unwrap();
    pretty_env_logger::init();
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

    let saras_courses_tuples = schema::users::table
        .inner_join(
            schema::participations::table.inner_join(
                schema::courses::table)
        )
        .filter(schema::users::name.eq("Sara")).load::<(User, (Participation, Course))>(con)?;
    info!("saras_courses {:?}", saras_courses_tuples);

    let saras_courses_struct = schema::users::table
        .inner_join(
            schema::participations::table.inner_join(
                schema::courses::table)
        )
        .filter(schema::users::name.eq("Sara")).load::<UserCourse>(con)?;
    info!("saras_courses {:?}", saras_courses_struct);

    let saras_courses_titles = schema::users::table
        .inner_join(
            schema::participations::table.inner_join(
                schema::courses::table)
        )
        .filter(schema::users::name.eq("Sara")).select((schema::users::name, schema::courses::title)).load::<(String, String)>(con)?;
    info!("saras_courses titles {:?}", saras_courses_titles);

    let courses = schema::courses::table.get_results::<Course>(con)?;
    info!("courses {:?}", courses);

    let new_user1 = NewUser { name: "Jonas 123".to_owned(), gender: 1 };
    let new_user1_inserted = diesel::insert_into(schema::users::table).values(&new_user1).get_result::<User>(con)?;
    info!("New user: {:?} inserted: {:?}", new_user1, new_user1_inserted);

    let user1_update = User { name: "Mr wurst".to_owned(), ..new_user1_inserted };
    info!("Updating user : {:?}", user1_update);
    let user1_updated_result = diesel::update(&user1_update).set(&user1_update).get_result::<User>(con)?;
    info!("updated user : {:?}", user1_updated_result);

    let new_user = NewUser {
        gender: 0,
        name: "Mr twister".to_owned(),
    };
    let new_user_inserted = diesel::insert_into(schema::users::table).values(&new_user).get_result::<User>(con)?;
    info!("New user {:?} inserted {:?}", &new_user, &new_user_inserted);

    let deleted_user = diesel::delete(&new_user_inserted).get_result::<User>(con);
    info!("deleted user : {:?}", deleted_user);

    let inserted_user = diesel::insert_into(schema::users::table).values(NewUser { gender: 0, name: "Esel".to_owned() }).get_result::<User>(con)?;
    info!("inserted : {:?}", inserted_user);

    return Ok(());
}
