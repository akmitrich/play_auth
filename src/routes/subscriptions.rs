use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<sqlx::PgPool>,
) -> HttpResponse {
    let request_id = uuid::Uuid::new_v4();
    let request_span = tracing::info_span!("Adding a new subscriber", %request_id, subscriber_email=%form.email, subscriber_name=%form.name);
    let _request_span_guard = request_span.enter();
    tracing::info!("Saving new subscriber details in the database");
    let new_uuid = sqlx::types::Uuid::parse_str(&uuid::Uuid::new_v4().to_string()).unwrap();
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        new_uuid,
        form.email,
        form.name,
        chrono::Utc::now()
    )
    .execute(connection.get_ref())
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
