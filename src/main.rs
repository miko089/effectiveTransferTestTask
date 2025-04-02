#[cfg(feature = "sync")]
mod sync;

use sha2::{Sha256, Digest};

fn main() {
    #[cfg(feature = "sync")]
    {
        let m = sync::run();
        if let Err(e) = m {
            eprintln!("Error: {}", e);
            return;
        }
        let mut hasher = Sha256::new();
        hasher.update(&m.unwrap());
        let result = hasher.finalize();
        println!("{:x}", result);
    }
}
