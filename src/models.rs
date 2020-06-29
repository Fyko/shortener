use diesel::{Queryable};
use super::schema::urls;

#[derive(Queryable, Insertable)]
#[table_name="urls"]
pub struct ShortenedURL {
	pub id: i32,
	pub short: String,
	pub long: String,
}