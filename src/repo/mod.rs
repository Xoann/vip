use std::path::{Path, PathBuf};

mod constants;
use self::constants::*;


pub struct Repo {
  root: PathBuf,
}

impl Repo {
  pub fn new() -> Self {
    Self {
      root: PathBuf::from(REPO_DIR),
    }
  }

  pub fn root(&self) -> &Path {
    &self.root
  }

  pub fn objects_dir(&self) -> PathBuf {
    self.root.join(OBJECTS_DIR)
  }

  pub fn object_dir(&self, hash: &str) -> PathBuf {
    let (dir_name, _) = hash.split_at(2);
    self.objects_dir().join(dir_name)
  }

  pub fn object_path(&self, hash: &str) -> PathBuf {
    let (_, file_name) = hash.split_at(2);
    self.object_dir(hash).join(file_name)
  }

  pub fn refs_dir(&self) -> PathBuf {
    self.root.join(REFS_DIR)
  }

  pub fn heads_dir(&self) -> PathBuf {
      self.refs_dir().join(HEADS_DIR)
  }

  pub fn tags_dir(&self) -> PathBuf {
      self.refs_dir().join(TAGS_DIR)
  }

  pub fn head_path(&self) -> PathBuf {
      self.root.join(HEAD_FILE)
  }
}