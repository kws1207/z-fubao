use crate::processor::Processor;
use solana_program::entrypoint;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // Print using ProgramError since Processor doesn't implement PrintProgramError
        msg!("Error: {:?}", error);
        return Err(error);
    }
    Ok(())
}
