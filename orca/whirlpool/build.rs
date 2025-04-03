fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/main.proto"); // Ensure build runs if proto changes
    
    // Compile the protobuf file using prost-build
    prost_build::Config::new()
        .out_dir("src/pb") // Specify the output directory for generated Rust files
        .compile_protos(&["proto/main.proto"], &["proto/"])?; // Specify proto files and include paths

    Ok(())
} 