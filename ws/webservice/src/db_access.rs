use  super::models::*;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db
(pool: &Pgpool,teacher_id:i32) -> Result<Vec<Course>,MyError> {
  let rows = sqlx::query!(
    r#"SELECT id, teacher_id,name,time
    FROM course
    WHERE teacher_id = $1"#.
    teacher_id
  )
  .fetch_all(pool)
  .await?;
  // .unwrap();

  let courses = rows.iter()
      .map(|r| Course{
        id:Some(r.id),
        teacher_id:r.teacher_id,
        name:r.name.clone(),
        time:Some(NaiveDateTime::from(r.time.unwrap())),
      })
      .collect();
  match courses.len() {
    0 => Err(MyError::NotFound("Courses not found for teacher".into())),
    _ => Ok(courses)
  }
}

pub async fn get_courses_details_db(pool: &Pgpool,teacher_id:i32,course_id:i32)->Course{
  let rows = sqlx::query!(
    r#"SELECT id, teacher_id,name,time
    FROM course
    WHERE teacher_id = $1 and id = $2"#.
    teacher_id,
    course_id
  )
  .fetch_all(pool)
  .await
  .unwrap();
  Course{
    id:Some(row.id),
    teacher_id:row.teacher_id,
    name :row.name.clone(),
    time:Some(NaiveDateTime::from(row.time.unwrap())),
  }
}

pub async fn post_new_course_db(pool: &Pgpool,new_course:Course)->Course{
  let rows = sqlx::query!(
    r#"INSERT INTO course(id, teacher_id,name)
    VALUES($1,$2,$3)
    RETURNING id,teacher_id,name,time"#,
    new_course.id, 
    new_course.teacher_id,
    new_course.name
  )
  .fetch_one(pool)
  .await
  .unwrap();
  Course{
    id:Some(row.id),
    teacher_id:row.teacher_id,
    name :row.name.clone(),
    time:Some(NaiveDateTime::from(row.time.unwrap())),
  }

}