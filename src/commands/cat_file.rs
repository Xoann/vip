use crate::repo::Repo;

pub enum CatFileMode {
  Type,
  Size,
  Content,
}

pub fn cat_file(hash: &str, mode: CatFileMode) -> std::io::Result<()> {
  let repo = Repo::new();
  let object = repo.read_object(hash)?;

  match mode {
    CatFileMode::Content => {
          print!("{}", String::from_utf8_lossy(&object.content));
      }
      CatFileMode::Type => {
          println!("{}", object.object_type.to_string());
      }
      CatFileMode::Size => {
          println!("{}", object.size);
      }
  }
  Ok(())
}


