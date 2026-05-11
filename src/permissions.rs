use rusqlite::Connection;
use crate::models::PermissionEntry;

pub fn extract_permissions(conn: &Connection) -> Result<Vec<PermissionEntry>, String> {
  let mut stmt = conn
    .prepare("SELECT origin, type, permission, expireTime FROM moz_perms")
    .map_err(|e| e.to_string())?;

  let perm_iter = stmt
    .query_map([], |row| {
      Ok(PermissionEntry {
        origin: row.get(0)?,
        perm_type: row.get(1)?,
        permission: row.get(2)?,
        expire_time: row.get(3)?,
      })
    })
    .map_err(|e| e.to_string())?;

  let mut results = Vec::new();
  for entry in perm_iter {
    results.push(entry.map_err(|e| e.to_string())?);
  }
  
  Ok(results)
}