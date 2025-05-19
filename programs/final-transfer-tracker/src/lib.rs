#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Final Transfer Tracker",
    project_url: "https://github.com/TurboChainx/final-transfer-tracker",
    contacts: "email:jrubeiphone@gmail.com, telegram:@jnice2025, discord:vD9GkpCJ",
    policy: "https://github.com/TurboChainx/final-transfer-tracker/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/TurboChainx/final-transfer-tracker",
    auditors: "darkSs Teams",
    acknowledgements: "Whitehat researchers are welcome."
}

// programs/final-transfer_tracker/src/lib.rs
use anchor_lang::prelude::*;

declare_id!("2K3rtnK7jN3BmFvPW4coMiF31noaXuaER6N4edNJZm7N");

#[program]
pub mod final_transfer_tracker {
    use super::*;

    pub fn initialize_owner(ctx: Context<InitializeOwner>) -> Result<()> {
        let owner_account = &mut ctx.accounts.owner_account;
        owner_account.owner = *ctx.accounts.signer.key;

        emit!(OwnerInitialized {
            owner: owner_account.owner,
        });

        Ok(())
    }

    // Owner-only: Update existing record
    pub fn add_transfer(
        ctx: Context<AddTransfer>,
        signature_1: String,
        signature_2: String,
        signature_3: String,
        from: Pubkey,
        to: Pubkey,
        amount: u64,
        timestamp: i64,
        wallet_balance: u64,
        sol_price: u64,
        token_price: u64,
    ) -> Result<()> {

        let owner = &ctx.accounts.owner_account.owner;
        require_keys_eq!(*owner, ctx.accounts.signer.key(), CustomError::Unauthorized);

        let record = &mut ctx.accounts.transfer_record;
        record.signature_1 = signature_1;
        record.signature_2 = signature_2;
        record.signature_3 = signature_3;
        record.from = from;
        record.to = to;
        record.amount = amount;
        record.timestamp = timestamp;
        record.wallet_balance = wallet_balance;
        record.sol_price = sol_price;
        record.token_price = token_price;

        emit!(TransferAdded {
            signature_1: record.signature_1.clone(),
            signature_2: record.signature_2.clone(),
            signature_3: record.signature_3.clone(),
            from: record.from,
            to: record.to,
            amount: record.amount,
            timestamp: record.timestamp,
            wallet_balance: record.wallet_balance,
            sol_price: record.sol_price,
            token_price: record.token_price,
        });

        Ok(())
    }

    // Owner-only: Update existing record
    pub fn update_transfer(
        ctx: Context<UpdateTransfer>,
        new_token_price: u64,
        new_sol_price: u64,
        new_wallet_balance: u64,
    ) -> Result<()> {
        let owner = &ctx.accounts.owner_account.owner;
        require_keys_eq!(*owner, ctx.accounts.signer.key(), CustomError::Unauthorized);

        let record = &mut ctx.accounts.transfer_record;
        record.wallet_balance = new_wallet_balance;
        record.sol_price = new_sol_price;
        record.token_price = new_token_price;

        emit!(TransferUpdated {
            signature_1: record.signature_1.clone(),
            signature_2: record.signature_2.clone(),
            signature_3: record.signature_3.clone(),
            new_wallet_balance: record.wallet_balance,
            new_sol_price: record.sol_price,
            new_token_price: record.token_price,
        });

        Ok(())
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>, new_owner: Pubkey) -> Result<()> {
        let owner_account = &mut ctx.accounts.owner_account;
        require_keys_eq!(owner_account.owner, ctx.accounts.signer.key(), CustomError::Unauthorized);
        owner_account.owner = new_owner;

        emit!(OwnershipTransferred {
            old_owner: ctx.accounts.signer.key(),
            new_owner,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeOwner<'info> {
    #[account(init, payer = signer, space = 8 + 32, seeds = [b"owner"], bump)]
    pub owner_account: Account<'info, OwnerAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(signature_1: String, signature_2: String, signature_3: String)]
pub struct AddTransfer<'info> {
    #[account(mut, seeds = [b"owner"], bump)]
    pub owner_account: Account<'info, OwnerAccount>,
    #[account(init, payer = signer, space = 8 + 1024, seeds = [b"transfer", signature_1.as_bytes(), signature_2.as_bytes(), signature_3.as_bytes()], bump)]
    pub transfer_record: Account<'info, TransferRecord>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTransfer<'info> {
    #[account(mut, seeds = [b"owner"], bump)]
    pub owner_account: Account<'info, OwnerAccount>,
    #[account(mut, seeds = [b"transfer", transfer_record.signature_1.as_bytes(), transfer_record.signature_2.as_bytes(), transfer_record.signature_3.as_bytes()], bump)]
    pub transfer_record: Account<'info, TransferRecord>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(mut, seeds = [b"owner"], bump)]
    pub owner_account: Account<'info, OwnerAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
pub struct OwnerAccount {
    pub owner: Pubkey,
}

#[account]
pub struct TransferRecord {
    pub signature_1: String,
    pub signature_2: String,
    pub signature_3: String,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub wallet_balance: u64,
    pub sol_price: u64,
    pub token_price: u64,
}

#[event]
pub struct OwnerInitialized {
    pub owner: Pubkey,
}

#[event]
pub struct TransferAdded {
    pub signature_1: String,
    pub signature_2: String,
    pub signature_3: String,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub wallet_balance: u64,
    pub sol_price: u64,
    pub token_price: u64,
}

#[event]
pub struct TransferUpdated {
    pub signature_1: String,
    pub signature_2: String,
    pub signature_3: String,
    pub new_wallet_balance: u64,
    pub new_sol_price: u64,
    pub new_token_price: u64,
}

#[event]
pub struct OwnershipTransferred {
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
}

#[error_code]
pub enum CustomError {
    #[msg("Only the owner can perform this action.")]
    Unauthorized,
}
