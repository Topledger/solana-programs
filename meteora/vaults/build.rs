// build.rs

use anchor_idl::{GeneratorOptions, Idl, IdlAccountItem};
use anyhow::Result;
use heck::{ ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use serde_json::from_str;
use sha2::{Digest, Sha256};
use std::{fs, process::Command};

fn main() -> Result<()> {
    // Force re-run check
    // --- Start: Protobuf Compilation --- //
    // Compile the proto file first
    println!("cargo:rerun-if-changed=proto/main.proto"); // Rerun if proto changes
    
    // Configure prost_build to output to src/pb
    let mut prost_build_config = prost_build::Config::new();
    prost_build_config.out_dir("src/pb"); // Output generated Rust code to src/pb

    // Create the src/pb directory if it doesn't exist
    fs::create_dir_all("src/pb")?;

    // Compile protos
    prost_build_config.compile_protos(&["proto/main.proto"], &["proto/"])?;
    println!("Successfully compiled protos to src/pb."); // Log success
    // --- End: Protobuf Compilation --- //

    // --- Start: Anchor IDL Processing --- //
    let idl_path = "idls/vaults.json";
    println!("cargo:rerun-if-changed={}", idl_path); // Rerun if IDL changes

    // Check if IDL exists
    if !std::path::Path::new(idl_path).exists() {
        eprintln!("Warning: IDL file not found at {}, skipping IDL processing.", idl_path);
        // Still return Ok because proto compilation might have succeeded
        return Ok(()); 
    }
    println!("Processing IDL: {}", idl_path);

    // 1) load the IDL JSON
    let idl_json = fs::read_to_string(idl_path)?;
    let idl: Idl = match from_str(&idl_json) {
        Ok(idl) => idl,
        Err(e) => {
            eprintln!("Error parsing IDL JSON from {}: {}", idl_path, e);
            // Return error if IDL parsing fails
            return Err(e.into()); 
        }
    };

    // 2) generate typedefs
    let mut opts = GeneratorOptions::default();
    opts.idl_path = idl_path.into();
    let gen = opts.to_generator();
    let typedefs_ts = anchor_idl::generate_typedefs(&gen.idl.types, &gen.struct_opts);

    // 3) build each Args and Accounts struct
    let mut args_structs = Vec::new();
    let mut accounts_structs = Vec::new();
    let mut arms = Vec::new();

    for ix in &idl.instructions {
        let name = &ix.name;
        let pascal = name.to_upper_camel_case();
        let args_ty = format_ident!("{}Args", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        let var = format_ident!("{}", pascal);

        // 3a) compute Anchor discriminator
        let mut hasher = Sha256::new();
        hasher.update(b"global:");
        hasher.update(name.to_snake_case().as_bytes()); // Use as_bytes()
        let hash = hasher.finalize();
        let disc_bytes: [u8; 8] = hash[0..8].try_into().unwrap();
        let disc_tokens = disc_bytes.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

        // 3b) build the Args struct fields
        let fields = ix.args.iter().map(|arg| {
            let f = format_ident!("{}", &arg.name.to_snake_case());
            let ty_ts = map_idl_type(&arg.ty);
            quote! { pub #f: #ty_ts, }
        });

        args_structs.push(quote! {
            // Derive AnchorDeserialize for args structs
            #[derive(anchor_lang::AnchorDeserialize, Debug)]
            pub struct #args_ty {
                #(#fields)*
            }
        });

        let flat_accounts = flatten_accounts(&ix.accounts);

        // build Accounts struct
        let acc_fields = flat_accounts.iter().map(|acc| {
            let f = format_ident!("{}", &acc.name);
            quote! { pub #f: anchor_lang::prelude::Pubkey, }
        });

        accounts_structs.push(quote! {
            #[derive(Debug)]
            pub struct #accounts_ty {
                #(#acc_fields)*
                pub remaining: Vec<anchor_lang::prelude::Pubkey>,
            }
        });

         // build match arm for decode_with_accounts
         let acc_idents: Vec<_> = flat_accounts.iter().map(|acc| format_ident!("{}", &acc.name)).collect();
         let extract_accounts = acc_idents.iter().map(|ident| {
             quote! {
                 let #ident = *keys.next()
                     .ok_or_else(|| anyhow::anyhow!(concat!("Missing account: ", stringify!(#ident))))?;
             }
         });

         arms.push(quote! {
            [#(#disc_tokens),*] => {
                // Use AnchorDeserialize, which reads directly from the slice `rest`
                let mut data_slice = rest;
                let args = #args_ty::deserialize(&mut data_slice)?;
                // consume accounts in order
                let mut keys = account_keys.iter();
                #(#extract_accounts)*
                let remaining = keys.cloned().collect();
                let accounts = #accounts_ty { #(#acc_idents),*, remaining };
                // Return the variant with populated accounts and args
                return Ok(Instruction::#var { accounts, args });
            }
        });
    }

    // 4) build the Instruction enum (no derives)
    let variants = idl.instructions.iter().map(|ix| {
        let name = &ix.name;
        let pascal = name.to_upper_camel_case();
        let var = format_ident!("{}", pascal);
        let args_ty = format_ident!("{}Args", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        quote! { #var { accounts: #accounts_ty, args: #args_ty }, }
    });
    let decoded_enum = quote! {
        #[derive(Debug)]
        pub enum Instruction {
            #(#variants)*
        }
    };

    // 5) implement decode()
    let decode_impl = quote! {
        impl Instruction {
            pub fn decode(
                account_keys: &[anchor_lang::prelude::Pubkey],
                data: &[u8],
            ) -> anyhow::Result<Self> {
                if data.len() < 8 {
                    anyhow::bail!("Data too short for discriminator: {}", data.len());
                }
                let (disc_slice, rest) = data.split_at(8);
                let disc: [u8; 8] = disc_slice.try_into().unwrap();
                match disc {
                    #(#arms)*
                    _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
                }
            }
        }
    };

    // 6) glue & write
    let out = quote! {
        // @generated by build.rs — DO NOT EDIT
        #![allow(dead_code)]
        #![allow(unused_imports)]

        // Add AnchorDeserialize to uses
        use anchor_lang::{self, AnchorDeserialize};
        use anyhow;
        use borsh::{BorshSchema, BorshSerialize}; // Keep BorshSerialize if needed elsewhere
        use std::convert::TryInto;
        use std::mem;

        // re-export
        pub use ix_data::*;
        pub use typedefs::*;
        pub use accounts_data::*;

        pub mod typedefs {
            use anchor_lang::prelude::*;
            #typedefs_ts
        }

        pub mod accounts_data {
            use anchor_lang::prelude::*;
            #(#accounts_structs)*
        }

        pub mod ix_data {
            use super::*;
            #(#args_structs)*
        }

        #decoded_enum
        #decode_impl
    };

    let out_path = "src/idl.rs";
    fs::write(&out_path, out.to_string())?;
    // Only run rustfmt if it's available
    if Command::new("rustfmt").arg(&out_path).status().is_err() {
         eprintln!("Warning: rustfmt not found or failed, skipping formatting of {}", out_path);
    }
    println!("Successfully generated {}", out_path);
    // --- End: Anchor IDL Processing --- //

    Ok(())
}

// helper to map IdlType → Rust TokenStream
fn map_idl_type(ty: &anchor_idl::IdlType) -> proc_macro2::TokenStream {
    use anchor_idl::IdlType::*;
    match ty {
        Bool => quote! { bool },
        U8 => quote! { u8 },
        I8 => quote! { i8 },
        U16 => quote! { u16 },
        I16 => quote! { i16 },
        U32 => quote! { u32 },
        I32 => quote! { i32 },
        U64 => quote! { u64 },
        I64 => quote! { i64 },
        U128 => quote! { u128 },
        I128 => quote! { i128 },
        Bytes => quote! { Vec<u8> },
        String => quote! { String },
        PublicKey => quote! { anchor_lang::prelude::Pubkey },
        Vec(inner) => {
            let inner = map_idl_type(inner);
            quote! { Vec<#inner> }
        }
        Array(inner, len) => {
            let inner = map_idl_type(inner);
            let l = *len;
            quote! { [#inner; #l] }
        }
        Defined(n) => {
            let ident = format_ident!("{}", n);
            quote! { typedefs::#ident } // Qualify with typedefs module
        }
        Option(inner) => {
            let inner = map_idl_type(inner);
            quote! { Option<#inner> }
        }
        _ => panic!("unsupported IDL type: {:?}", ty),
    }
}

fn flatten_accounts<'a>(items: &'a [IdlAccountItem]) -> Vec<&'a anchor_idl::IdlAccount> {
    let mut out = Vec::new();
    for item in items {
        match item {
            IdlAccountItem::IdlAccount(acc) => out.push(acc),
            IdlAccountItem::IdlAccounts(group) => {
                out.extend(flatten_accounts(&group.accounts));
            }
        }
    }
    out
}