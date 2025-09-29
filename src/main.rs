#[cfg(feature = "bin")]
genv::s!(I18N_GRPC: String | "http://127.0.0.1:3333".to_owned());

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  #[cfg(feature = "bin")]
  {
    let mut api = tran_api::conn(I18N_GRPC.as_str(), Ok).await?;
    let version = api.version(tran_api::Void {}).await?.into_inner();
    println!("v{}.{}.{}", version.major, version.minor, version.patch);
  }
  Ok(())
}
