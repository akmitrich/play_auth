use actix_web::{web, HttpResponse};

use crate::domain::{NewSubscriber, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<sqlx::PgPool>) -> HttpResponse {
    tracing::info!("Saving new subscriber details in the database");
    match insert_subscriber(
        pool.get_ref(),
        &NewSubscriber {
            email: form.email.to_owned(),
            name: SubscriberName::parse(form.name.to_owned()),
        },
    )
    .await
    {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute insert query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn insert_subscriber(
    pool: &sqlx::PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    let new_uuid = sqlx::types::Uuid::parse_str(&uuid::Uuid::new_v4().to_string()).unwrap();
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        new_uuid,
        new_subscriber.email,
        new_subscriber.name.inner_ref(),
        chrono::Utc::now()
    )
    .execute(pool)
    .await
    .inspect_err(|e| tracing::error!("Failed to execute query: {:?}", e))?;
    Ok(())
}
