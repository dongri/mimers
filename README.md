# mimers

mimers provides useful MIME related functions. For Quoted-Printable and Base64 encoding

### examples
```rust
extern crate mimers;

use mimers::{WordEncoder, Encoding};

fn main() {
  let encoder = WordEncoder::new("UTF-8", Encoding::QEncoding);
  let encoded_word = encoder.encode_word("abcあいうえお".to_string());
  println!("{}", encoded_word); // =?UTF-8?Q?abc=E3=81=82=E3=81=84=E3=81=86=E3=81=88=E3=81=8A?=
}
```
