use bindgen_macro::bindgen;
use primitives::env;

#[bindgen]
pub fn invoke() {
    env::log("hello"); // prints `hello` on the VM

    // computes the sha256 hash on the VM and returns the digest to the client
    let hash = env::sha256("message".as_bytes());
    let hash = std::str::from_utf8(&hash).unwrap();
    env::log(format!("hash: {:?}", hash)); // prints the raw sha256 bytes on the VM
}
