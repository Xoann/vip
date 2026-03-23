use std::path::Path;
use crate::repo::Repo;

pub fn hash_object(path: &Path) -> std::io::Result<()> {
  let repo = Repo::new();
  let hash_hex = repo.hash_blob(path)?;
  println!("{}", hash_hex);
  Ok(())
}