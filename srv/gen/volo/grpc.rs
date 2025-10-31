//  DON'T EDIT ! AUTOGEN BY rs2proto

use volo_grpc::{Response, Status};
use xrpc::{AsyncCall, Call, IntoResponse, volo::grpc::split};

use crate::{Log, adapter};

pub struct S;

impl volo_gen::js0_srv::Api for S {
  async fn captcha_new(
    &self,
    req: volo_grpc::Request<volo_gen::google::protobuf::Empty>,
  ) -> Result<Response<volo_gen::js0_srv::Captcha>, Status> {
    adapter::CaptchaNew::call::<Log, _, _>(split(req)).into_response()
  }
}
