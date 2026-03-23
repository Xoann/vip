use std::fs;
use sha1::{Sha1, Digest};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::path::Path;
use std::io::Write;
use crate::repo::Repo;

pub fn hash_object(path: &Path) -> std::io::Result<String> {
  let repo = Repo::new();

  let content = fs::read(path)?;
  let header = format!("blob {}\0", content.len()); 
  let object_bytes = [header.as_bytes(), &content].concat();

  let hash = Sha1::digest(&object_bytes);
  let hash_hex = format!("{:x}", hash);

  let object_dir = repo.object_dir(&hash_hex);
  let object_path = repo.object_path(&hash_hex);
  
  if object_path.exists() {
    return Ok(hash_hex);
  }
  
  fs::create_dir_all(&object_dir)?;

  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
  encoder.write_all(&object_bytes)?;
  let compressed = encoder.finish()?;

  fs::write(&object_path, compressed)?;

  Ok(hash_hex)
}