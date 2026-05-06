use rusqlite::Connection;
use crate::models::CookieEntry;

pub fn extract_cookies(conn: &Connection) -> Result<Vec<CookieEntry>, String> {
  let mut stmt = conn
    .prepare("SELECT name, value, host, path, expiry FROM moz_cookies")
    .map_err(|e| e.to_string())?;

  let cookie_iter = stmt
    .query_map([], |row| {
      Ok(CookieEntry {
        name: row.get(0)?,
        value: row.get(1)?,
        host: row.get(2)?,
        path: row.get(3)?,
        expiry: row.get(4)?,
      })
    })
    .map_err(|e| e.to_string())?;

  let mut result = Vec::new();
  for entry in cookie_iter {
    result.push(entry.map_err(|e| e.to_string())?);
  }

  Ok(result)
}