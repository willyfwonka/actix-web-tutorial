use bcrypt::DEFAULT_COST;
use diesel::result::Error;
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::core::exception::Exception;
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::Pool;

#[derive(Identifiable, Associations, Queryable, Debug, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub username: String,
  #[serde(skip_serializing)]
  pub password: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
  pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct CreateUser {
  pub email: String,
  pub username: String,
  pub password: String,
}

impl User {
  pub fn find_one(user_id: i32, pool: &Pool) -> Result<User, Exception> {
    let conn = pool.get().unwrap();
    let user = users.find(user_id).first(&conn);

    match user {
      Ok(user) => Ok(user),
      Err(why) => match why {
        Error::NotFound => Err(Exception::NotFound),
        _ => Err(Exception::InternalServerError),
      },
    }
  }

  pub fn find_one_by_email(query: String, pool: &Pool) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let user = users.filter(email.eq(&query)).first(&conn);

    user
  }

  pub fn find(pool: &Pool) -> Result<(Vec<User>, i64), diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items: Vec<User> = users.load(&conn).unwrap();
    let count: i64 = users.count().first(&conn).unwrap();
    Ok((items, count))
  }

  pub fn save(
    mut create_user_input: CreateUser,
    pool: &Pool,
  ) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();

    create_user_input.password = bcrypt::hash(create_user_input.password, DEFAULT_COST).unwrap();

    let user = CreateUser {
      ..create_user_input
    };
    let saved_user = insert_into(users).values(&user).get_result(&conn)?;
    Ok(saved_user)
  }
}
