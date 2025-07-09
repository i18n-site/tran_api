tonic::include_proto!("api");

#[cfg(feature = "conn")]
mod conn;

#[cfg(feature = "conn")]
pub use conn::conn;
