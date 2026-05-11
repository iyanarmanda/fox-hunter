use rusqlite::Connection;
use crate::models::BookmarkEntry;

pub fn extract_bookmarks(conn: &Connection) -> Result<Vec<BookmarkEntry>, String> {
  let mut stmt = conn.prepare("
    SELECT b.title, p.url, b.dateAdded 
    FROM moz_bookmarks b
    JOIN moz_places p ON b.fk = p.id
    WHERE b.type = 1
    ORDER BY b.dateAdded DESC
  ").map_err(|e| e.to_string())?;

  let bookmark_iter = stmt.query_map([], |row| {
    Ok(BookmarkEntry {
      title: row.get::<_, Option<String>>(0)?.unwrap_or_default(),
      url: row.get(1)?,
      date_added: row.get(2)?,
    })
  }).map_err(|e| e.to_string())?;

  let mut results = Vec::new();
  for entry in bookmark_iter {
    results.push(entry.map_err(|e| e.to_string())?);
  }
  
  Ok(results)
}