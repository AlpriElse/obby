use std::fs;
use std::path::Path;

/// Returns a String with the contents of a file
pub fn get_file_contents(path: &Path) -> String {
  let file = fs::read(&path).expect("Problem opening the file");

  return String::from_utf8_lossy(&file).into();
}

/// Returns a Vec<String> of paths within a directory recursively
pub fn collect_files(dir_path: &Path) -> Vec<String> {
  if !dir_path.is_dir() {
    return vec![];
  }

  let mut files: Vec<String> = Vec::new();

  if let Ok(entries) = fs::read_dir(dir_path) {
    for entry_result in entries {
      let entry = match entry_result {
        Ok(f) => f,
        Err(e) => {
          println!("{:?}", e);
          continue;
        },
      };

      let file_type = match entry.file_type() {
        Ok(ft) => ft,
        Err(e) => {
          println!("{:?}", e);
          continue;
        },
      };

      let path = match entry.path().to_str() {
        Some(p) => String::from(p),
        None => continue,
      };

      if file_type.is_file() {
        files.push(path);
        continue;
      }  

      if file_type.is_dir() {
        let mut deep_files = collect_files(Path::new(&path));
        files.append(&mut deep_files);

        continue;
      }
    }
  }

  return files;
}