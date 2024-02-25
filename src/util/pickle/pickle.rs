
// https://github.com/python/cpython/blob/main/Lib/pickle.py

use std::{collections::HashMap, error::Error};
use super::error::PickleError;



#[derive(Debug, Clone)]
pub enum Pickle {
    Dict(HashMap<String, Pickle>),
    String(String),
    List(Vec<Pickle>),
    Number(PickleNumber),
    Binary(Vec<u8>),
    Tuple3((Box<Pickle>, Box<Pickle>, Box<Pickle>)),
}

impl Pickle {

    pub fn list_push(&mut self, item: Pickle) -> Result<(), Box<dyn Error>> {
        match self {
            Pickle::List(list) => list.push(item),
            _ => return Err(Box::new(PickleError::InvalidReferencePickleType)),
        }
        Ok(())
    }

    pub fn dict_set(&mut self, key: Pickle, item: Pickle) -> Result<(), Box<dyn Error>> {
        match self {
            Pickle::Dict(dict) => {
                let key = match key {
                    Pickle::String(str) => str,
                    _ => return Err(Box::new(PickleError::InvalidReferencePickleType)),
                };

                dict.insert(key, item);
            }
            _ => return Err(Box::new(PickleError::InvalidReferencePickleType)),
        }
        Ok(())
    }

}



#[derive(Debug, Clone)]
pub enum PickleNumber {
    Int(i64),
    Uint(u64),
    Float(f64),
}



// TODO: Change to macro.
impl TryInto<u8> for Pickle {
    type Error = PickleError;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Pickle::Number(PickleNumber::Int(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            Pickle::Number(PickleNumber::Uint(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            _ => Err(PickleError::CannotTryInto),
        }
    }
}

impl TryInto<u16> for Pickle {
    type Error = PickleError;

    fn try_into(self) -> Result<u16, Self::Error> {
        match self {
            Pickle::Number(PickleNumber::Int(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            Pickle::Number(PickleNumber::Uint(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            _ => Err(PickleError::CannotTryInto),
        }
    }
}

impl TryInto<u32> for Pickle {
    type Error = PickleError;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            Pickle::Number(PickleNumber::Int(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            Pickle::Number(PickleNumber::Uint(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            _ => Err(PickleError::CannotTryInto),
        }
    }
}

impl TryInto<u64> for Pickle {
    type Error = PickleError;

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            Pickle::Number(PickleNumber::Int(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            Pickle::Number(PickleNumber::Uint(v)) => Ok(v),
            _ => Err(PickleError::CannotTryInto),
        }
    }
}

impl TryInto<i8> for Pickle {
    type Error = PickleError;

    fn try_into(self) -> Result<i8, Self::Error> {
        match self {
            Pickle::Number(PickleNumber::Int(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            Pickle::Number(PickleNumber::Uint(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            _ => Err(PickleError::CannotTryInto),
        }
    }
}

impl TryInto<i16> for Pickle {
    type Error = PickleError;

    fn try_into(self) -> Result<i16, Self::Error> {
        match self {
            Pickle::Number(PickleNumber::Int(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            Pickle::Number(PickleNumber::Uint(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            _ => Err(PickleError::CannotTryInto),
        }
    }
}

impl TryInto<i32> for Pickle {
    type Error = PickleError;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Pickle::Number(PickleNumber::Int(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            Pickle::Number(PickleNumber::Uint(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            _ => Err(PickleError::CannotTryInto),
        }
    }
}

impl TryInto<i64> for Pickle {
    type Error = PickleError;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Pickle::Number(PickleNumber::Int(v)) => Ok(v),
            Pickle::Number(PickleNumber::Uint(v)) => v.try_into().map_err(|_| PickleError::CannotTryInto),
            _ => Err(PickleError::CannotTryInto),
        }
    }
}

impl TryInto<f32> for Pickle {
    type Error = PickleError;

    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Pickle::Number(PickleNumber::Float(v)) => Ok(v as f32), // Precision loss.
            _ => Err(PickleError::CannotTryInto),
        }
    }
}

impl TryInto<f64> for Pickle {
    type Error = PickleError;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Pickle::Number(PickleNumber::Float(v)) => Ok(v),
            _ => Err(PickleError::CannotTryInto),
        }
    }
}
