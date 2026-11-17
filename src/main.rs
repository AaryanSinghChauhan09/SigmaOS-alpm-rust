//! SigmaOS-alpm-rust — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_alpm_rust_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-alpm-rust: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-alpm-rust: ok");
}
