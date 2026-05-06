pub struct HistoryEntry {
  pub url: String,
  pub title: String,
  pub visit_count: i32,
  pub last_visit_date: i64,
}

pub struct CookieEntry {
  pub name: String,
  pub value: String,
  pub host: String,
  pub path: String,
  pub expiry: i64,
}

pub struct DownloadEntry {
  pub url: String,
  pub path: String,
  pub date_added: i64,
}