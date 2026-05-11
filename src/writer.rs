use crate::models::{HistoryEntry, CookieEntry, DownloadEntry, FormEntry, BookmarkEntry};
use chrono::{DateTime, Utc};
use std::error::Error;
use std::fs::File;

pub fn save_history_to_csv(entries: &[HistoryEntry], filename: &str) -> Result<(), Box<dyn Error>> {
  let file = File::create(filename)?;
  let mut wtr = csv::Writer::from_writer(file);

  wtr.write_record(&["URL", "Title", "Visit Count", "Last Visit Date"])?;

  for entry in entries {
    let datetime: DateTime<Utc> = DateTime::from_timestamp(entry.last_visit_date / 1_000_000, 0)
      .unwrap_or_default();

    wtr.write_record(&[
      &entry.url,
      &entry.title,
      &entry.visit_count.to_string(),
      &datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
    ])?;
  }

  wtr.flush()?;
  Ok(())
}

pub fn save_cookies_to_csv(entries: &[CookieEntry], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
  let file = File::create(filename)?;
  let mut wtr = csv::Writer::from_writer(file);

  wtr.write_record(&["Name", "Value", "Host", "Path", "Expiry"])?;

  for entry in entries {
    wtr.write_record(&[
      &entry.name,
      &entry.value,
      &entry.host,
      &entry.path,
      &entry.expiry.to_string(),
    ])?;
  }

  wtr.flush()?;
  Ok(())
}

pub fn save_downloads_to_csv(entries: &[DownloadEntry], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
  let file = File::create(filename)?;
  let mut wtr = csv::Writer::from_writer(file);

  wtr.write_record(&["URL", "Local Path", "Download Date"])?;

  for entry in entries {
    let datetime: DateTime<Utc> = DateTime::from_timestamp(entry.date_added / 1_000_000, 0)
      .unwrap_or_default();

    wtr.write_record(&[
      &entry.url,
      &entry.path,
      &datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
    ])?;
  }

  wtr.flush()?;
  Ok(())
}


pub fn save_forms_to_csv(entries: &[FormEntry], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
  let file = std::fs::File::create(filename)?;
  let mut wtr = csv::Writer::from_writer(file);

  wtr.write_record(&["Field Name", "Value", "Times Used", "First Used", "Last Used"])?;

  for entry in entries {
    let first: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_timestamp(entry.first_used / 1_000_000, 0).unwrap_or_default();
    let last: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_timestamp(entry.last_used / 1_000_000, 0).unwrap_or_default();
        
    wtr.write_record(&[
      &entry.fieldname,
      &entry.value,
      &entry.times_used.to_string(),
      &first.format("%Y-%m-%d %H:%M:%S").to_string(),
      &last.format("%Y-%m-%d %H:%M:%S").to_string(),
    ])?;
  }

  wtr.flush()?;
  Ok(())
}

pub fn save_bookmarks_to_csv(entries: &[BookmarkEntry], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
  let file = std::fs::File::create(filename)?;
  let mut wtr = csv::Writer::from_writer(file);

  wtr.write_record(&["Title", "URL", "Date Added"])?;

  for entry in entries {
    let datetime: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_timestamp(entry.date_added / 1_000_000, 0)
      .unwrap_or_default();
        
    wtr.write_record(&[
      &entry.title,
      &entry.url,
      &datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
    ])?;
  }

  wtr.flush()?;
  Ok(())
}
