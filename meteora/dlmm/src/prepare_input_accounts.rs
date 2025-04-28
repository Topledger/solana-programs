// This file will contain account preparation logic for Meteora DLMM
// It should be implemented based on Meteora's DLMM program structure

use std::collections::HashMap;
use crate::instructions::InstructionType;

// Struct to hold input accounts mapping results
pub struct InputAccounts {
    pub accounts: Vec<String>,
    pub input_accounts: HashMap<String, String>,
}

// Maps account indexes to their corresponding roles based on the instruction type
pub fn map_accounts(
    account_indices: &[u8],
    instruction_type: InstructionType,
) -> InputAccounts {
    let mut role_by_index = HashMap::new();
    
    // Create a mapping of indices to account roles based on instruction type
    match instruction_type {
        InstructionType::Swap | InstructionType::SwapWithPriceImpact | InstructionType::SwapExactOut => {
            // Match V1 IDL definition (15 accounts)
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 2, "reserveX");
            assign_if_exists(&mut role_by_index, 3, "reserveY");
            assign_if_exists(&mut role_by_index, 4, "userTokenIn");
            assign_if_exists(&mut role_by_index, 5, "userTokenOut");
            assign_if_exists(&mut role_by_index, 6, "tokenXMint");
            assign_if_exists(&mut role_by_index, 7, "tokenYMint");
            assign_if_exists(&mut role_by_index, 8, "oracle");
            assign_if_exists(&mut role_by_index, 9, "hostFeeIn");
            assign_if_exists(&mut role_by_index, 10, "user");
            assign_if_exists(&mut role_by_index, 11, "tokenXProgram");
            assign_if_exists(&mut role_by_index, 12, "tokenYProgram");
            assign_if_exists(&mut role_by_index, 13, "eventAuthority");
            assign_if_exists(&mut role_by_index, 14, "program");
        },
        
        // Core pool operations
        InstructionType::InitializeLbPair => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 2, "tokenMintX");
            assign_if_exists(&mut role_by_index, 3, "tokenMintY");
            assign_if_exists(&mut role_by_index, 4, "reserveX");
            assign_if_exists(&mut role_by_index, 5, "reserveY");
            assign_if_exists(&mut role_by_index, 6, "oracle");
            assign_if_exists(&mut role_by_index, 7, "presetParameter");
            assign_if_exists(&mut role_by_index, 8, "funder");
            assign_if_exists(&mut role_by_index, 9, "tokenProgram");
            assign_if_exists(&mut role_by_index, 10, "systemProgram");
            assign_if_exists(&mut role_by_index, 11, "rent");
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },
        
        InstructionType::InitializePermissionLbPair => {
            // Mapping based on user-provided example
            assign_if_exists(&mut role_by_index, 0, "base");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 3, "tokenMintX");
            assign_if_exists(&mut role_by_index, 4, "tokenMintY");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "oracle");
            assign_if_exists(&mut role_by_index, 8, "admin");
            assign_if_exists(&mut role_by_index, 9, "tokenProgram");
            assign_if_exists(&mut role_by_index, 10, "systemProgram");
            assign_if_exists(&mut role_by_index, 11, "rent");
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },
        
        // Liquidity operations
        InstructionType::AddLiquidity | 
        InstructionType::AddLiquidityByWeight | 
        InstructionType::AddLiquidityByStrategy => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 3, "userTokenX");
            assign_if_exists(&mut role_by_index, 4, "userTokenY");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "tokenXMint");
            assign_if_exists(&mut role_by_index, 8, "tokenYMint");
            assign_if_exists(&mut role_by_index, 9, "binArrayLower");
            assign_if_exists(&mut role_by_index, 10, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 11, "sender");
            assign_if_exists(&mut role_by_index, 12, "tokenXProgram");
            assign_if_exists(&mut role_by_index, 13, "tokenYProgram");
            assign_if_exists(&mut role_by_index, 14, "eventAuthority");
            assign_if_exists(&mut role_by_index, 15, "program");
        },
        
        InstructionType::AddLiquidityOneSide | 
        InstructionType::AddLiquidityByStrategyOneSide | 
        InstructionType::AddLiquidityOneSidePrecise => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 3, "userToken");
            assign_if_exists(&mut role_by_index, 4, "reserve");
            assign_if_exists(&mut role_by_index, 5, "tokenMint");
            assign_if_exists(&mut role_by_index, 6, "binArrayLower");
            assign_if_exists(&mut role_by_index, 7, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 8, "sender");
            assign_if_exists(&mut role_by_index, 9, "tokenProgram");
            assign_if_exists(&mut role_by_index, 10, "eventAuthority");
            assign_if_exists(&mut role_by_index, 11, "program");
        },
        
        InstructionType::RemoveLiquidity | 
        InstructionType::RemoveAllLiquidity | 
        InstructionType::RemoveLiquidityByRange => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 3, "userTokenX");
            assign_if_exists(&mut role_by_index, 4, "userTokenY");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "tokenXMint");
            assign_if_exists(&mut role_by_index, 8, "tokenYMint");
            assign_if_exists(&mut role_by_index, 9, "binArrayLower");
            assign_if_exists(&mut role_by_index, 10, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 11, "sender");
            assign_if_exists(&mut role_by_index, 12, "tokenXProgram");
            assign_if_exists(&mut role_by_index, 13, "tokenYProgram");
            assign_if_exists(&mut role_by_index, 14, "eventAuthority");
            assign_if_exists(&mut role_by_index, 15, "program");
        },
        
        // Position management
        InstructionType::InitializePosition => {
            assign_if_exists(&mut role_by_index, 0, "payer");
            assign_if_exists(&mut role_by_index, 2, "position");
            assign_if_exists(&mut role_by_index, 3, "lbPair");
            assign_if_exists(&mut role_by_index, 4, "owner");
            assign_if_exists(&mut role_by_index, 5, "systemProgram");
            assign_if_exists(&mut role_by_index, 6, "rent");
            assign_if_exists(&mut role_by_index, 7, "eventAuthority");
            assign_if_exists(&mut role_by_index, 8, "program");
        },
    
        InstructionType::InitializePositionPda => {
            assign_if_exists(&mut role_by_index, 0, "payer");
            assign_if_exists(&mut role_by_index, 1, "base");
            assign_if_exists(&mut role_by_index, 2, "position");
            assign_if_exists(&mut role_by_index, 3, "lbPair");
            assign_if_exists(&mut role_by_index, 4, "owner");
            assign_if_exists(&mut role_by_index, 5, "systemProgram");
            assign_if_exists(&mut role_by_index, 6, "rent");
            assign_if_exists(&mut role_by_index, 7, "eventAuthority");
            assign_if_exists(&mut role_by_index, 8, "program");
        },
        
        InstructionType::ClosePosition => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayLower");
            assign_if_exists(&mut role_by_index, 3, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 4, "sender");
            assign_if_exists(&mut role_by_index, 5, "rentReceiver");
            assign_if_exists(&mut role_by_index, 6, "eventAuthority");
            assign_if_exists(&mut role_by_index, 7, "program");
        },
        
        // Fee operations
        InstructionType::ClaimFee => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "position");
            assign_if_exists(&mut role_by_index, 2, "binArrayLower");
            assign_if_exists(&mut role_by_index, 3, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 4, "sender");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "userTokenX");
            assign_if_exists(&mut role_by_index, 8, "userTokenY");
            assign_if_exists(&mut role_by_index, 9, "tokenXMint");
            assign_if_exists(&mut role_by_index, 10, "tokenYMint");
            assign_if_exists(&mut role_by_index, 11, "tokenProgram");
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },
        
        // Reward operations
        InstructionType::ClaimReward => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "position");
            assign_if_exists(&mut role_by_index, 2, "binArrayLower");
            assign_if_exists(&mut role_by_index, 3, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 4, "sender");
            assign_if_exists(&mut role_by_index, 5, "rewardVault");
            assign_if_exists(&mut role_by_index, 6, "rewardMint");
            assign_if_exists(&mut role_by_index, 7, "userTokenAccount");
            assign_if_exists(&mut role_by_index, 8, "tokenProgram");
            assign_if_exists(&mut role_by_index, 9, "eventAuthority");
            assign_if_exists(&mut role_by_index, 10, "program");
        },

        InstructionType::FundReward => {
            // Updated mapping based on provided IDL
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "rewardVault");
            assign_if_exists(&mut role_by_index, 2, "rewardMint");
            assign_if_exists(&mut role_by_index, 3, "funderTokenAccount");
            assign_if_exists(&mut role_by_index, 4, "funder");
            assign_if_exists(&mut role_by_index, 5, "binArray"); // Added based on IDL
            assign_if_exists(&mut role_by_index, 6, "tokenProgram");
            assign_if_exists(&mut role_by_index, 7, "eventAuthority");
            assign_if_exists(&mut role_by_index, 8, "program");
        },
        
        // V2 Swap variants share similar base accounts
        InstructionType::Swap2 | InstructionType::SwapExactOut2 | InstructionType::SwapWithPriceImpact2 => {
             // Match V2 IDL definition (16 base accounts)
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension"); // Added based on IDL
            assign_if_exists(&mut role_by_index, 2, "reserveX");  // Index adjusted
            assign_if_exists(&mut role_by_index, 3, "reserveY");  // Index adjusted
            assign_if_exists(&mut role_by_index, 4, "userTokenIn"); // Index adjusted
            assign_if_exists(&mut role_by_index, 5, "userTokenOut"); // Index adjusted
            assign_if_exists(&mut role_by_index, 6, "tokenXMint"); // Index adjusted
            assign_if_exists(&mut role_by_index, 7, "tokenYMint"); // Index adjusted
            assign_if_exists(&mut role_by_index, 8, "oracle"); // Index adjusted
            assign_if_exists(&mut role_by_index, 9, "hostFeeIn"); // Index adjusted
            assign_if_exists(&mut role_by_index, 10, "user"); // Index adjusted, renamed from owner
            assign_if_exists(&mut role_by_index, 11, "tokenXProgram"); // Index adjusted
            assign_if_exists(&mut role_by_index, 12, "tokenYProgram"); // Added based on IDL
            assign_if_exists(&mut role_by_index, 13, "memoProgram"); // Added based on IDL
            assign_if_exists(&mut role_by_index, 14, "eventAuthority"); // Index adjusted
            assign_if_exists(&mut role_by_index, 15, "program"); // Index adjusted
            // Additional accounts beyond index 15 are handled dynamically via RemainingAccountsInfo argument
        },

        InstructionType::RemoveLiquidityByRange2 => {
            // Mapping based on the V2 IDL (15 accounts)
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension"); // Optional
            assign_if_exists(&mut role_by_index, 3, "userTokenX");
            assign_if_exists(&mut role_by_index, 4, "userTokenY");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "tokenXMint");
            assign_if_exists(&mut role_by_index, 8, "tokenYMint");
            assign_if_exists(&mut role_by_index, 9, "sender"); // Signer
            assign_if_exists(&mut role_by_index, 10, "tokenXProgram");
            assign_if_exists(&mut role_by_index, 11, "tokenYProgram");
            assign_if_exists(&mut role_by_index, 12, "memoProgram");
            assign_if_exists(&mut role_by_index, 13, "eventAuthority");
            assign_if_exists(&mut role_by_index, 14, "program");
            // Remaining accounts handled by the instruction arguments (RemainingAccountsInfo)
        },
        
        // Add Mappings for other V2 Instructions

        InstructionType::InitializeCustomizablePermissionlessLbPair => {
            // Based on V1 IDL
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension"); // Optional
            assign_if_exists(&mut role_by_index, 2, "tokenMintX");
            assign_if_exists(&mut role_by_index, 3, "tokenMintY");
            assign_if_exists(&mut role_by_index, 4, "reserveX");
            assign_if_exists(&mut role_by_index, 5, "reserveY");
            assign_if_exists(&mut role_by_index, 6, "oracle");
            assign_if_exists(&mut role_by_index, 7, "userTokenX"); // User token X
            assign_if_exists(&mut role_by_index, 8, "funder"); // Signer
            assign_if_exists(&mut role_by_index, 9, "tokenProgram");
            assign_if_exists(&mut role_by_index, 10, "systemProgram");
            assign_if_exists(&mut role_by_index, 11, "userTokenY"); // User token Y
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },

        InstructionType::InitializeLbPair2 => {
            // Similar to V1 InitializeLbPair
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 2, "tokenMintX");
            assign_if_exists(&mut role_by_index, 3, "tokenMintY");
            assign_if_exists(&mut role_by_index, 4, "reserveX");
            assign_if_exists(&mut role_by_index, 5, "reserveY");
            assign_if_exists(&mut role_by_index, 6, "oracle");
            assign_if_exists(&mut role_by_index, 7, "presetParameter");
            assign_if_exists(&mut role_by_index, 8, "funder"); 
            assign_if_exists(&mut role_by_index, 9, "tokenBadgeX");
            assign_if_exists(&mut role_by_index, 10, "tokenBadgeY");
            assign_if_exists(&mut role_by_index, 11, "tokenProgramX"); 
            assign_if_exists(&mut role_by_index, 12, "tokenProgramY"); 
            assign_if_exists(&mut role_by_index, 13, "systemProgram");
            assign_if_exists(&mut role_by_index, 14, "eventAuthority");
            assign_if_exists(&mut role_by_index, 15, "program");
        },

        InstructionType::InitializeCustomizablePermissionlessLbPair2 => {
            // Based on V2 IDL - Corrected
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension"); // Optional
            assign_if_exists(&mut role_by_index, 2, "tokenMintX");
            assign_if_exists(&mut role_by_index, 3, "tokenMintY");
            assign_if_exists(&mut role_by_index, 4, "reserveX");
            assign_if_exists(&mut role_by_index, 5, "reserveY");
            assign_if_exists(&mut role_by_index, 6, "oracle");
            assign_if_exists(&mut role_by_index, 7, "userTokenX"); // User token X
            assign_if_exists(&mut role_by_index, 8, "funder"); // Signer
            assign_if_exists(&mut role_by_index, 9, "tokenBadgeX"); // Optional
            assign_if_exists(&mut role_by_index, 10, "tokenBadgeY"); // Optional
            assign_if_exists(&mut role_by_index, 11, "tokenProgramX"); // V2 split token programs
            assign_if_exists(&mut role_by_index, 12, "tokenProgramY"); // V2 split token programs
            assign_if_exists(&mut role_by_index, 13, "systemProgram");
            assign_if_exists(&mut role_by_index, 14, "userTokenY"); // User token Y
            assign_if_exists(&mut role_by_index, 15, "eventAuthority");
            assign_if_exists(&mut role_by_index, 16, "program");
        },

        InstructionType::ClaimFee2 => {
            // Similar to V1 ClaimFee
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "position");
            assign_if_exists(&mut role_by_index, 2, "sender");
            assign_if_exists(&mut role_by_index, 3, "reserveX");
            assign_if_exists(&mut role_by_index, 4, "reserveY");
            assign_if_exists(&mut role_by_index, 5, "userTokenX");
            assign_if_exists(&mut role_by_index, 6, "userTokenY");
            assign_if_exists(&mut role_by_index, 7, "tokenXMint");
            assign_if_exists(&mut role_by_index, 8, "tokenYMint");
            assign_if_exists(&mut role_by_index, 9, "tokenProgramX");
            assign_if_exists(&mut role_by_index, 10, "tokenProgramY");
            assign_if_exists(&mut role_by_index, 11, "memoProgram");
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },

        InstructionType::ClaimReward2 => {
            // Similar to V1 ClaimReward
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "position");
            assign_if_exists(&mut role_by_index, 2, "sender");
            assign_if_exists(&mut role_by_index, 3, "rewardVault");
            assign_if_exists(&mut role_by_index, 4, "rewardMint");
            assign_if_exists(&mut role_by_index, 5, "userTokenAccount");
            assign_if_exists(&mut role_by_index, 6, "tokenProgram");
            assign_if_exists(&mut role_by_index, 7, "memoProgram");
            // Token program likely passed via RemainingAccountsInfo for transfer hooks
            assign_if_exists(&mut role_by_index, 8, "eventAuthority");
            assign_if_exists(&mut role_by_index, 9, "program");
        },

        InstructionType::AddLiquidity2 | 
        InstructionType::AddLiquidityByStrategy2 => { 
            // Based on V2 IDL (14 accounts)
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 3, "userTokenX");
            assign_if_exists(&mut role_by_index, 4, "userTokenY");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "tokenXMint");
            assign_if_exists(&mut role_by_index, 8, "tokenYMint");
            assign_if_exists(&mut role_by_index, 9, "sender");
            assign_if_exists(&mut role_by_index, 10, "tokenXProgram");
            assign_if_exists(&mut role_by_index, 11, "tokenYProgram");
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },

        InstructionType::AddLiquidityOneSidePrecise2 => {
            // Based on V2 IDL (10 accounts)
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 3, "userToken");
            assign_if_exists(&mut role_by_index, 4, "reserve");
            assign_if_exists(&mut role_by_index, 5, "tokenMint");
            assign_if_exists(&mut role_by_index, 6, "sender");
            assign_if_exists(&mut role_by_index, 7, "tokenProgram");
            assign_if_exists(&mut role_by_index, 8, "eventAuthority");
            assign_if_exists(&mut role_by_index, 9, "program");
        },

        InstructionType::RemoveLiquidityByRange2 => {
            // Similar to V1 RemoveLiquidity variants
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension"); // If applicable
            assign_if_exists(&mut role_by_index, 3, "userTokenX");
            assign_if_exists(&mut role_by_index, 4, "userTokenY");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "tokenXMint");
            assign_if_exists(&mut role_by_index, 8, "tokenYMint");
            assign_if_exists(&mut role_by_index, 9, "binArrayLower");
            assign_if_exists(&mut role_by_index, 10, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 11, "sender"); // Or owner
            // Token programs likely passed via RemainingAccountsInfo for transfer hooks
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },

        InstructionType::RemoveLiquidity2 => {
            // Similar to V1 RemoveLiquidity variants
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension"); // If applicable
            assign_if_exists(&mut role_by_index, 3, "userTokenX");
            assign_if_exists(&mut role_by_index, 4, "userTokenY");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "tokenXMint");
            assign_if_exists(&mut role_by_index, 8, "tokenYMint");
            assign_if_exists(&mut role_by_index, 9, "sender");
            assign_if_exists(&mut role_by_index, 10, "tokenXProgram");
            assign_if_exists(&mut role_by_index, 11, "tokenYProgram"); 
            assign_if_exists(&mut role_by_index, 12, "memoProgram"); 
            assign_if_exists(&mut role_by_index, 13, "eventAuthority");
            assign_if_exists(&mut role_by_index, 14, "program");
        },

        InstructionType::ClosePosition2 => {
            // Similar to V1 ClosePosition
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "sender");
            assign_if_exists(&mut role_by_index, 2, "rentReceiver");
            assign_if_exists(&mut role_by_index, 3, "eventAuthority");
            assign_if_exists(&mut role_by_index, 4, "program");
        },

        // UpdateFeesAndReward2 is handled by UpdateFeesAndRewards mapping

        InstructionType::ClosePositionIfEmpty => {
            // Guessing accounts based on name
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "sender");
            assign_if_exists(&mut role_by_index, 2, "rentReceiver");
            assign_if_exists(&mut role_by_index, 3, "eventAuthority");
            assign_if_exists(&mut role_by_index, 4, "program");
        },

        InstructionType::InitializeTokenBadge => {
            // Guessing accounts based on name
            assign_if_exists(&mut role_by_index, 0, "tokenBadge"); // The account being initialized
            assign_if_exists(&mut role_by_index, 1, "lbPair");     // Likely associated with a pair
            assign_if_exists(&mut role_by_index, 2, "payer");      // Who pays for init
            assign_if_exists(&mut role_by_index, 3, "systemProgram");
            assign_if_exists(&mut role_by_index, 4, "rent");
            assign_if_exists(&mut role_by_index, 5, "eventAuthority");
            assign_if_exists(&mut role_by_index, 6, "program");
        },
        InstructionType::CreateClaimProtocolFeeOperator => {
            // Guessing accounts
            assign_if_exists(&mut role_by_index, 0, "claimFeeOperator"); // Account being created
            assign_if_exists(&mut role_by_index, 1, "operator");         // Authority
            assign_if_exists(&mut role_by_index, 2, "admin");
            assign_if_exists(&mut role_by_index, 3, "systemProgram");
        },
        InstructionType::CloseClaimProtocolFeeOperator => {
            // Guessing accounts
            assign_if_exists(&mut role_by_index, 0, "claimFeeOperator"); // Account being closed
            assign_if_exists(&mut role_by_index, 1, "admin");         // Authority
            assign_if_exists(&mut role_by_index, 2, "rentReceiver");
        },

        // Event logs typically don't have associated accounts passed in the instruction
        // itself (they might reference accounts involved in the original action).
        // We return an empty mapping for EventLog.
        InstructionType::EventLog => {
            // No accounts associated with EventLog instruction itself
        },
        
        // Reward operations
        InstructionType::InitializeReward => {
            // Based on JSON: lbPair, rewardVault, rewardMint, tokenBadge(opt), admin, tokenProgram, systemProgram, rent, eventAuthority, program
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "rewardVault");
            assign_if_exists(&mut role_by_index, 2, "rewardMint");
            assign_if_exists(&mut role_by_index, 3, "admin");
            assign_if_exists(&mut role_by_index, 4, "tokenProgram");
            assign_if_exists(&mut role_by_index, 5, "systemProgram");
            assign_if_exists(&mut role_by_index, 6, "rent");
            assign_if_exists(&mut role_by_index, 7, "eventAuthority");
            assign_if_exists(&mut role_by_index, 8, "program");
        },
        
        InstructionType::InitializePresetParameter => {
            assign_if_exists(&mut role_by_index, 0, "presetParameter");
            assign_if_exists(&mut role_by_index, 1, "admin");
            assign_if_exists(&mut role_by_index, 2, "systemProgram");
            assign_if_exists(&mut role_by_index, 3, "rent");
        },
        
        InstructionType::InitializePresetParameter2 => {
            assign_if_exists(&mut role_by_index, 0, "presetParameter");
            assign_if_exists(&mut role_by_index, 1, "admin");
            assign_if_exists(&mut role_by_index, 2, "systemProgram");
        },
        
        InstructionType::ClosePresetParameter => {
            assign_if_exists(&mut role_by_index, 0, "presetParameter");
            assign_if_exists(&mut role_by_index, 1, "admin");
            assign_if_exists(&mut role_by_index, 2, "rentReceiver");
        },
        
        InstructionType::ClosePresetParameter2 => {
            assign_if_exists(&mut role_by_index, 0, "presetParameter");
            assign_if_exists(&mut role_by_index, 1, "admin");
            assign_if_exists(&mut role_by_index, 2, "rentReceiver");
        },
        
        InstructionType::TogglePairStatus => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "admin");
        },
        
        InstructionType::UpdateWhitelistedWallet => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "admin");
        },
        
        InstructionType::InitializeBinArrayBitmapExtension => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 2, "funder");
            assign_if_exists(&mut role_by_index, 3, "systemProgram");
            assign_if_exists(&mut role_by_index, 4, "rent");
        },

        InstructionType::InitializeBinArray => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArray");
            assign_if_exists(&mut role_by_index, 2, "funder");
            assign_if_exists(&mut role_by_index, 3, "systemProgram");
        },

        InstructionType::MigrateBinArray => {
            assign_if_exists(&mut role_by_index, 0, "binArray");
        },
        
        InstructionType::GoToABin => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 2, "fromBinArray");
            assign_if_exists(&mut role_by_index, 3, "toBinArray");
            assign_if_exists(&mut role_by_index, 4, "eventAuthority");
            assign_if_exists(&mut role_by_index, 5, "program");
        },
        
        InstructionType::UpdateFeesAndRewards => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayLower");
            assign_if_exists(&mut role_by_index, 3, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 4, "owner");
        },
        
        InstructionType::SetMaxSwappedAmount => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "admin");
        },

        InstructionType::SetPreActivationDuration => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "creator");
        },
        
        InstructionType::SetPreActivationSwapAddress => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "creator");
        },
        
        InstructionType::SetActivationPoint => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "admin");
        },
        
        InstructionType::SetLockReleaseSlot => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "sender");
            assign_if_exists(&mut role_by_index, 3, "eventAuthority");
            assign_if_exists(&mut role_by_index, 4, "program");
        },
        
        InstructionType::RemoveLiquiditySingleSide => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 3, "userToken");
            assign_if_exists(&mut role_by_index, 4, "reserve");
            assign_if_exists(&mut role_by_index, 5, "tokenMint");
            assign_if_exists(&mut role_by_index, 6, "binArrayLower");
            assign_if_exists(&mut role_by_index, 7, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 8, "sender");
            assign_if_exists(&mut role_by_index, 9, "tokenProgram");
            assign_if_exists(&mut role_by_index, 10, "eventAuthority");
            assign_if_exists(&mut role_by_index, 11, "program");
        },
        
        InstructionType::WithdrawProtocolFee => {
            // Based on V2 IDL (12 accounts)
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "reserveX");
            assign_if_exists(&mut role_by_index, 2, "reserveY");
            assign_if_exists(&mut role_by_index, 3, "tokenXMint");
            assign_if_exists(&mut role_by_index, 4, "tokenYMint");
            assign_if_exists(&mut role_by_index, 5, "receiverTokenX");
            assign_if_exists(&mut role_by_index, 6, "receiverTokenY");
            assign_if_exists(&mut role_by_index, 7, "claimFeeOperator"); // Renamed from feeOwner
            assign_if_exists(&mut role_by_index, 8, "operator"); // Added signer
            assign_if_exists(&mut role_by_index, 9, "tokenXProgram"); // Index updated
            assign_if_exists(&mut role_by_index, 10, "tokenYProgram"); // Index updated
            assign_if_exists(&mut role_by_index, 11, "memoProgram"); // Added
        },

        InstructionType::UpdateFeeParameters => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "admin");
            assign_if_exists(&mut role_by_index, 2, "eventAuthority");
            assign_if_exists(&mut role_by_index, 3, "program");
        },
        
        InstructionType::MigratePosition => {
            assign_if_exists(&mut role_by_index, 0, "positionV2");
            assign_if_exists(&mut role_by_index, 1, "positionV1");
            assign_if_exists(&mut role_by_index, 2, "lbPair");
            assign_if_exists(&mut role_by_index, 3, "binArrayLower");
            assign_if_exists(&mut role_by_index, 4, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 5, "owner");
            assign_if_exists(&mut role_by_index, 6, "systemProgram");
            assign_if_exists(&mut role_by_index, 7, "rentReceiver");
            assign_if_exists(&mut role_by_index, 8, "eventAuthority");
            assign_if_exists(&mut role_by_index, 9, "program");
        },
        
        InstructionType::UpdatePositionOperator => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "owner");
            assign_if_exists(&mut role_by_index, 2, "eventAuthority");
            assign_if_exists(&mut role_by_index, 3, "program");
        },
        
        InstructionType::SetActivationSlot => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "admin");
        },
        
        InstructionType::IncreaseOracleLength => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "admin");
        },
        
        // Add more instruction types as needed
        _ => {
            // Default to generic account labels for unmapped instructions
            for idx in 0..account_indices.len() {
                role_by_index.insert(idx, format!("account_{}", idx));
            }
        }
    }
    
    // Convert index-based mapping to account-based vector and return account roles in the same order as accounts
    let mut accounts = Vec::new();
    for idx in 0..account_indices.len() {
        let role = role_by_index.get(&idx).cloned().unwrap_or_else(|| format!("account_{}", idx));
        accounts.push(role);
    }
    
    // We're returning an empty input_accounts map here - this will be properly filled
    // in the process_instruction function where we have access to the account addresses
    let input_accounts = HashMap::new();
    
    InputAccounts { 
        accounts,
        input_accounts,
    }
}

// Helper function to assign an account role if it exists at a given index
fn assign_if_exists(roles: &mut HashMap<usize, String>, idx: usize, role: &str) {
    roles.insert(idx, role.to_string());
}

// Helper function for instructions with no specific accounts (like EventLog)
pub fn map_empty_accounts() -> InputAccounts {
    InputAccounts {
        accounts: Vec::new(),
        input_accounts: HashMap::new(),
    }
}

// Helper function for generic account mapping if specific mapping not defined
// Used as a fallback or for Unknown instructions 