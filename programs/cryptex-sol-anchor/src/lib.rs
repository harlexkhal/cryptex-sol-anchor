use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Burn, Mint, MintTo, SetAuthority, TokenAccount, Transfer, InitializeAccount};
use spl_token::instruction::AuthorityType;

declare_id!("HZoyHJuhYdp7qSBXbthx8mRX5hNTQWcyR5icn27CVPeg");

#[program]
pub mod cryptex_sol_anchor {
    use super::*;

    pub fn wrap(ctx: Context<Wrap>, amount: u64) -> Result<()> {

        let (pda, bump) = Pubkey::find_program_address(&[b"cryptex"], ctx.program_id);

        msg!("pda {}, porgramID {} :: Wrapping!", pda, ctx.program_id);
        token::transfer(
            ctx.accounts.transfer_context(),
            amount,
        )?;

        token::mint_to(
         ctx.accounts.mint_context().with_signer(&[&[&b"cryptex"[..], &[bump]]]),
         amount,
        )?;
        
        Ok(())
    }

    pub fn unwrap(ctx: Context<UnWrap>, amount: u64) -> Result<()> {

        let (pda, bump) = Pubkey::find_program_address(&[b"cryptex"], ctx.program_id);

        msg!("pda {}, porgramID {} :: Unwrapping!", pda, ctx.program_id);

        token::burn(
         ctx.accounts.burn_context(),
         amount,
        )?;
        
        token::transfer(
            ctx.accounts.pda_transfer_context().with_signer(&[&[&b"cryptex"[..], &[bump]]]),
            amount,
        )?;
        Ok(())
    }

    // Assigning authority to pda, can either be acct authority or mint authority
    pub fn assign_authority_to_pda(ctx: Context<AssignAuthorityToPDA>) -> Result<()> {

        let (pda, _bump) = Pubkey::find_program_address(&[b"cryptex"], ctx.program_id);

        token::set_authority(
            ctx.accounts.init_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(pda),
        )?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Wrap<'info> {
    #[account(signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub signer: AccountInfo<'info>,
    #[account(mut)]
    pub transfer_to_pubkey: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub owner_pubkey: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub mint_pubkey: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub mint_to_pubkey: Box<Account<'info, TokenAccount>>,
    #[account()]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub pda_account_pubkey: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UnWrap<'info> {
    #[account(signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub signer: AccountInfo<'info>,
    #[account(mut)]
    pub transfer_to_pubkey: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub owner_pubkey: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub mint_pubkey: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub burn_from: Box<Account<'info, TokenAccount>>,
    #[account()]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub pda_account_pubkey: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AssignAuthorityToPDA<'info> {
    #[account(signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub current_authority_signer: AccountInfo<'info>,
    #[account(mut)]
    pub acct_or_mint_pubkey: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

impl<'info> Wrap<'info> {
    fn transfer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.owner_pubkey.to_account_info().clone(),
            to: self
                .transfer_to_pubkey
                .to_account_info()
                .clone(),
            authority: self.signer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn mint_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint_pubkey.to_account_info().clone(),
            to: self
                .mint_to_pubkey
                .to_account_info()
                .clone(),
            authority: self.pda_account_pubkey.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> UnWrap<'info> {
    fn pda_transfer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.owner_pubkey.to_account_info().clone(),
            to: self
                .transfer_to_pubkey
                .to_account_info()
                .clone(),
            authority: self.pda_account_pubkey.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn burn_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: self.mint_pubkey.to_account_info().clone(),
            from: self
                .burn_from
                .to_account_info()
                .clone(),
            authority: self.signer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> AssignAuthorityToPDA<'info> {
    fn init_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.acct_or_mint_pubkey.to_account_info().clone(),
            current_authority: self.current_authority_signer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}