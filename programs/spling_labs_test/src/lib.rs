use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, MintTo, mint_to, TokenAccount, Burn};
use anchor_spl::{token};
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::program::invoke;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod spling_labs_test {
    use super::*;

    pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

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
        // Create the CpiContext we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute anchor's helper function to mint tokens
        mint_to(cpi_ctx, amount)?;

        // Tranfer 0.1 SOL as payment to mint the 1000 tokens
        let transfer_instruction = &system_instruction::transfer(
            &ctx.accounts.authority.key(),
            &ctx.accounts.treasury.key(),
            sol_to_lamports(0.1),
        );
        let transfer_account_info = &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.treasury.clone()
        ];
        invoke(transfer_instruction, transfer_account_info)?;
     

        let state = &mut ctx.accounts.state;
        state.treasury = ctx.accounts.treasury.key();
        
        
        // Burn one TokenA token and increase counter by 1 and state size by 10kb
        let burn_cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let burn_cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContext we need for the request
        let burn_cpi_ctx = CpiContext::new(burn_cpi_program, burn_cpi_accounts);

        // Execute anchor's helper function to burn tokens
        token::burn(burn_cpi_ctx, 1)?;

        state.counter = state.counter.saturating_add(1);

        Ok(())
    }

    

}

pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * LAMPORTS_PER_SOL as f64) as u64
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
   /// CHECK: This is the token that we want to mint
   #[account(mut)]
   pub mint: Account<'info, Mint>,
   pub token_program: Program<'info, Token>,
   /// CHECK: This is the token account that we want to mint tokens to
   #[account(init, payer = authority, token::mint = mint, token::authority = authority)]
   pub token_account: Account<'info, TokenAccount>,
   /// CHECK: the authority of the mint account
   #[account(mut)]
   pub authority: Signer<'info>,
    /// Account which holds tokens bidded by biders
   #[account(init, payer = authority, space = 0)]
   /// CHECK:
   pub treasury: AccountInfo<'info>,
   /// CHECK: Save state of program
   #[account(
    init, 
    payer = authority, 
    space = 8 + 100, 
    seeds=[b"state".as_ref(), authority.key().as_ref()],
    bump
   )]
   pub state: Account<'info, State>,
   pub system_program: Program<'info, System>,
   ///CHECK: This is not dangerous because we don't read or write from this account
   pub rent: AccountInfo<'info>,
}


#[account]
pub struct State {
    pub treasury: Pubkey,
    pub counter: u64,
}