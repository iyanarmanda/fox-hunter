use std::env;

mod discovery;
mod database;
mod history;
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

    match database::establish_connection(target_path, "places.sqlite") {
      Ok(conn) => {
        println!("[+] Successfully connected to places.sqlite");

        match history::extract_history(&conn) {
          Ok(entries) => {
            let csv_file = "/outputs/history_report.csv";
            match writer::save_to_csv(&entries, csv_file) {
              Ok(_) => println!("\n[+] All {} entries successfully saved to {}", entries.len(), csv_file),
              Err(e) => println!("[!] Failed to save history to CSV: {}", e),
            }

            println!("\n[+] Successfully extracted {} history entries.", entries.len());
          },
          Err(e) => println!("[!] Error extracting history: {}", e),
        }
      },
      Err(e) => {
        println!("[!] Database connection error: {}", e);
      }
    }
  } else {
    println!("[!] Error: Path not found or not a directory: {}", target_path);
  }
}