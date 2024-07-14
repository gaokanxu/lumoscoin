use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    //msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Token {
    pub owner: Pubkey,
    pub balance: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TokenInstruction {
    CreateToken { initial_supply: u64 },
    Transfer { amount: u64 },
}

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TokenInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let accounts_iter = &mut accounts.iter();
    let payer_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    
    match instruction {
        TokenInstruction::CreateToken { initial_supply } => {
            let mut token = Token::try_from_slice(&token_account.data.borrow())?;
            token.owner = *payer_account.key;
            token.balance = initial_supply;
            token.serialize(&mut &mut token_account.data.borrow_mut()[..])?;
        }
        TokenInstruction::Transfer { amount } => {
            let destination_account = next_account_info(accounts_iter)?;
            let mut token = Token::try_from_slice(&token_account.data.borrow())?;
            
            if token.owner != *payer_account.key {
                return Err(ProgramError::IncorrectProgramId);
            }
            
            
            if token.balance < amount {
                return Err(ProgramError::InsufficientFunds);
            }
            
            token.balance -= amount;
            let mut destination_token = Token::try_from_slice(&destination_account.data.borrow())?;
            destination_token.balance += amount;
            token.serialize(&mut &mut token_account.data.borrow_mut()[..])?;
            destination_token.serialize(&mut &mut destination_account.data.borrow_mut()[..])?;
        }
    }
    Ok(())
}

