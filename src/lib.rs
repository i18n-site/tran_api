tonic::include_proto!("api");

#[cfg(feature = "conn")]
mod conn;

#[cfg(feature = "conn")]
pub use conn::conn;

#[cfg(feature = "file_type")]
pub fn file_type(filename: impl AsRef<str>) -> FileType {
  let filename = filename.as_ref();
  if let Some(ext) = filename.rsplit('.').next()
    && ["yml", "yaml"].contains(&ext)
  {
    return FileType::Yml;
  }

  FileType::Md
}
