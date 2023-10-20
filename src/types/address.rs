use namada::types::{
    address,
    key::{
        self,
        common::{PublicKey, SecretKey},
        RefTo,
    },
};
use std::str::FromStr;

pub struct Address {
    implicit: address::Address,
}

impl Address {
    pub fn new(secret: String) -> Address {
        let private = SecretKey::Ed25519(
            key::ed25519::SecretKey::from_str(&secret).expect("ed25519 encoding should not fail"),
        );

        let public = PublicKey::from(private.ref_to());
        let implicit = address::Address::Implicit(address::ImplicitAddress::from(&public));

        Address { implicit }
    }

    pub fn implicit(&self) -> String {
        self.implicit.encode()
    }
}
