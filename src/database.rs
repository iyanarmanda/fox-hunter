use rusqlite::{Connection, OpenFlags};
use std::path::Path;

pub fn establish_connection(profile_path: &str, db_name:&str) -> Result<Connection, String> {
  let db_path = Path::new(profile_path).join(db_name);

  if !db_path.exists() {
    return Err(format!("Database file not found: {}", db_name));
  }

  match Connection::open_with_flags(
    &db_path,
    OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
  ) {
    Ok(conn) => Ok(conn),
    Err(e) => Err(format!("Failed to open database: {}", e)),
  }
}