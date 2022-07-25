use std::path::Path;
use obby::{collect_files, get_file_contents};

pub fn convert_dates() {
  //  TODO: Needs implementation
  println!("Converting dates!");
}

pub fn list_dangling_links() {
  //  TODO: Needs implementation
  println!("Listing dangling links!");
}

pub fn list_empty_files(working_directory: String) {
  let paths = collect_files(Path::new(&working_directory));

  let mut empty_file_paths = Vec::new();
  for path in paths {
    if get_file_contents(Path::new(&path)).len() == 0 {
      empty_file_paths.push(path);
    }
  }

  pp_path_vec(&empty_file_paths);
}

pub fn list_referenceless_files() {
  //  TODO: Needs implementation
  println!("Listing referenceless files");
}

fn pp_path_vec(paths: &Vec<String>) {
  for path in paths {
    println!("{}", path);
  }
}