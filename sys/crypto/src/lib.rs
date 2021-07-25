extern crate ed25519;
use ed25519::signature::Signature as _;
use ed25519_dalek::*;
use hash::{hex::FromHex, Sha512};

pub const SECRET_KEY_LENGTH: usize = 32;
pub const PUBLIC_KEY_LENGTH: usize = 32;

pub const KEYPAIR_LENGTH: usize = SECRET_KEY_LENGTH + PUBLIC_KEY_LENGTH; // 64usize

pub const SIGNATURE_LENGTH: usize = 64;

#[cfg(test)]
mod tests;
#[test]
fn gen_keypair() {
  let secret_key: &[u8] = b"833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42";
  let public_key: &[u8] = b"ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf";
  let message: &[u8] = b"616263";
  let signature: &[u8] = b"98a70222f0b8121aa9d30f813d683f809e462b469c7ff87639499bb94e6dae4131f85042463c2a355a2003d062adf5aaa10b8c61e636062aaad11c2a26083406";

  let sec_bytes: Vec<u8> = FromHex::from_hex(secret_key).unwrap();
  let pub_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
  let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
  let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

  let secret: SecretKey = SecretKey::from_bytes(&sec_bytes[..SECRET_KEY_LENGTH]).unwrap();
  let public: PublicKey = PublicKey::from_bytes(&pub_bytes[..PUBLIC_KEY_LENGTH]).unwrap();
  let keypair: Keypair = Keypair {
    secret: secret,
    public: public,
  };
  let sig1: Signature = Signature::from_bytes(&sig_bytes[..]).unwrap();
  let mut prehash_for_signing: Sha512 = Sha512::default();
  let mut prehash_for_verifying: Sha512 = Sha512::default();
  prehash_for_signing.update(&msg_bytes[..]);
  prehash_for_verifying.update(&msg_bytes[..]);

  let sig2: Signature = keypair.sign_prehashed(prehash_for_signing, None).unwrap();

  assert!(
    sig1 == sig2,
    "Original signature from test vectors doesn't equal signature produced:\
                \noriginal:\n{:?}\nproduced:\n{:?}",
    sig1,
    sig2
  );
  assert!(
    keypair
      .verify_prehashed(prehash_for_verifying, None, &sig2)
      .is_ok(),
    "Could not verify ed25519ph signature!"
  );
}
