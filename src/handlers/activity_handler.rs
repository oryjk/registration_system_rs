use actix_web::{web, HttpResponse, Responder};
use crate::application::activity_service::ActivityService;
use crate::models::activity::Activity;
use crate::models::activity_status_update::ActivityStatusUpdate;
use crate::ports::activity_repository::ActivityRepository;

pub async fn get_activity_handler<R: ActivityRepository>(
    id: web::Path<String>,
    activity_service: web::Data<ActivityService<R>>,
) -> impl Responder {
    let activity_id = id.into_inner();
    match activity_service.get_activity(&activity_id).await {
        Ok(Some(activity)) => HttpResponse::Ok().json(activity),
        Ok(None) => HttpResponse::NotFound().body("Activity not found"),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

pub async fn update_activity_status_handler<R: ActivityRepository>(
    id: web::Path<String>,
    status_update: web::Json<ActivityStatusUpdate>,
    activity_service: web::Data<ActivityService<R>>,
) -> impl Responder {
    let activity_id = id.into_inner();
    let new_status = status_update.status;

    if new_status < 0 || new_status > 2 {
        return HttpResponse::BadRequest().body("Invalid status value. Must be 0, 1, or 2.");
    }

    match activity_service.update_activity_status(&activity_id, new_status).await {
        Ok(_) => HttpResponse::Ok().body("Activity status updated successfully"),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

pub async fn create_activity_handler<R: ActivityRepository>(
    activity: web::Json<Activity>,
    activity_service: web::Data<ActivityService<R>>,
) -> impl Responder {
    let new_activity = activity.into_inner();

    match activity_service.create_activity(&new_activity).await {
        Ok(_) => HttpResponse::Created().body("Activity created successfully"),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

pub async fn list_activities_handler<R: ActivityRepository>(
    activity_service: web::Data<ActivityService<R>>,
) -> impl Responder {
    match activity_service.list_activities().await {
        Ok(activities) => HttpResponse::Ok().json(activities),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}