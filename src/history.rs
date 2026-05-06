use rusqlite::Connection;
use crate::models::HistoryEntry;

pub fn extract_history(conn: &Connection) -> Result<Vec<HistoryEntry>, String> {
  let mut stmt = conn
    .prepare("SELECT url, title, visit_count, last_visit_date FROM moz_places WHERE last_visit_date IS NOT NULL ORDER BY last_visit_date DESC")
    .map_err(|e| e.to_string())?;

  let history_iter = stmt
    .query_map([], |row | {
      Ok(HistoryEntry {
        url: row.get(0)?,
        title: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
        visit_count: row.get(2)?,
        last_visit_date: row.get(3)?,
      })
    })
    .map_err(|e| e.to_string())?;

  let mut result = Vec::new();
  for entry in history_iter {
    result.push(entry.map_err(|e| e.to_string())?);
  }

  Ok(result)
}