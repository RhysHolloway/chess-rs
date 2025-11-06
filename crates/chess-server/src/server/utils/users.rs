use sqlx::Row;

const USER_AND_USER_PROFILE_QUERY: &str = "
SELECT 
  u.id AS u_id, 
  u.email AS u_email, 
  u.password AS u_password, 
  u.first_name AS u_first_name, 
  u.last_name AS u_last_name, 
  u.is_active AS u_is_active, 
  u.is_staff AS u_is_staff, 
  u.is_superuser AS u_is_superuser, 
  u.thumbnail AS u_thumbnail, 
  u.date_joined AS u_date_joined, 
  p.id AS p_id, 
  p.user_id AS p_user_id, 
  p.phone_number AS p_phone_number, 
  p.birth_date AS p_birth_date, 
  p.github_link AS p_github_link 
FROM 
  users u 
  LEFT JOIN user_profile p ON p.user_id = u.id 
WHERE 
  u.is_active = true AND ";

#[derive(serde::Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
    pub username: String,
    pub is_superuser: bool,
}

#[tracing::instrument(name = "Getting an active user from the DB.", skip(pool))]
pub async fn get_active_user_from_db(
    pool: Option<&sqlx::postgres::PgPool>,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
    id: Option<uuid::Uuid>,
    email: Option<&String>,
) -> Result<User, sqlx::Error> {
    let mut query_builder =
        sqlx::query_builder::QueryBuilder::new(USER_AND_USER_PROFILE_QUERY);

    if let Some(id) = id {
        query_builder.push(" u.id=");
        query_builder.push_bind(id);
    }

    if let Some(e) = email {
        query_builder.push(" u.email=");
        query_builder.push_bind(e);
    }

    let sqlx_query = query_builder
        .build()
        .map(|row: sqlx::postgres::PgRow| User {
            id: row.get("u_id"),
            email: row.get("u_email"),
            username: row.get("u_username"),
            password: row.get("u_password"),
            is_superuser: row.get("u_is_superuser"),
        });

    let fetched_query = {
        if pool.is_some() {
            let p = pool.unwrap();
            sqlx_query.fetch_one(p).await
        } else {
            let t = transaction.unwrap().as_mut();
            sqlx_query.fetch_one(t).await
        }
    };
    match fetched_query {
        Ok(user) => Ok(user),
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "User not found in DB: {:#?}", e);
            Err(e)
        }
    }
}
