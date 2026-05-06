use rusqlite::Connection;
use crate::models::DownloadEntry;

pub fn extract_downloads(conn: &Connection) -> Result<Vec<DownloadEntry>, String> {
  let mut stmt = conn.prepare("
    SELECT p.url, a.content, a.dateAdded
    FROM moz_annos a
    JOIN moz_places p ON a.place_id = p.id
    WHERE a.anno_attribute_id = (
      SELECT id FROM moz_anno_attributes WHERE name = 'downloads/destinationFileURI'
    )
    ORDER BY a.dateAdded DESC
  ").map_err(|e| e.to_string())?;

  let download_iter = stmt.query_map([], |row| {
    Ok(DownloadEntry {
      url: row.get(0)?,
      path: row.get(1)?,
      date_added: row.get(2)?,
    })
  }).map_err(|e| e.to_string())?;

  let mut result = Vec::new();
  for entry in download_iter {
    result.push(entry.map_err(|e| e.to_string())?);
  }

  Ok(result)
}