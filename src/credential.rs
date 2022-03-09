use std::error::Error;
use keyring;

pub fn add_credential(url: &str, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let entry = keyring::Entry::new(url, username);
    entry.set_password(password).map_err(|e| { e.into() })
}
