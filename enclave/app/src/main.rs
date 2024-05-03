mod attestation;
mod encryption;

use serde_bytes::ByteBuf;
use rocket::State;
use rocket::serde::{Deserialize, json::Json};

use attestation::get_attestation_doc;
use encryption::Encryption;

#[macro_use] extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GetAttestationReq {
    nonce: Option<String>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ProcessReq {
    encrypted_payload: String,
}

#[get("/health-check")]
fn health_check() -> String {
    "".to_string()
}

#[post("/get-attestation", data = "<req>")]
fn get_attestation(req: Option<Json<GetAttestationReq>>, encryption: &State<Encryption>) -> String {
    let nonce = match req {
        Some(req) => {
            match req.nonce.to_owned() {
                Some(nonce) => Some(ByteBuf::from(nonce)),
                None => None
            }
        },
        None => None
    };

    let public_key = Some(encryption.get_pub_key_byte());
    let user_data = None;
    
    let attestation_doc = get_attestation_doc(public_key, user_data, nonce)
        .expect("Cannot get attestation document");

    attestation_doc
}

#[post("/process", data = "<req>")]
fn process(req: Json<ProcessReq>, encryption: &State<Encryption>) -> String {
    let encrypted_payload = req.encrypted_payload.to_owned();

    encryption.decrypt(encrypted_payload)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Encryption::new())
        .mount("/", routes![health_check])
        .mount("/", routes![get_attestation])
        .mount("/", routes![process])
}
