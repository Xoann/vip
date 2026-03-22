use std::fs;
// use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;

const REPO_DIR: &str = ".vip";
const OBJECTS_DIR: &str = "objects";
const REFS_DIR: &str = "refs";
const HEADS_DIR: &str = "heads";
const TAGS_DIR: &str = "tags";

const HEAD_FILE: &str = "HEAD";
const DEFAULT_BRANCH: &str = "main";

pub fn init() -> std::io::Result<()> {
  let repo_dir = Path::new(REPO_DIR);

  if repo_dir.exists() {
    return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, format!("Repository already exists at {}", repo_dir.display())));
  }

  let objects_dir = repo_dir.join(OBJECTS_DIR);
  let heads_dir = repo_dir.join(REFS_DIR).join(HEADS_DIR);
  let tags_dir = repo_dir.join(REFS_DIR).join(TAGS_DIR);
  
  fs::create_dir_all(objects_dir)?;
  fs::create_dir_all(heads_dir)?;
  fs::create_dir_all(tags_dir)?;
  
  let head_path = repo_dir.join(HEAD_FILE);

  let mut head_file = OpenOptions::new().write(true).create_new(true).open(&head_path)?;
  head_file.write_all(format!("ref: {}/{}/{}\n", REFS_DIR, HEADS_DIR, DEFAULT_BRANCH).as_bytes())?;

  Ok(())
}