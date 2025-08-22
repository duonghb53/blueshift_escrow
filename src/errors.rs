use {
    num_derive::FromPrimitive,
    // num_traits::FromPrimitive,
    pinocchio::program_error::{ProgramError, ToStr},
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PinocchioError {
    // 0
    /// Lamport balance below rent-exempt threshold.
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,
    #[error("Invalid address")]
    InvalidAddress,
    #[error("Invalid owner")]
    InvalidOwner,
    #[error("Invalid account data")]
    InvalidAccountData,
    #[error("Invalid instruction data")]
    InvalidInstructionData,
    #[error("Invalid account")]
    InvalidAccount,
    #[error("Not a signer")]
    NotSigner,
}

impl ToStr for PinocchioError {
    fn to_str<E>(&self) -> &'static str {
        match self {
            PinocchioError::NotRentExempt => "Error: Lamport balance below rent-exempt threshold",
            PinocchioError::InvalidAddress => "Error: Invalid address",
            PinocchioError::InvalidOwner => "Error: Invalid owner",
            PinocchioError::InvalidAccountData => "Error: Invalid account data",
            PinocchioError::InvalidInstructionData => "Error: Invalid instruction data",
            PinocchioError::InvalidAccount => "Error: Invalid account",
            PinocchioError::NotSigner => "Error: Not a signer",
        }
    }
}

impl From<PinocchioError> for ProgramError {
    fn from(e: PinocchioError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl TryFrom<u32> for PinocchioError {
    type Error = ProgramError;
    fn try_from(error: u32) -> Result<Self, Self::Error> {
        match error {
            0 => Ok(PinocchioError::NotRentExempt),
            1 => Ok(PinocchioError::InvalidAddress),
            2 => Ok(PinocchioError::InvalidOwner),
            3 => Ok(PinocchioError::InvalidAccountData),
            4 => Ok(PinocchioError::InvalidInstructionData),
            5 => Ok(PinocchioError::InvalidAccount),
            6 => Ok(PinocchioError::NotSigner),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}
