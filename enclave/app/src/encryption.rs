use rand;
use base64::prelude::*;
use rsa::{pkcs8::EncodePublicKey, RsaPrivateKey, RsaPublicKey, Oaep, sha2::Sha256};
use serde_bytes::ByteBuf;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;

#[derive(Clone)]
pub struct Encryption {
    priv_key: RsaPrivateKey,
    pub_key: RsaPublicKey,
}

impl Encryption {
    /// Constructor
    pub fn new() -> Encryption {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits)
            .expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        Encryption {
            priv_key: priv_key,
            pub_key: pub_key,
        }
    }

    pub fn get_pub_key_byte (&self) -> ByteBuf {
        let pub_key_der = self.pub_key
            .to_public_key_der()
            .expect("Failed to convert public key into DER format");
        ByteBuf::from(pub_key_der.as_bytes())
    }

    pub fn decrypt (&self, encrypted_secret: String) -> Vec<u8> {
        let parts: Vec<&str> = encrypted_secret.split(":")
            .collect();

        let enc_session_key_b64 = parts[0];
        let nonce_b64 = parts[1];
        let ciphertext_b64 = parts[2];

        let enc_session_key = BASE64_STANDARD.decode(enc_session_key_b64)
            .expect("Failed to decode enc_session_key");
        let nonce = BASE64_STANDARD.decode(nonce_b64)
            .expect("Failed to decode nonce");
        let ciphertext = BASE64_STANDARD.decode(ciphertext_b64)
            .expect("Failed to decode ciphertext");

        let session_key = self.priv_key
            .decrypt(Oaep::new::<Sha256>(), enc_session_key.as_slice())
            .expect("Failed to decrypt session key");
        let cipher = Aes256Gcm::new_from_slice(&session_key.as_slice())
            .expect("Failed to create cipher");
        
        cipher.decrypt(Nonce::from_slice(nonce.as_slice()), ciphertext.as_slice())
            .expect("Failed to decrypt ciphertext")
    }
}
