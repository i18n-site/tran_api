tonic::include_proto!("api");

use bytes::Bytes;
use grpc_client::endpoint;
use tonic::{
  codec::CompressionEncoding,
  transport::{Channel, Error},
};

use crate::api_client::ApiClient;

pub async fn conn(addr: impl Into<Bytes>) -> Result<ApiClient<Channel>, Error> {
  Ok(
    ApiClient::connect(endpoint(addr)?)
      .await?
      .send_compressed(CompressionEncoding::Gzip)
      .accept_compressed(CompressionEncoding::Gzip),
  )
}
