tonic::include_proto!("api");
use CompressionEncoding::Gzip;
use bytes::Bytes;
use grpc_client::endpoint;
use tonic::{
  Request, Response,
  body::Body,
  client::GrpcService,
  codec::CompressionEncoding,
  service::{Interceptor, interceptor::InterceptedService},
  transport::{Channel, Error},
};

use crate::api_client::ApiClient;

pub fn req_interceptor(mut req: tonic::Request<()>) -> tonic::Result<tonic::Request<()>> {
  req
    .metadata_mut()
    .insert("Authorization", "XXXXXXXXXXXX".try_into().unwrap());
  Ok(req)
}
pub async fn conn(addr: impl Into<Bytes>) -> Result<ApiClient<Channel>, Error> {
  let channel = endpoint(addr)?;

  let client = ApiClient::connect(channel)
    .await?
    .send_compressed(Gzip)
    .accept_compressed(Gzip);

  Ok(client)
}
