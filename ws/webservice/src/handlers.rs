use super::state::AppState;
use actix_web::{web, HttpResponse};
use super::db_access;

pub async fn health_check_handler(
    app_state:web::Data<AppState>
  ) -> HttpResponse{
    let health_check_response = &app_state::health_check_response;

    let mut visit_count = app_state::visit_count.lock().unwrap();
    let response =
      format!("{} {} times", health_check_response,visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}
use super::models::Course;
// use chrono::Utc;

pub async fn new_course(
  new_course:web::Json<Course>
  app_state:web::Data<AppState>,
) -> HttpResponse{
  let course = post_new_course_db(&app_state.db, new_course.into()).await;
  HttpResponse::Ok().json(course)
}


pub async fn get_courses_for_teacher(
  app_state:web::Data<AppState>,
  params:web::Path<(usize,)>
) -> HttpResponse{
  let teacher_id = i32::try_from(params.0).unwrap();
  let courses = get_courses_for_teacher_db(&app_state.db,teacher_id).await;
  HttpResponse::Ok().json(courses)
}

pub async fn get_course_detail(
  app_state:web::Data<AppState>,
  params:web::Path<(usize,usize)>
) -> HttpResponse{
  let teacher_id = i32::try_from(params.0).unwrap();
  let course_id = i32::try_from(params.1).unwrap();
  let course = get_courses_details_db(&app_state.db,teacher_id,course).await;
  HttpResponse::Ok().json(course)
}

// pub async fn get_courses_for_teacher(
//   app_state:web::Data<AppState>,
//   params:web::Path<(usize)>,
// ) -> HttpResponse{
//   let teacher_id = params.0;

//   let teacher_courses = app_state
//     .courses
//     .lock()
//     .unwrap()
//     .clone()
//     .into_iter()
//     .filter(|course| course.teacher_id == teacher_id)
//     .collect::<Vec<Course>>();
  
//   if filtered_courses.len() > 0{
//     HttpResponse::Ok().json(filtered_courses)   
//   }else{
//     HttpResponse::Ok().json("No courses found for teacher".to_string())
//   }
// }

#[cfg(test)]
mod tests{
  use super::*;
  use actix_web::http::StatusCode;
  use std::sync::Mutex;

  use chrono::NaiveDateTime;
  use dotenv::dotenv;
  use sqlx::postgres::PgPoolOptions;
  use std::env;
  // #[actix_rt::test]
  // async fn post_course_test(){
  //   let course = web::Json(Course{
  //     teacher_id :1,
  //     name:"Test course".into(),
  //     id: None,
  //     time:None,
  //   });
  //   let app_state: web::Data<AppState> = web::Data::new(AppState{
  //     health_check_response:"",to_string(),
  //     visit_count:Mutex::new(0),
  //     courses:Mutex::new(vec![]),
  //   });
  //   let resp = new_course(course,app_state).await;
  //   assert_eq!(resp.status(),StatusCode::Ok)
  // }

  #[actix_rt::test]
  async fn post_course_test(){
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let app_state:web::Data<AppState> = web::Data::new(AppState{
      health_check_response:"",to_string(),
      visit_count:Mutex::new(0),
      //courses:Mutex::new(vec![]),
      db:db_pool,
    });
    let cousrse = web::Json(Course{
      teacher_id:1,
      name:"Test course".to_string(),
      id:Some(3),
      time:None,
    })
    let resp = new_course(course,app_state).await;
    assert_eq!(resp.status(),StatusCode::Ok);
  }

  #[actix_rt::test]
  async fn get_all_courses_success(){
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let app_state:web::Data<AppState> = web::Data::new(AppState{
      health_check_response:"",to_string(),
      visit_count:Mutex::new(0),
      //courses:Mutex::new(vec![]),
      db:db_pool,
    });
    let teacher_id:web::Path<(usize,)> = web::Path::From((1,));
    let resp = get_courses_for_teacher(app_state,teacher_id).await;
    assert_eq!(resp.status(),StatusCode::Ok);
  }

  #[actix_rt::test]
  async fn get_one_course_success(){
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let app_state:web::Data<AppState> = web::Data::new(AppState{
      health_check_response:"",to_string(),
      visit_count:Mutex::new(0),
      //courses:Mutex::new(vec![]),
      db:db_pool,
    });
    let params :web::Path<(usize,usize)> = web::Path::from((1,1));
    let resp = get_course_detail(app_state,params).await;
    assert_eq!(resp.status(),StatusCode::Ok);
  }
}