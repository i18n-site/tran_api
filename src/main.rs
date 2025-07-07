genv::s!(I18N_GRPC: String | "http://127.0.0.1:3333/".to_owned());

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api = tran_api::conn(I18N_GRPC.as_str()).await?;

  Ok(())
}
