#![cfg_attr(docsrs, feature(doc_cfg))]

use aok::Result;
use click_captcha::{ICO_LI, IcoPosLi, PosLi, webp};
use uuid::Uuid;
use xkv::{
  R,
  fred::{interfaces::KeysInterface, types::Expiration},
};

const R_CAPTCHA: &[u8] = b"captcha:";

pub struct Captcha {
  pub id: Vec<u8>,
  pub img: Vec<u8>,
  pub ico_li: Vec<String>,
}

pub fn new() -> Result<Captcha> {
  let id = Uuid::new_v4().to_bytes_le().to_vec();

  let (img, IcoPosLi { ico_li, pos_li }) = webp(500, 500, ICO_LI)?;

  tokio::spawn({
    let key = xbin::concat!(R_CAPTCHA, id);
    let pos_li = bitcode::encode(&pos_li);
    async move {
      xerr::log!(
        R.set::<(), _, _>(&key[..], pos_li, Some(Expiration::EX(180)), None, false)
          .await
      );
    }
  });

  Ok(Captcha {
    id,
    img: img.into(),
    ico_li: ico_li.into_iter().map(|pos| ICO_LI[pos].into()).collect(),
  })
}

#[rs2proto::ignore]
pub async fn verify(id: &[u8], xy_li: &[u32]) -> Result<bool> {
  if let Some(pos_li) = R
    .get::<Option<Vec<u8>>, _>(&xbin::concat!(R_CAPTCHA, id)[..])
    .await?
    && let Ok(pos_li) = bitcode::decode::<PosLi>(&pos_li)
  {
    return Ok(click_captcha::verify(pos_li, xy_li));
  }

  Ok(false)
}
