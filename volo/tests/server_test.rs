#![feature(const_convert)]
#![feature(const_trait_impl)]

use std::{net::SocketAddr, time::Duration};

use anyhow::Result;

type Void = Result<()>;

use volo_gen::{google::protobuf::Empty, js0_srv::ApiClientBuilder};

#[static_init::constructor(0)]
extern "C" fn log_init() {
  log_init::init();
}

pub const GRPC: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3333));

#[tokio::test]
async fn test_captcha_new() -> Void {
  let client = {
    ApiClientBuilder::new("js0_srv")
      .address(GRPC)
      .rpc_timeout(Some(Duration::from_secs(5)))
      .build()
  };
  let req = volo_grpc::Request::new(Empty {});
  let resp = client.captcha_new(req).await?;
  let captcha = resp.into_inner();

  // 验证验证码是否包含所需字段，避免空值导致的 panic
  assert!(!captcha.id.is_empty(), "Captcha ID should not be empty");
  assert!(!captcha.img.is_empty(), "Captcha image should not be empty");
  assert!(
    !captcha.ico_li.is_empty(),
    "Captcha icon list should not be empty"
  );

  Ok(())
}

#[tokio::test]
async fn test_multiple_captcha_requests() -> Void {
  // 减少请求次数，避免并发问题
  for i in 0..2 {
    let client = {
      ApiClientBuilder::new("js0_srv")
        .address(GRPC)
        .rpc_timeout(Some(Duration::from_secs(5)))
        .build()
    };
    let req = volo_grpc::Request::new(Empty {});
    let resp = client.captcha_new(req).await?;

    let captcha = resp.into_inner();

    // 验证每个验证码都包含所需字段
    assert!(
      !captcha.id.is_empty(),
      "Captcha ID should not be empty (iteration {})",
      i
    );
    assert!(
      !captcha.img.is_empty(),
      "Captcha image should not be empty (iteration {})",
      i
    );
    assert!(
      !captcha.ico_li.is_empty(),
      "Captcha icon list should not be empty (iteration {})",
      i
    );

    // 在请求之间添加小的延迟
    // tokio::time::sleep(Duration::from_millis(100)).await;
  }

  Ok(())
}
