use std::io::Result;

fn main() -> Result<()> {
    let mut prost_build = prost_build::Config::new();
    prost_build.out_dir("src/pb"); // Specify the correct output directory
    // Compile main.proto instead of raydium_amm.proto
    prost_build.compile_protos(&["proto/main.proto"], &["proto/"])?;
    // Update rerun-if-changed condition
    println!("cargo:rerun-if-changed=proto/main.proto");
    Ok(())
} 