use quoted_printable::{encode_to_str};
use base64::{encode};

#[derive(PartialEq)]
pub enum Encoding {
  BEncoding,
  QEncoding,
}

impl Encoding {
  pub fn name(&self) -> &str {
    match *self {
      Encoding::BEncoding => "B",
      Encoding::QEncoding => "Q",
    }
  }
}

pub struct WordEncoder {
  charset: &'static str,
  encoding: Encoding,
}

impl WordEncoder {

  pub fn new(charset: &'static str, encoding: Encoding) -> WordEncoder {
    WordEncoder {
      charset: charset,
      encoding: encoding,
    }
  }

  pub fn encode_word(&self, data: String) -> String {
    let to_charset = self.charset;
    let mut encoded_str = match self.encoding {
      Encoding::BEncoding => encode(data.as_bytes()),
      Encoding::QEncoding => encode_to_str(data).replace("",""),
    };
    encoded_str = encoded_str.replace(" ", "_");
    let mut last_token = "";
    let len = encoded_str.len();
    if &encoded_str[len-2..len] != "?=" {
      last_token = "?=";
    }
    return format!("=?{}?{}?{}{}", to_charset, self.encoding.name(), encoded_str, last_token);
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let e = WordEncoder::new("UTF-8", Encoding::QEncoding);
    assert_eq!(e.encode_word("サポート".to_string()), "=?UTF-8?Q?=E3=82=B5=E3=83=9D=E3=83=BC=E3=83=88?=");
    assert_eq!(e.encode_word("abcあいうえお".to_string()), "=?UTF-8?Q?abc=E3=81=82=E3=81=84=E3=81=86=E3=81=88=E3=81=8A?=");
    assert_eq!(e.encode_word("abc한국어".to_string()), "=?UTF-8?Q?abc=ED=95=9C=EA=B5=AD=EC=96=B4?=");
    assert_eq!(e.encode_word("abc日本語".to_string()), "=?UTF-8?Q?abc=E6=97=A5=E6=9C=AC=E8=AA=9E?=");
    assert_eq!(e.encode_word("abc中文".to_string()), "=?UTF-8?Q?abc=E4=B8=AD=E6=96=87?=");
    assert_eq!(e.encode_word("abc def".to_string()), "=?UTF-8?Q?abc_def?=");
  }
}
