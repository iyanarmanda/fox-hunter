use std::env;

mod discovery;
mod database;
mod history;
mod cookies;
mod download;
mod forms;
mod writer;
mod models;

fn main() {
  println!("--- Fox-Hunter: Digital Forensic Triage Tool ---");

  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("[!] Error: Target path not specified.");
    println!("Use: cargo run -- <PATH_TO_FIREFOX_PROFILE>");
    println!("Example: cargo run -- /home/user/.mozilla/firefox/xxxxxxxx.default-release/");
    return;
  }

  let target_path = &args[1];

  if discovery::validate_path(target_path) {
    println!("[+] Path validated: {}", target_path);
    println!("[*] Preparing artifact extraction...");

    if let Ok(conn) = database::establish_connection(target_path, "places.sqlite") {
      match history::extract_history(&conn) {
        Ok(entries) => {
          let _ = writer::save_history_to_csv(&entries, "outputs/history_report.csv");
          println!("[+] Extracted {} history entries to 'outputs/history_report.csv'", entries.len());
        },
        Err(e) => println!("[!] Failed to extract history: {}", e),
      }

      match download::extract_downloads(&conn) {
        Ok(entries) => {
          let _ = writer::save_downloads_to_csv(&entries, "outputs/download_report.csv");
          println!("[+] Extracted {} download entries to 'outputs/download_report.csv'", entries.len());
        },
        Err(e) => println!("[!] Failed to extract download: {}", e)
      }
    }

    if let Ok(conn) = database::establish_connection(target_path, "cookies.sqlite") {
      match cookies::extract_cookies(&conn) {
        Ok(entries) => {
          let _ = writer::save_cookies_to_csv(&entries, "outputs/cookies_report.csv");
          println!("[+] Extracted {} cookie entries to 'outputs/cookies_report.csv'", entries.len());
        },
        Err(e) => println!("[!] Failed to extract cookies: {}", e),
      }
    }

    if let Ok(conn) = database::establish_connection(target_path, "formhistory.sqlite") {
      match forms::extract_forms(&conn) {
        Ok(entries) => {
          let _ = writer::save_forms_to_csv(&entries, "outputs/forms_report.csv");
          println!("[+] Extracted {} data form to 'outputs/forms_report.csv'.", entries.len());
        },
        Err(e) => println!("[!] Failed to extract form: {}", e),
      }
    }

  } else {
    println!("[!] Error: Path not found or not a directory: {}", target_path);
  }
}