//! Type validating that the account signed the transaction
use crate::error::ErrorCode;
use crate::{Accounts, AccountsExit, Key, Result, ToAccountInfos, ToAccountMetas};
use solana_program::account_info::AccountInfo;
use solana_program::instruction::AccountMeta;
use solana_program::pubkey::Pubkey;
use std::collections::BTreeSet;
use std::ops::Deref;

/// Type validating that the account signed the transaction. No other ownership
/// or type checks are done. If this is used, one should not try to access the
/// underlying account data.
///
/// Checks:
///
/// - `Signer.info.is_signer == true`
///
/// # Example
/// ```ignore
/// #[account]
/// #[derive(Default)]
/// pub struct MyData {
///     pub data: u64
/// }
///
/// #[derive(Accounts)]
/// pub struct Example<'info> {
///     #[account(init, payer = payer)]
///     pub my_acc: Account<'info, MyData>,
///     #[account(mut)]
///     pub payer: Signer<'info>,
///     pub system_program: Program<'info, System>
/// }
/// ```
///
/// When creating an account with `init`, the `payer` needs to sign the transaction.
#[derive(Debug, Clone)]
pub struct Signer<'a, 'info> {
    info: &'a AccountInfo<'info>,
}

impl<'a, 'info> Signer<'a, 'info> {
    fn new(info: &'a AccountInfo<'info>) -> Signer<'a, 'info> {
        Self { info }
    }

    /// Deserializes the given `info` into a `Signer`.
    #[inline(never)]
    pub fn try_from(info: &'a AccountInfo<'info>) -> Result<Signer<'a, 'info>> {
        if !info.is_signer {
            return Err(ErrorCode::AccountNotSigner.into());
        }
        Ok(Signer::new(info))
    }
}

impl<'a, 'info, B> Accounts<'a, 'info, B> for Signer<'a, 'info> {
    #[inline(never)]
    fn try_accounts(
        _program_id: &Pubkey,
        accounts: &mut &'a [AccountInfo<'info>],
        _ix_data: &[u8],
        _bumps: &mut B,
        _reallocs: &mut BTreeSet<Pubkey>,
    ) -> Result<Self> {
        if accounts.is_empty() {
            return Err(ErrorCode::AccountNotEnoughKeys.into());
        }
        let account = &accounts[0];
        *accounts = &accounts[1..];
        Signer::try_from(account)
    }
}

impl<'a, 'info> AccountsExit<'info> for Signer<'a, 'info> {}

impl<'a, 'info> ToAccountMetas for Signer<'a, 'info> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.info.is_signer);
        let meta = match self.info.is_writable {
            false => AccountMeta::new_readonly(*self.info.key, is_signer),
            true => AccountMeta::new(*self.info.key, is_signer),
        };
        vec![meta]
    }
}

impl<'a, 'info> ToAccountInfos<'info> for Signer<'a, 'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.info.clone()]
    }
}

impl<'a, 'info> AsRef<AccountInfo<'info>> for Signer<'a, 'info> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        self.info
    }
}

impl<'a, 'info> Deref for Signer<'a, 'info> {
    type Target = AccountInfo<'info>;

    fn deref(&self) -> &Self::Target {
        self.info
    }
}

impl<'a, 'info> Key for Signer<'a, 'info> {
    fn key(&self) -> Pubkey {
        *self.info.key
    }
}
