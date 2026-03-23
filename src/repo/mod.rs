use std::path::{Path, PathBuf};
use std::io::Result;
use std::fs;
use sha1::{Sha1, Digest};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::{Read, Write};

mod constants;
use self::constants::*;
mod object;
use self::object::Object;
use self::object::ObjectType;


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

  // Path helpers
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

  // objects 

  pub fn hash_blob<P: AsRef<Path>>(&self, path: P) -> Result<String> {
    let content = fs::read(path)?;
    let header = format!("blob {}\0", content.len()); 
    let object_bytes = [header.as_bytes(), &content].concat();

    let hash = Sha1::digest(&object_bytes);
    let hash_hex = format!("{:x}", hash);

    let object_dir = self.object_dir(&hash_hex);
    let object_path = self.object_path(&hash_hex);
    
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

  pub fn read_object(&self, hash: &str) -> Result<Object> {
    let object_path = self.object_path(hash);

    if !object_path.exists() {
      return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Object not found"));
    }

    let compressed = fs::read(object_path)?;
    let mut decoder = flate2::read::ZlibDecoder::new(&compressed[..]);
    let mut object_bytes = Vec::new();
    decoder.read_to_end(&mut object_bytes)?;

    let null_index = object_bytes
    .iter()
    .position(|&b| b == 0)
    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid object header"))?;
  
    let header_bytes = &object_bytes[..null_index];
    let header = std::str::from_utf8(header_bytes).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "object header is not valid UTF-8",
            )
        })?;
    let mut parts = header.split_whitespace();

    let content_type_str = parts.next().ok_or_else(|| {
      std::io::Error::new(std::io::ErrorKind::InvalidData, "object header missing type")
    })?;
    let content_size_str = parts.next().ok_or_else(|| {
      std::io::Error::new(std::io::ErrorKind::InvalidData, "object header missing size")
    })?;

    let content_size: usize = content_size_str.parse().map_err(|_| {
      std::io::Error::new(std::io::ErrorKind::InvalidData, "object header has invalid size")
    })?;

    let content = object_bytes[null_index + 1..].to_vec();

    if content.len() != content_size {
      return Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("object content size mismatch: expected {}, got {}", content_size, content.len()),
      ));
    }

    let object_type = content_type_str.parse::<ObjectType>()?;

    Ok(Object {
      object_type,
      size: content_size,
      content,
    })
  }
}