//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::ResolveArgs;
use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct Resolve {
    /// Resolver
    pub resolver: solana_program::pubkey::Pubkey,
    /// Parimutuel market
    pub market: solana_program::pubkey::Pubkey,
    /// Oracle request
    pub request: solana_program::pubkey::Pubkey,
    /// Parimutuel program
    pub parimutuel_program: solana_program::pubkey::Pubkey,
}

impl Resolve {
    pub fn instruction(
        &self,
        args: ResolveInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: ResolveInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.resolver, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.market, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.request, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.parimutuel_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = ResolveInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::PARIMUTUEL_RESOLVER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct ResolveInstructionData {
    discriminator: u8,
}

impl ResolveInstructionData {
    fn new() -> Self {
        Self { discriminator: 1 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResolveInstructionArgs {
    pub resolve_args: ResolveArgs,
}

/// Instruction builder for `Resolve`.
///
/// ### Accounts:
///
///   0. `[]` resolver
///   1. `[writable]` market
///   2. `[]` request
///   3. `[optional]` parimutuel_program (default to `Cf9JrByfmw6CYSry39pfg2BSGHRgde2Cp5y6yZ3a2Yeo`)
#[derive(Default)]
pub struct ResolveBuilder {
    resolver: Option<solana_program::pubkey::Pubkey>,
    market: Option<solana_program::pubkey::Pubkey>,
    request: Option<solana_program::pubkey::Pubkey>,
    parimutuel_program: Option<solana_program::pubkey::Pubkey>,
    resolve_args: Option<ResolveArgs>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ResolveBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Resolver
    #[inline(always)]
    pub fn resolver(&mut self, resolver: solana_program::pubkey::Pubkey) -> &mut Self {
        self.resolver = Some(resolver);
        self
    }
    /// Parimutuel market
    #[inline(always)]
    pub fn market(&mut self, market: solana_program::pubkey::Pubkey) -> &mut Self {
        self.market = Some(market);
        self
    }
    /// Oracle request
    #[inline(always)]
    pub fn request(&mut self, request: solana_program::pubkey::Pubkey) -> &mut Self {
        self.request = Some(request);
        self
    }
    /// `[optional account, default to 'Cf9JrByfmw6CYSry39pfg2BSGHRgde2Cp5y6yZ3a2Yeo']`
    /// Parimutuel program
    #[inline(always)]
    pub fn parimutuel_program(
        &mut self,
        parimutuel_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.parimutuel_program = Some(parimutuel_program);
        self
    }
    #[inline(always)]
    pub fn resolve_args(&mut self, resolve_args: ResolveArgs) -> &mut Self {
        self.resolve_args = Some(resolve_args);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Resolve {
            resolver: self.resolver.expect("resolver is not set"),
            market: self.market.expect("market is not set"),
            request: self.request.expect("request is not set"),
            parimutuel_program: self
                .parimutuel_program
                .unwrap_or(solana_program::pubkey!("Cf9JrByfmw6CYSry39pfg2BSGHRgde2Cp5y6yZ3a2Yeo")),
        };
        let args = ResolveInstructionArgs {
            resolve_args: self.resolve_args.clone().expect("resolve_args is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `resolve` CPI accounts.
pub struct ResolveCpiAccounts<'a, 'b> {
    /// Resolver
    pub resolver: &'b solana_program::account_info::AccountInfo<'a>,
    /// Parimutuel market
    pub market: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Parimutuel program
    pub parimutuel_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `resolve` CPI instruction.
pub struct ResolveCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Resolver
    pub resolver: &'b solana_program::account_info::AccountInfo<'a>,
    /// Parimutuel market
    pub market: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Parimutuel program
    pub parimutuel_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: ResolveInstructionArgs,
}

impl<'a, 'b> ResolveCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ResolveCpiAccounts<'a, 'b>,
        args: ResolveInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            resolver: accounts.resolver,
            market: accounts.market,
            request: accounts.request,
            parimutuel_program: accounts.parimutuel_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.resolver.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.market.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.request.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.parimutuel_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = ResolveInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::PARIMUTUEL_RESOLVER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.resolver.clone());
        account_infos.push(self.market.clone());
        account_infos.push(self.request.clone());
        account_infos.push(self.parimutuel_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Resolve` via CPI.
///
/// ### Accounts:
///
///   0. `[]` resolver
///   1. `[writable]` market
///   2. `[]` request
///   3. `[]` parimutuel_program
pub struct ResolveCpiBuilder<'a, 'b> {
    instruction: Box<ResolveCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ResolveCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ResolveCpiBuilderInstruction {
            __program: program,
            resolver: None,
            market: None,
            request: None,
            parimutuel_program: None,
            resolve_args: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Resolver
    #[inline(always)]
    pub fn resolver(
        &mut self,
        resolver: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.resolver = Some(resolver);
        self
    }
    /// Parimutuel market
    #[inline(always)]
    pub fn market(
        &mut self,
        market: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.market = Some(market);
        self
    }
    /// Oracle request
    #[inline(always)]
    pub fn request(
        &mut self,
        request: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.request = Some(request);
        self
    }
    /// Parimutuel program
    #[inline(always)]
    pub fn parimutuel_program(
        &mut self,
        parimutuel_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.parimutuel_program = Some(parimutuel_program);
        self
    }
    #[inline(always)]
    pub fn resolve_args(&mut self, resolve_args: ResolveArgs) -> &mut Self {
        self.instruction.resolve_args = Some(resolve_args);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> &mut Self {
        self.instruction.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = ResolveInstructionArgs {
            resolve_args: self.instruction.resolve_args.clone().expect("resolve_args is not set"),
        };
        let instruction = ResolveCpi {
            __program: self.instruction.__program,

            resolver: self.instruction.resolver.expect("resolver is not set"),

            market: self.instruction.market.expect("market is not set"),

            request: self.instruction.request.expect("request is not set"),

            parimutuel_program: self
                .instruction
                .parimutuel_program
                .expect("parimutuel_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct ResolveCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    resolver: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    parimutuel_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    resolve_args: Option<ResolveArgs>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
