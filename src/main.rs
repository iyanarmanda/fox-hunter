use std::env;
mod discovery;
mod database;

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
      Ok(_conn) => {
        println!("[+] Successfully connected to places.sqlite");
      },
      Err(e) => {
        println!("[!] Database connection error: {}", e);
      }
    }
  } else {
    println!("[!] Error: Path not found or not a directory: {}", target_path);
  }
}