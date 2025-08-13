use borsh::{BorshDeserialize};
use borsh_derive::BorshSerialize;
use solana_program::program_error::ProgramError;
use std::io;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct UpdateArgs {
  pub value: u32,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct UserValues{
  pub value: u32,
}

pub enum CounterInstruction {
  Increment(UserValues),
  Decrement(UserValues),
  Update(UpdateArgs),
  Reset
}

impl CounterInstruction{
  pub fn unpack(input: &[u8]) -> Result<Self,ProgramError> {
    let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;


    Ok(match variant{
      0 => Self::Increment(UserValues::try_from_slice(rest).unwrap()),
      1 => Self::Decrement(UserValues::try_from_slice(rest).unwrap()),
      2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
      3 => Self::Reset,
      _ => return Err(ProgramError::InvalidInstructionData),
    })
  }
}