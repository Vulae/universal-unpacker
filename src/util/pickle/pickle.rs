
// https://github.com/python/cpython/blob/main/Lib/pickle.py

use std::collections::HashMap;
use super::error::PickleError;



// TODO: Maybe some way to convert pickle to type if possible.
// eg: let a: Vec<(String, Vec<(u64, u64)>)> = pickle.parse()?;

#[derive(Debug, Clone)]
pub enum Pickle {
    None,
    Bool(bool),
    Number(PickleNumber),
    String(String),
    Binary(Vec<u8>),
    List(Vec<Pickle>),
    Dict(HashMap<String, Pickle>),
    Tuple(Vec<Pickle>),
    Module(PickleModule),
    Class(PickleClass),
}



#[derive(Debug, Clone)]
pub enum PickleNumber {
    Int(i64),
    Uint(u64),
    Float(f64),
}



#[derive(Debug, Clone)]
pub struct PickleModule {
    pub module: String,
    pub name: String,
}

impl PickleModule {
    pub fn new(module: String, name: String) -> Self {
        Self { module, name }
    }

    pub fn class(&mut self, args: Pickle) -> PickleClass {
        PickleClass::new(self.clone(), args)
    }
}



#[derive(Debug, Clone)]
pub struct PickleClass {
    pub module: PickleModule,
    pub args: Box<Pickle>,
    pub state: Option<Box<Pickle>>,
    pub data: HashMap<String, Pickle>,
}

impl PickleClass {
    pub fn new(module: PickleModule, args: Pickle) -> Self {
        Self { module, args: Box::new(args), state: None, data: HashMap::new() }
    }
}





macro_rules! pickle_try_into {
    ($type:ty, $match_head:pat, $match_body:block) => (
        impl TryInto<$type> for Pickle {
            type Error = PickleError;

            fn try_into(self) -> Result<$type, Self::Error> {
                match self {
                    $match_head => $match_body,
                    _ => Err(PickleError::CannotTryInto),
                }
            }
        }

        impl TryInto<Option<$type>> for Pickle {
            type Error = PickleError;

            fn try_into(self) -> Result<Option<$type>, Self::Error> {
                match self {
                    $match_head => {
                        match $match_body {
                            Ok(v) => Ok(Some(v)),
                            Err(err) => Err(err)
                        }
                    },
                    Pickle::None => Ok(None),
                    _ => Err(PickleError::CannotTryInto),
                }
            }
        }
    )
}



pickle_try_into!(bool, Pickle::Bool(bool), { Ok(bool) });

pickle_try_into!(u8, Pickle::Number(PickleNumber::Uint(v)), { v.try_into().map_err(|_| PickleError::CannotTryInto) });
pickle_try_into!(u16, Pickle::Number(PickleNumber::Uint(v)), { v.try_into().map_err(|_| PickleError::CannotTryInto) });
pickle_try_into!(u32, Pickle::Number(PickleNumber::Uint(v)), { v.try_into().map_err(|_| PickleError::CannotTryInto) });
pickle_try_into!(u64, Pickle::Number(PickleNumber::Uint(v)), { v.try_into().map_err(|_| PickleError::CannotTryInto) });
pickle_try_into!(i8, Pickle::Number(PickleNumber::Int(v)), { v.try_into().map_err(|_| PickleError::CannotTryInto) });
pickle_try_into!(i16, Pickle::Number(PickleNumber::Int(v)), { v.try_into().map_err(|_| PickleError::CannotTryInto) });
pickle_try_into!(i32, Pickle::Number(PickleNumber::Int(v)), { v.try_into().map_err(|_| PickleError::CannotTryInto) });
pickle_try_into!(i64, Pickle::Number(PickleNumber::Int(v)), { v.try_into().map_err(|_| PickleError::CannotTryInto) });
pickle_try_into!(f32, Pickle::Number(PickleNumber::Float(v)), { Ok(v as f32) }); // Precision loss.
pickle_try_into!(f64, Pickle::Number(PickleNumber::Float(v)), { v.try_into().map_err(|_| PickleError::CannotTryInto) });

pickle_try_into!(String, Pickle::String(str), { Ok(str) });

pickle_try_into!(Vec<u8>, Pickle::Binary(bin), { Ok(bin) });

pickle_try_into!(Vec<Pickle>, Pickle::List(vec) | Pickle::Tuple(vec), { Ok(vec) });

pickle_try_into!(HashMap<String, Pickle>, Pickle::Dict(dict), { Ok(dict) });

pickle_try_into!((), Pickle::Tuple(tuple), {
    if tuple.len() == 0 {
        Ok(())
    } else {
        Err(PickleError::CannotTryInto)
    }
});
// This is very dirty.
// But I cannot figure out another way to do a single-tuple.
pickle_try_into!((Pickle, ()), Pickle::Tuple(tuple), {
    if tuple.len() == 1 {
        Ok((tuple[0].clone(), ()))
    } else {
        Err(PickleError::CannotTryInto)
    }
});
pickle_try_into!((Pickle, Pickle), Pickle::Tuple(tuple), {
    if tuple.len() == 2 {
        Ok((tuple[0].clone(), tuple[1].clone()))
    } else {
        Err(PickleError::CannotTryInto)
    }
});
pickle_try_into!((Pickle, Pickle, Pickle), Pickle::Tuple(tuple), {
    if tuple.len() == 3 {
        Ok((tuple[0].clone(), tuple[1].clone(), tuple[2].clone()))
    } else {
        Err(PickleError::CannotTryInto)
    }
});
pickle_try_into!((Pickle, Pickle, Pickle, Pickle), Pickle::Tuple(tuple), {
    if tuple.len() == 4 {
        Ok((tuple[0].clone(), tuple[1].clone(), tuple[2].clone(), tuple[3].clone()))
    } else {
        Err(PickleError::CannotTryInto)
    }
});
pickle_try_into!((Pickle, Pickle, Pickle, Pickle, Pickle), Pickle::Tuple(tuple), {
    if tuple.len() == 5 {
        Ok((tuple[0].clone(), tuple[1].clone(), tuple[2].clone(), tuple[3].clone(), tuple[4].clone()))
    } else {
        Err(PickleError::CannotTryInto)
    }
});
pickle_try_into!((Pickle, Pickle, Pickle, Pickle, Pickle, Pickle), Pickle::Tuple(tuple), {
    if tuple.len() == 6 {
        Ok((tuple[0].clone(), tuple[1].clone(), tuple[2].clone(), tuple[3].clone(), tuple[4].clone(), tuple[5].clone()))
    } else {
        Err(PickleError::CannotTryInto)
    }
});
pickle_try_into!((Pickle, Pickle, Pickle, Pickle, Pickle, Pickle, Pickle), Pickle::Tuple(tuple), {
    if tuple.len() == 7 {
        Ok((tuple[0].clone(), tuple[1].clone(), tuple[2].clone(), tuple[3].clone(), tuple[4].clone(), tuple[5].clone(), tuple[6].clone()))
    } else {
        Err(PickleError::CannotTryInto)
    }
});



pickle_try_into!(PickleModule, Pickle::Module(module), { Ok(module) });
pickle_try_into!(PickleClass, Pickle::Class(class), { Ok(class) });


