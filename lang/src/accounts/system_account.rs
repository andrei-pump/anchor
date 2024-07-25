//! Type validating that the account is owned by the system program

use crate::error::ErrorCode;
use crate::*;
use solana_program::system_program;
use std::ops::Deref;

/// Type validating that the account is owned by the system program
///
/// Checks:
///
/// - `SystemAccount.info.owner == SystemProgram`
#[derive(Debug, Clone)]
pub struct SystemAccount<'a, 'info> {
    info: &'a AccountInfo<'info>,
}

impl<'a, 'info> SystemAccount<'a, 'info> {
    fn new(info: &'a AccountInfo<'info>) -> SystemAccount<'a, 'info> {
        Self { info }
    }

    #[inline(never)]
    pub fn try_from(info: &'a AccountInfo<'info>) -> Result<SystemAccount<'a, 'info>> {
        if *info.owner != system_program::ID {
            return Err(ErrorCode::AccountNotSystemOwned.into());
        }
        Ok(SystemAccount::new(info))
    }
}

impl<'a, 'info, B> Accounts<'a, 'info, B> for SystemAccount<'a, 'info> {
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
        SystemAccount::try_from(account)
    }
}

impl<'a, 'info> AccountsExit<'info> for SystemAccount<'a, 'info> {}

impl<'a, 'info> ToAccountMetas for SystemAccount<'a, 'info> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.info.is_signer);
        let meta = match self.info.is_writable {
            false => AccountMeta::new_readonly(*self.info.key, is_signer),
            true => AccountMeta::new(*self.info.key, is_signer),
        };
        vec![meta]
    }
}

impl<'a, 'info> ToAccountInfos<'info> for SystemAccount<'a, 'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.info.clone()]
    }
}

impl<'a, 'info> AsRef<AccountInfo<'info>> for SystemAccount<'a, 'info> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        self.info
    }
}

impl<'a, 'info> Deref for SystemAccount<'a, 'info> {
    type Target = AccountInfo<'info>;

    fn deref(&self) -> &Self::Target {
        self.info
    }
}

impl<'a, 'info> Key for SystemAccount<'a, 'info> {
    fn key(&self) -> Pubkey {
        *self.info.key
    }
}
