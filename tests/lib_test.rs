use std::path::Path;
use obby::{get_file_contents, collect_files};

#[test]
fn test_get_file() {
  assert_eq!(
    get_file_contents(Path::new("test_fixtures/empty.txt")), 
    ""
  );

  assert_eq!(
    get_file_contents(Path::new("test_fixtures/hello world.txt")), 
    "hello world"
  ); 
}

#[test]
fn test_collect_files() {
  let mut paths = collect_files(Path::new("test_fixtures/test_dir"));

  paths.sort();

  let expected = vec![
    "test_fixtures/test_dir/a.txt",
    "test_fixtures/test_dir/b.txt",
    "test_fixtures/test_dir/dir_1/1.txt",
    "test_fixtures/test_dir/dir_2/2.txt"
  ];

  assert_eq!(paths.len(), expected.len());

  for i in 0..expected.len() {
    assert_eq!(
      paths.get(i).unwrap(),
      expected.get(i).unwrap()
    );
  }
}
