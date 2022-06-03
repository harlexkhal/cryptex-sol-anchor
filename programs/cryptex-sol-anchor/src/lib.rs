use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, MintTo, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod cryptex_sol_anchor {
    use super::*;

    pub fn stake(ctx: Context<Stake>) -> Result<()> {

        token::transfer(
            ctx.accounts.transfer_to_fluidity_context(),
            ctx.accounts.stake_account.amount,
        )?;

        Ok(())
    }

    pub fn mint(ctx: Context<Fmint>) -> Result<()> {

        token::mint_to(
            ctx.accounts.mint_to_user_context(),
            ctx.accounts.stake_account.amount,
        )?;

        Ok(())
    }
}

#[account]
pub struct StakeAccount {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub signer: AccountInfo<'info>,
    #[account(mut)]
    pub destination_pubkey: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub source_pubkey: Box<Account<'info, TokenAccount>>,
    pub stake_account: Box<Account<'info, StakeAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Fmint<'info> {
    #[account(signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub signer: AccountInfo<'info>,
    #[account(mut)]
    pub mint_token_pubkey: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub destination_pubkey: Box<Account<'info, TokenAccount>>,
    pub stake_account: Box<Account<'info, StakeAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

impl<'info> Stake<'info> {
    fn transfer_to_fluidity_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.source_pubkey.to_account_info().clone(),
            to: self
                .destination_pubkey
                .to_account_info()
                .clone(),
            authority: self.signer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> Fmint<'info> {
    fn mint_to_user_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint_token_pubkey.to_account_info().clone(),
            to: self
                .destination_pubkey
                .to_account_info()
                .clone(),
            authority: self.signer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}