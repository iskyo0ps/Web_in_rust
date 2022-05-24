
use super::state::AppState;
use actix_web::{web, HttpResponse};

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


pub async fn get_courses_for_teacher(
  app_state:web::Data<AppState>,
  params:web::Path<(usize)>,
) -> HttpResponse{
  let teacher_id = params.0;

  let teacher_courses = app_state
    .courses
    .lock()
    .unwrap()
    .clone()
    .into_iter()
    .filter(|course| course.teacher_id == teacher_id)
    .collect::<Vec<Course>>();
  
  if filtered_courses.len() > 0{
    HttpResponse::Ok().json(filtered_courses)   
  }else{
    HttpResponse::Ok().json("No courses found for teacher".to_string())
  }
}

#[cfg(test)]
mod tests{
  use super::*;
  use actix_web::http::StatusCode;
  use std::sync::Mutex;

  #[actix_rt::test]
  async fn post_course_test(){
    let course = web::Json(Course{
      teacher_id :1,
      name:"Test course".into(),
      id: None,
      time:None,
    });
    let app_state: web::Data<AppState> = web::Data::new(AppState{
      health_check_response:"",to_string(),
      visit_count:Mutex::new(0),
      courses:Mutex::new(vec![]),
    });
    let resp = new_course(course,app_state).await;
    assert_eq!(resp.status(0,StatusCode::Ok))
  }
}