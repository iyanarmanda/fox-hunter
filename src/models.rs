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

pub struct FormEntry {
  pub fieldname: String,
  pub value: String,
  pub times_used: i32,
  pub first_used: i64,
  pub last_used: i64,
}

pub struct BookmarkEntry {
  pub title: String,
  pub url: String,
  pub date_added: i64,
}