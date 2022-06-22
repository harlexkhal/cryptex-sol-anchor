use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Burn, Mint, MintTo, SetAuthority, TokenAccount, Transfer, InitializeAccount};
use spl_token::instruction::AuthorityType;

declare_id!("HZoyHJuhYdp7qSBXbthx8mRX5hNTQWcyR5icn27CVPeg");

const TRUSTED_AUTHORITY: &str = "2VYJuoYPoHmtNkrYcuYtppBiX1sxiMmL2mvDZuJq27Jr";

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
        
        let token_amount = jet_proto_v1_cpi::Amount {
            units: jet_proto_v1_cpi::AmountUnits::Tokens,
            value: amount,
        };
        
        //deposit token into jet.
        jet_proto_v1_cpi::deposit_tokens(
            ctx.accounts.jet_v1_deposit_context(),
            token_amount
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

        let token_amount = jet_proto_v1_cpi::Amount {
            units: jet_proto_v1_cpi::AmountUnits::Tokens,
            value: amount,
        };

        //withdraw token from jet.
        jet_proto_v1_cpi::withdraw_tokens(
            ctx.accounts.jet_v1_withdraw_context(),
            token_amount
        )?;

        Ok(())
    }

    pub fn reward(_ctx: Context<Reward>, _sender_amount: u64, _receiver_amount: u64) -> Result<()> {
        //do like a transfer from a prize pool account owned by the pda.
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

    // deposit note account. for token returned by jet as kinda like a receipt for your deposit
    #[account(mut)]
    pub deposit_note_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,

    //accounts provided by Jet
    #[account(mut)]
    pub market_authority: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub market: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub reserve: Box<Account<'info, TokenAccount>>,
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

    //accounts provided by Jet
    #[account(mut)]
    pub market_authority: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub market: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub reserve: Box<Account<'info, TokenAccount>>,
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

#[derive(Accounts)]
pub struct Reward<'info> {
    #[account(signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub signer: AccountInfo<'info>,
    
    #[account(
        mut,
        constraint = Pubkey::from_str(TRUSTED_AUTHORITY).unwrap() == *signer.key,
    )]
    pub sender_address_pubkey: Box<Account<'info, TokenAccount>>,
    pub receiver_address_pubkey: Box<Account<'info, TokenAccount>>,
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

    fn jet_v1_deposit_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, jet_proto_v1_cpi::accounts::DepositTokens<'info>> {
        let cpi_accounts = jet_proto_v1_cpi::accounts::DepositTokens {
            market: self.market.to_account_info().clone(),
            market_authority: self.market_authority.to_account_info().clone(),
            reserve: self.pda_account_pubkey.to_account_info().clone(),
            vault: self.pda_account_pubkey.to_account_info().clone(),
            deposit_note_mint: self.pda_account_pubkey.to_account_info().clone(),
            depositor: self.pda_account_pubkey.to_account_info().clone(),
            deposit_note_account: self.pda_account_pubkey.to_account_info().clone(),
            deposit_source: self.pda_account_pubkey.to_account_info().clone(),
            token_program: self.pda_account_pubkey.clone(),
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

    fn jet_v1_withdraw_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, jet_proto_v1_cpi::accounts::WithdrawTokens<'info>> {
        let cpi_accounts = jet_proto_v1_cpi::accounts::WithdrawTokens {
            market: self.market.to_account_info().clone(),
            market_authority: self.market_authority.to_account_info().clone(),
            reserve: self.pda_account_pubkey.to_account_info().clone(),
            vault: self.pda_account_pubkey.to_account_info().clone(),
            deposit_note_mint: self.pda_account_pubkey.to_account_info().clone(),
            depositor: self.pda_account_pubkey.to_account_info().clone(),
            deposit_note_account: self.pda_account_pubkey.to_account_info().clone(),
            withdraw_account: self.pda_account_pubkey.to_account_info().clone(),
            token_program: self.pda_account_pubkey.clone(),
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