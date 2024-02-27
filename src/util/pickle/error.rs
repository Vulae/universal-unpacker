
use std::{error::Error, fmt};
use super::parser::{PickleOpcode, PickleProtocol};



#[derive(Debug)]
pub enum PickleError {
    UnknownProtocol(u8),
    UnsupportedProtocol(PickleProtocol),
    StackEmpty,
    StackMark,
    MemoIndexOutOfBounds,
    MemoIndexEmpty,
    MemoEmpty,
    PickleInvalidProtocolOperation,
    UnsupportedOperation(PickleOpcode),
    InvalidOperation(u8),
    InvalidReferencePickleType,
    CannotTryInto,
}

impl fmt::Display for PickleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownProtocol(protocol) => write!(f, "Pickle protocol unknown. {:#?}", protocol),
            Self::UnsupportedProtocol(protocol) => write!(f, "Pickle protocol unsupported. {:#?}", protocol),
            Self::StackEmpty => write!(f, "Pickle stack operation cannot complete due to stack being empty."),
            Self::StackMark => write!(f, "Pickle stack operation cannot complete due to item being a marker."),
            Self::MemoIndexOutOfBounds => write!(f, "Pickle memo list index out of bounds."),
            Self::MemoIndexEmpty => write!(f, "Pickle memo index position is empty."),
            Self::MemoEmpty => write!(f, "Pickle memo cannot get item due to memo list being empty."),
            Self::PickleInvalidProtocolOperation => write!(f, "Pickle first operation MUST be PROTO."),
            Self::UnsupportedOperation(opcode) => write!(f, "Pickle unsupported operation {:#?}.", opcode),
            Self::InvalidOperation(opcode) => write!(f, "Pickle invalid operation {:x}", opcode),
            Self::InvalidReferencePickleType => write!(f, "Pickle tried to reference pickle that is not correct type."),
            Self::CannotTryInto => write!(f, "Pickle cannot try into."),
        }
    }
}

impl Error for PickleError { }


