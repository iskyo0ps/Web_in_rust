use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize,Debug,Clone)]
pub struct Course{
  pub teacher_id:i32,
  pub id:Option<i32>,
  pub name:String,
  pub time:Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course{
  fn from(course:web::Json<Course>) ->Self{
    Course{
      teacher_id :Course.teacher_id,
      id :Course.id,
      name:Course.name,
      time:Course.time,
    }
  }
}