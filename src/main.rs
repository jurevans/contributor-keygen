use bip39::MnemonicType;
use types::{
    address::Address,
    mnemonic::{print_formatted_mnemonic, Mnemonic},
    wallet::HDWallet,
};
mod types;

fn main() {
    let path = "m/44'/877'/0'/0/0";

    println!("\nDeriving keys for BIP44 path: {} ...\n", &path);

    // TODO: Optionally prompt user for mnemonic, and skip
    // generation and verification if so
    let mnemonic = Mnemonic::new(MnemonicType::Words24);
    println!("Generated mnemonic:\n\n{}\n", mnemonic.to_phrase());

    print_formatted_mnemonic(mnemonic.to_words());

    let hd_wallet = HDWallet::new(&mnemonic.to_seed()).unwrap();
    let keypair = hd_wallet.derive(String::from(path)).unwrap();

    println!("\nPublic key: 00{}\n", keypair.public.to_hex());

    let address = Address::new(keypair.private.to_hex());

    println!("Address: {}\n", address.implicit());
}
