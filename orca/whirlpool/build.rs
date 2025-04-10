fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/main.proto"); // Ensure build runs if proto changes
    
    // Configure prost-build
    let mut config = prost_build::Config::new();

    // Add Default derive to all generated structs
    // config.type_attribute(".", "#[derive(Default)]"); 
    
    // config.type_attribute(".", "#[derive(BorshDeserialize)]"); // Keep commented out
    
    config.out_dir("src/pb") // Specify the output directory for generated Rust files
        .compile_protos(&["proto/main.proto"], &["proto/"])?; // Specify proto files and include paths

    Ok(())
} 