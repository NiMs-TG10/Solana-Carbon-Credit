use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Burn, Transfer};
use mpl_token_metadata::{
    instruction::{create_metadata_accounts_v3},
    state::Metadata,
};
use anchor_lang::solana_program::program::invoke;

declare_id!("YourProgramIdHere");

#[program]
pub mod carbon_credit {
    use super::*;

    // Initialize the mint with Token Extensions metadata
    pub fn initialize_mint(ctx: Context<InitializeMint>, decimals: u8) -> Result<()> {
        // Initialize the SPL token mint
        let cpi_accounts = token::InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        token::initialize_mint(
            cpi_accounts,
            decimals,
            &ctx.accounts.authority.key(),
            Some(&ctx.accounts.authority.key()),
        )?;

        // Create metadata with unified name "CarbonCredit"
        let metadata_seeds = &[
            b"metadata",
            ctx.accounts.metadata_program.key.as_ref(),
            ctx.accounts.mint.key().as_ref(),
        ];
        let (metadata_pda, _bump) = Pubkey::find_program_address(metadata_seeds, &ctx.accounts.metadata_program.key());
        let metadata_accounts = create_metadata_accounts_v3(
            *ctx.accounts.metadata_program.key,
            metadata_pda,
            *ctx.accounts.mint.key,
            *ctx.accounts.authority.key,
            *ctx.accounts.payer.key,
            *ctx.accounts.authority.key,
            "CarbonCredit".to_string(),
            "CC".to_string(),
            "https://your-uri.com".to_string(), // Replace with actual URI for metadata
            None,
            0,
            false,
            false,
            None,
            None,
            None,
        );

        invoke(
            &metadata_accounts,
            &[
                ctx.accounts.metadata_program.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        Ok(())
    }

    // Mint tokens based on carbon credit amount
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::mint_to(cpi_accounts, amount)?;

        Ok(())
    }

    // Transfer tokens to another user (basic trading)
    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        token::transfer(cpi_accounts, amount)?;

        Ok(())
    }

    // Burn tokens to retire carbon credits
    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        token::burn(cpi_accounts, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    #[account(address = mpl_token_metadata::id())]
    pub metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum CarbonCreditError {
    #[msg("Invalid operation.")]
    InvalidOperation,
}