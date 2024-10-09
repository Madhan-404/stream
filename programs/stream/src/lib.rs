use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use streamflow_sdk;
use streamflow_sdk::accounts::Transfer as CpiTransfer;

declare_id!("B3kcoBKPkVwVo3oUQ3bwndM3QgnEgfHXWu7ZbXxF3zaE");

#[program]
pub mod marketplace {
    use super::*;

    pub fn list_contract(ctx: Context<ListContract>, price: u64,stream_id: Pubkey) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.seller = ctx.accounts.seller.key();
        listing.price = price;
        listing.stream_id = stream_id;
        listing.status = ListingStatus::Active;

        Ok(())
    }

    pub fn buy_contract(ctx: Context<BuyContract>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;

        if ctx.accounts.buyer.lamports() < listing.price {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Transfer ownership using Streamflow SDK
        let accs = CpiTransfer {
            authority: ctx.accounts.seller.to_account_info(),
            new_recipient: ctx.accounts.buyer.to_account_info(),
            new_recipient_tokens: ctx.accounts.buyer_tokens.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.streamflow_program.to_account_info(), accs);
        streamflow_sdk::cpi::buy_contract(cpi_ctx)

        // Transfer SOL to seller
        let transfer_instruction = anchor_lang::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &ctx.accounts.seller.key(),
            listing.price
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.seller.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        listing.status = ListingStatus::Sold;

        Ok(())
    }

    pub fn delist_contract(ctx: Context<DelistContract>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;

        if listing.seller != ctx.accounts.seller.key() {
            return Err(ErrorCode::Unauthorized.into());
        }

        if listing.status != ListingStatus::Active {
            return Err(ErrorCode::AlreadySold.into());
        }

        listing.status = ListingStatus::Delisted;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListContract<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + 32 + 8 + 32 + 1, // discriminator + seller + price + contract_address + status
        seeds = [b"listing", seller.key().as_ref(), contract.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub streamflow_program: Program<'info, streamflow_sdk::program::Streamflow>,
}

#[derive(Accounts)]
pub struct BuyContract<'info> {
    #[account(
        mut,
        seeds = [b"listing", listing.seller.as_ref(), listing.contract_address.as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer
    )]
    pub buyer_tokens: Account<'info, TokenAccount>,
    pub metadata: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub streamflow_program: Program<'info, streamflow_sdk::program::Streamflow>,
}

#[derive(Accounts)]
pub struct DelistContract<'info> {
    #[account(
        mut,
        seeds = [b"listing", seller.key().as_ref(), listing.contract_address.as_ref()],
        bump,
        has_one = seller
    )]
    pub listing: Account<'info, Listing>,
    pub seller: Signer<'info>,
}

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub price: u64,
    pub contract_address: Pubkey,
    pub status: ListingStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ListingStatus {
    Active,
    Sold,
    Delisted,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds for this purchase.")]
    InsufficientFunds,
    
    #[msg("Unauthorized action.")]
    Unauthorized,

    #[msg("This contract has already been sold or delisted.")]
    AlreadySold,
}