fn main() {
    // Pass REFRONT_ENV to the Rust compiler if set
    if let Ok(env) = std::env::var("REFRONT_ENV") {
        println!("cargo:rustc-env=REFRONT_ENV={}", env);
        println!("cargo:warning=Building with REFRONT_ENV={}", env);
    } else {
        println!("cargo:warning=REFRONT_ENV not set, defaulting to development");
    }
    
    tauri_build::build()
}
