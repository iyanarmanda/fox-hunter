use rusqlite::Connection;
use crate::models::FormEntry;

pub fn extract_forms(conn: &Connection) -> Result<Vec<FormEntry>, String> {
  let mut stmt = conn
    .prepare("SELECT fieldname, value, timesUsed, firstUsed, lastUsed FROM moz_formhistory ORDER BY lastUsed DESC")
    .map_err(|e| e.to_string())?;

  let form_iter = stmt
    .query_map([], |row| {
      Ok(FormEntry {
        fieldname: row.get(0)?,
        value: row.get(1)?,
        times_used: row.get(2)?,
        first_used: row.get(3)?,
        last_used: row.get(4)?,
      })
    })
    .map_err(|e| e.to_string())?;

  let mut results = Vec::new();
  for entry in form_iter {
    results.push(entry.map_err(|e| e.to_string())?);
  }
  
  Ok(results)
}