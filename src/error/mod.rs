use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulationError {
    #[error("invalid addressing mode")]
    InvalidAddressingMode,
    #[error("opcode not implemented")]
    OpcodeNotImplemented,
    #[error("invalid bus operation")]
    InvalidBusOperation(#[from] BusError),
}

#[derive(Error, Debug)]
pub enum BusError {
    #[error("attempted invalid read at address {0}")]
    InvalidRead(u16),
    #[error("attempted invalid write at address {0}")]
    InvalidWrite(u16),
}
