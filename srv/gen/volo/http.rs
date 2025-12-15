//  DON'T EDIT ! AUTOGEN BY rs2proto

use http_grpc::Response;
use volo_http::Bytes;
use xrpc::{
  AsyncCall, Call,
  volo::http::{Req, parse},
};

use crate::{Log, adapter};

pub struct Api;

impl http_grpc::Grpc for Api {
  async fn run(req: Req, func_id: u32, args: Bytes) -> Option<Response> {
    Some(match func_id {
      1 => adapter::CaptchaNew::call::<Log, _, _>((req, parse(args))).into(),
      _ => {
        log::warn!("unkown func_id {func_id}");
        return None;
      }
    })
  }
}
