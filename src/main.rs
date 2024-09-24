use clap::Parser;
use regex::Regex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::types::crypto::get_key_pair_from_rng;
use sui_sdk::types::crypto::SuiKeyPair;

pub const SUI_PRIV_KEY_PREFIX: &str = "suiprivkey";

pub fn generate_keypair() -> SuiKeyPair {
    // Generate a new key pair using the provided function
    SuiKeyPair::Ed25519(get_key_pair_from_rng(&mut rand::rngs::OsRng).1)
}

// Command-line arguments structure
#[derive(Parser)]
struct Cli {
    /// Desired prefix for the Sui address, remember to add "0x" in the input
    #[clap(short, long)]
    prefix: String,

    /// Number of threads to use
    #[clap(short, long, default_value_t = 8)]
    threads: usize,
}


#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let desired_prefix = &args.prefix;
    let n_threads = args.threads;
    let pattern = format!(r"^{}", regex::escape(desired_prefix));
    let re = Regex::new(&pattern).unwrap();
    // Create a thread pool using rayon for parallelism
    let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(n_threads).build().unwrap();
    let attempt_count = Arc::new(AtomicUsize::new(0));

    thread_pool.scope(|s| {
        (0..n_threads).for_each(|_| {
            let re = re.clone();
            let value = attempt_count.clone();
            s.spawn(move |_| {
                loop {
                    let keypair = generate_keypair();
                    // Convert the Sui address to a string
                    let address_str = SuiAddress::from(&keypair.public()).to_string();
                    // Check if the address matches the desired prefix
                    if re.is_match(&address_str) {
                        println!("Found matching Sui address: {}", address_str);
                        println!("Private Key: {:?}", keypair.encode());
                    }
                    // Increment the global attempt count
                    let current_attempts = value.fetch_add(1, Ordering::Relaxed);
                    if current_attempts % 10000000 == 0 {
                        println!("{} {:?} Attempts: {}", chrono::Utc::now(), std::thread::current().id(), current_attempts);
                    }
                }
            });
        });
    });
}