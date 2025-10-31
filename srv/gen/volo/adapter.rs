//  DON'T EDIT ! AUTOGEN BY rs2proto

use xrpc::{AsyncCall, Call, Ext, Func, Map, ReqArgs, Result};

pub struct CaptchaNew;

pub fn captcha_new(
  _: &volo_gen::google::protobuf::Empty,
) -> anyhow::Result<volo_gen::js0_srv::Captcha> {
  let r = js0_srv::captcha::new()?;
  Ok(volo_gen::js0_srv::Captcha {
    id: r.id.into(),
    img: r.img.into(),
    ico_li: r.ico_li.into_iter().map(|i| i.into()).collect(),
  })
}

impl Func for CaptchaNew {
  type Args = volo_gen::google::protobuf::Empty;
  type Result = volo_gen::js0_srv::Captcha;
  fn name() -> &'static str {
    "captcha_new"
  }
}

impl Call for CaptchaNew {
  fn inner<H: Map, E: Ext>(req_args: ReqArgs<H, E, Self::Args>) -> Result<Self::Result> {
    captcha_new(req_args.args).into()
  }
}
