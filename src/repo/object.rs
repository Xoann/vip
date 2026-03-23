use std::str::FromStr;
use std::fmt;

pub struct Object {
  pub object_type: ObjectType,
  pub content: Vec<u8>,
  pub size: usize,
}

pub enum ObjectType {
  Blob,
  Tree,
  Commit,
}

impl FromStr for ObjectType {
  type Err = std::io::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "blob" => Ok(ObjectType::Blob),
      "tree" => Ok(ObjectType::Tree),
      "commit" => Ok(ObjectType::Commit),
      _ => Err(std::io::Error::new(
              std::io::ErrorKind::InvalidData,
              format!("unsupported object type: {}", s),
          )),
    }
  }
}

impl fmt::Display for ObjectType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ObjectType::Blob => write!(f, "blob"),
      ObjectType::Tree => write!(f, "tree"),
      ObjectType::Commit => write!(f, "commit"),
    }
  }
}