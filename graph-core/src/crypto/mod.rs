mod pkce;

pub use pkce::*;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
pub fn secure_random_32() -> String {
    let mut buf = [0; 32];

    graviola::random::fill(&mut buf);

    URL_SAFE_NO_PAD.encode(buf)
}
