use rocket::serde::json::Json;

use crate::model::notification::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::service::notification::NotificationService;
use bambangshop_receiver::Result;

#[get("/subscribe/<product_type>")]
pub fn subscribe(product_type: String) -> Result<Json<SubscriberRequest>> {
    return match NotificationService::subscribe(product_type.as_str()) {
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e),
    };
}
