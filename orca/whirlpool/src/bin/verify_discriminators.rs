use hex;
use std::collections::HashMap;
use std::env;
use substreams_solana_orca_whirlpool::instructions;

fn print_usage() {
    println!("Usage:");
    println!("  verify_discriminators [OPTIONS]");
    println!();
    println!("Options:");
    println!("  verify                  Verify all discriminators (default)");
    println!("  generate-all            Generate code for all discriminators");
    println!("  generate [INSTRUCTION]  Generate code for a specific instruction");
    println!("  help                    Print this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Default action is verify
    let command = if args.len() > 1 { &args[1] } else { "verify" };
    
    match command {
        "verify" => verify_discriminators(),
        "generate-all" => generate_all_discriminators(),
        "generate" => {
            if args.len() < 3 {
                println!("Error: Missing instruction name");
                print_usage();
                return;
            }
            generate_discriminator(&args[2]);
        },
        "help" => print_usage(),
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
        }
    }
}

fn verify_discriminators() {
    println!("Verifying Orca Whirlpool instruction discriminators...\n");
    
    let mut mismatches = HashMap::new();
    let mut matches = 0;
    
    // Check each instruction type in INSTRUCTION_TYPES
    for (stored_disc, inst_name) in instructions::INSTRUCTION_TYPES.iter() {
        let computed_disc = instructions::compute_discriminator(inst_name);
        
        if stored_disc != &computed_disc {
            mismatches.insert(
                inst_name.to_string(), 
                (hex::encode(stored_disc), hex::encode(computed_disc))
            );
        } else {
            matches += 1;
        }
    }
    
    // Print results
    println!("Results:");
    println!("- Total instructions checked: {}", instructions::INSTRUCTION_TYPES.len());
    println!("- Matching discriminators: {}", matches);
    println!("- Mismatched discriminators: {}", mismatches.len());
    
    if !mismatches.is_empty() {
        println!("\nDiscriminator mismatches found:");
        println!("{:<30} {:<18} {:<18}", "Instruction", "Stored", "Computed");
        println!("{}", "-".repeat(68));
        
        for (inst_name, (stored, computed)) in mismatches.iter() {
            println!("{:<30} {:<18} {:<18}", inst_name, stored, computed);
        }
    } else {
        println!("\nAll discriminators match! 🎉");
    }
}

fn generate_all_discriminators() {
    println!("Generating code for all discriminators...\n");
    
    let instruction_names: Vec<&str> = instructions::INSTRUCTION_TYPES
        .iter()
        .map(|(_, name)| *name)
        .collect();
        
    let code = instructions::generate_discriminator_code(&instruction_names);
    
    println!("pub const INSTRUCTION_TYPES: [(&[u8], &str); {}] = [\n{}", 
        instruction_names.len(), code);
}

fn generate_discriminator(instruction_name: &str) {
    println!("Generating discriminator for: {}\n", instruction_name);
    
    let code = instructions::generate_discriminator_code(&[instruction_name]);
    
    println!("{}", code);
} 