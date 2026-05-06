use std::path::Path;

pub fn validate_path(path_str: &str) -> bool {
  let path = Path::new(path_str);
  
  path.exists() && path.is_dir()
}