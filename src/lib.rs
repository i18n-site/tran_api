tonic::include_proto!("api");

use concat_array::concat_array;

#[cfg(feature = "conn")]
mod conn;

#[cfg(feature = "conn")]
pub use conn::conn;

pub const EXT_YML: [&str; 2] = ["yml", "yaml"];
pub const EXT_MD: [&str; 1] = ["md"];
pub const EXT_HTM: [&str; 2] = ["htm", "html"];

pub const EXT: &[&str] = &concat_array!(EXT_YML, EXT_MD, EXT_HTM);
//
// #[cfg(feature = "file_type")]
// pub enum FileType {
//   Yml,
//   Md,
//   Htm,
// }
//
// #[cfg(feature = "file_type")]
// pub fn file_type(filename: impl AsRef<str>) -> FileType {
//   let filename = filename.as_ref();
//   if let Some(ext) = filename.rsplit('.').next() {
//     for (ext_li, file_type) in [(EXT_YML, FileType::Yml), (EXT_HTM, FileType::Htm)] {
//       if ext_li.contains(&ext) {
//         return file_type;
//       }
//     }
//   }
//
//   FileType::Md
// }
