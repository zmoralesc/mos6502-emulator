use thiserror::Error;

use crate::mos6502::AddressingMode;

#[derive(Error, Debug)]
pub enum CpuError {
    #[error("invalid addressing mode")]
    InvalidAddressingMode(AddressingMode),
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
    #[error("attempted write on read-only address {0}")]
    ReadOnlyAddress(u16),
}
