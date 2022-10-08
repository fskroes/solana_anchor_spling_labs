use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, MintTo};


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod spling_labs_test {
    use anchor_spl::token::mint_to;

    use super::*;

    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        Ok(())
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        
        // Create the MintTo struct for our context
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute Anchor's helper function to mint tokens
        mint_to(cpi_ctx, amount)?;


        // Function to tranfer funds from Payer to PDA (TODO)
        // let instruction = &system_instruction::transfer(
        //     &ctx.accounts.bidder.key(),
        //     &ctx.accounts.treasury.key(),
        //     raised_by,
        // );
        // let account_info = &[
        //     ctx.accounts.bidder.to_account_info(),
        //     ctx.accounts.treasury.clone()
        // ];
        // invoke(instruction, account_info)?;


        Ok(())
    }
}


#[derive(Accounts)]
#[instruction()]
pub struct InitializeMint<'info> {
    
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    
    ///CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    
    /// CHECK: this is the token that we want to mint
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
   /// CHECK: This is the token account that we want to mint tokens to
   #[account(mut)]
   pub token_account: AccountInfo<'info>,
   /// CHECK: the authority of the mint account
   pub authority: Signer<'info>,  
}


#[account]
#[derive(Default)]
pub struct State {
    pub counter: u8,

    pub user_sending: Pubkey,

    pub mint_of_token_being_sent: Pubkey,

    pub escrow_wallet: Pubkey,

    pub amount_tokens: u64,
}