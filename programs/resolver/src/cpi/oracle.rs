use borsh::BorshDeserialize;
use optimistic_oracle::accounts::Request;
use optimistic_oracle::types::AccountType;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;

pub fn request_from_account_info(info: &AccountInfo) -> Result<Request, ProgramError> {
    if !common::cmp_pubkeys(info.owner, &optimistic_oracle::ID) {
        err!("Request account is not owned by the oracle program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let data = info.try_borrow_data()?;

    let account_type = data.first().copied();
    if account_type != Some(AccountType::Request as u8) {
        err!("Request account has invalid account type: {account_type:x?}");
        return Err(ProgramError::InvalidAccountData);
    }

    Request::deserialize(&mut &data[..]).map_err(|err| {
        err!("Failed to deserialize Request account: {err}");
        ProgramError::InvalidAccountData
    })
}
