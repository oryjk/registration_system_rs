use crate::models::activity::{Activity, ActivityInfo, ActivityWithInfo};
use actix_web::{web, HttpResponse, Responder};
use sqlx::prelude::*;
use sqlx::MySqlPool;

pub async fn query_all_activities(pool: web::Data<MySqlPool>) -> impl Responder {
    let activities = sqlx::query_as!( Activity, r#" SELECT id, cover, end_time, holding_date, location, name, start_time, status FROM rs_activity  ORDER BY holding_date desc"# )
        .fetch_all(&**pool)
        .await;
    let activity_infos = sqlx::query_as!( ActivityInfo, r#" SELECT activity_id, color, opposing, opposing_color FROM rs_activity_info "# )
        .fetch_all(&**pool)
        .await;
    let activity_info_map: std::collections::HashMap<_, _> = match activity_infos {
        Ok(activity_infos) => {
            activity_infos
                .into_iter()
                .map(|info| (info.activity_id.clone(), info))
                .collect()
        }
        Err(_) => panic!("An error occurred")
    };
    let activity_details: Vec<ActivityWithInfo> = match activities {
        Ok(activities) => {
            activities
                .into_iter()
                .filter_map(|activity| {
                    activity_info_map.get(&activity.id)
                        .map(|info| ActivityWithInfo {
                            activity,
                            activity_info: Some(info.clone()),
                        })
                })
                .collect()
        }
        Err(_) => panic!("An error occurred")
    };


    let json_response = serde_json::to_string(&activity_details).unwrap();

    HttpResponse::InternalServerError()
        .content_type("application/json; charset=utf-8")
        .body(json_response)
}