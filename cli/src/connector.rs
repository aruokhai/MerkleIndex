use std::io::Error;
use subxt_signer::sr25519::dev::{self};
use crate::healthindex::*;

/// Show the configuration file
pub fn register_provider(url: String, user: String, client: OnlineClient::<PolkadotConfig>) -> Result<(), Box<dyn std::error::Error>> {
    let provider = match user.as_str() {
        "alice" => dev::alice(),
        "bob" => dev::bob().into(),
        "charlie" => dev::charlie(),
        "dave" => dev::dave(),
        "fredie" => dev::ferdie(),
        "eve" => dev::eve(),
        _ => dev::one(),
    };
    let txn = client.storage().;
    println!("Connection with parachain established.");
    println!("You got it right!");

    Ok(())
}
