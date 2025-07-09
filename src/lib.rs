tonic::include_proto!("api");

use concat_array::concat_array;

#[cfg(feature = "conn")]
mod conn;

#[cfg(feature = "conn")]
pub use conn::conn;

pub const EXT_YML: [&str; 2] = ["yml", "yaml"];
pub const EXT_MD: [&str; 1] = ["md"];

pub const EXT: &[&str] = &concat_array!(EXT_YML, EXT_MD);

#[cfg(feature = "file_type")]
pub fn file_type(filename: impl AsRef<str>) -> FileType {
  let filename = filename.as_ref();
  if let Some(ext) = filename.rsplit('.').next()
    && EXT_YML.contains(&ext)
  {
    return FileType::Yml;
  }

  FileType::Md
}
