mod attestation;
mod encryption;
mod http;

use std::io::Write;

use libc;
use tokio_vsock::{VsockAddr, VsockListener, VsockStream};

use attestation::get_attestation_doc;
use encryption::Encryption;
use http::get_encrypted_secret_from_api;

const LISTENING_PORT: u32 = 1000;

async fn process(mut socket: VsockStream, encryption: &Encryption) {
    let public_key = Some(encryption.get_pub_key_byte());
    let user_data = None;
    let nonce = None;
    
    let attestation_doc = get_attestation_doc(public_key, user_data, nonce)
        .expect("Cannot get attestation document");

    let encrypted_secret = get_encrypted_secret_from_api(attestation_doc)
        .await;

    let secret = encryption.decrypt(encrypted_secret);

    let _ = socket.write_all(secret.as_slice());
}

#[tokio::main]
async fn main() {
    let encryption = Encryption::new();

    let listen_port = LISTENING_PORT;
    let addr = VsockAddr::new(libc::VMADDR_CID_ANY, listen_port);
    let mut listener = VsockListener::bind(addr)
        .expect("fail to bind address");

    println!("Listening for connections on port: {}", listen_port);

    loop {
        let encryption_clone = encryption.clone();

        match listener.accept().await {
            Ok((socket, _)) => {
                tokio::spawn(async move {
                    process(socket, &encryption_clone).await;
                });
            },
            Err(_) => {}
        };
    }
}
