//! Used for calculating fingerprint of HTTPS certificates.
//! Also provides a way to split the certs 

use pem::parse;
use sha2::{Sha256, Digest};


// Converts pem into bytes and returns the Sha 256 digest of the bytes
pub fn calculate_fingerprint(certificate: &[u8]) -> Option<Vec<u8>> {
  parse(certificate).map(|data| {
    Sha256::digest(&data.contents).iter().cloned().collect()
  }).ok()
}

// As der is already in bytes we can directly convert bytes into Sha 256 digestss
pub fn calculate_fingerprint_from_der(certificate: &[u8]) -> Vec<u8> {
  Sha256::digest(&certificate).iter().cloned().collect()
}


pub fn split_certificate_chain(mut chain: String) -> Vec<String> {
  let mut v = Vec::new();

  let end = "-----END CERTIFICATE-----";
  loop {
    match chain.find(end) {
      Some(sz) => {
        let cert: String = chain.drain(..sz+end.len()).collect();
        v.push(cert.trim().to_string());
      },
      None     => break,
    }
  }
  v
}
