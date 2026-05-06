use crate::models::HistoryEntry;
use chrono::{DateTime, Utc};
use std::error::Error;
use std::fs::File;

pub fn save_to_csv(entries: &[HistoryEntry], filename: &str) -> Result<(), Box<dyn Error>> {
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