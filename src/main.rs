#[cfg(feature = "sync")]
mod sync;

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
use tokio;

use sha2::{Digest, Sha256};

const HOST: &str = "127.0.0.1";
const PORT: &str = "8080";

#[cfg(all(feature = "sync", feature = "async"))]
compile_error!("Features async and sync are mutually exclusive. Please enable only one of them.");

#[cfg(not(any(feature = "sync", feature = "async")))]
compile_error!("Enable either sync or async feature in Cargo.toml");

#[cfg(feature = "sync")]
fn main() {
    let m = sync::run(HOST, PORT);
    if let Err(e) = m {
        eprintln!("Error: {}", e);
        return;
    }
    println!("Length: {}", m.as_ref().unwrap().len());
    let mut hasher = Sha256::new();
    hasher.update(&m.unwrap());
    let result = hasher.finalize();
    println!("{:x}", result);
}

#[cfg(feature = "async")]
#[tokio::main]
async fn main() {
    let m = r#async::run(HOST, PORT).await;
    if let Err(e) = m {
        eprintln!("Error: {}", e);
        return;
    }
    println!("Length: {}", m.as_ref().unwrap().len());
    let mut hasher = Sha256::new();
    hasher.update(&m.unwrap());
    let result = hasher.finalize();
    println!("{:x}", result);
}
