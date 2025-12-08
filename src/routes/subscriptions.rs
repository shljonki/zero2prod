use actix_web::{HttpResponse, web};
use chrono::{Utc};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use tracing;

#[derive(Deserialize)]
pub struct FormData {
    pub name:  String,
    pub email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber SPAN - subscribe()",
    skip(form, connection),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
// isto je što se tiče performansa ocemo li vratit impl Responder ili HttpResponse
pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&form, &connection).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish()
    }
}

#[tracing::instrument(
    name = "Saving new subscriber in database - insert_subscriber()",
    skip(form, connection)
)]
async fn insert_subscriber(form: &web::Form<FormData>, connection: &web::Data<PgPool>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
	// Using the `?` operator to return early 
	// if the function failed, returning a sqlx::Error
	// We will talk about error handling in depth later!	
    })?;
    Ok(())
}
