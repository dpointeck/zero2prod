use actix_web::{HttpResponse, web};
use crate::FormData;

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse { HttpResponse::Ok().finish() }