use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Supplier {
  pub id: i32,
  pub name: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub deleted_at: Option<NaiveDateTime>,
}
