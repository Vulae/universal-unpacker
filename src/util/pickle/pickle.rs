
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
    BigInt(Vec<u8>),
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

macro_rules! pickle_try_into_int {
    ($type:ty) => {
        pickle_try_into!(
            $type,
            Pickle::Number(num),
            {
                match(num) {
                    PickleNumber::Uint(v) => v.try_into().map_err(|_| PickleError::CannotTryInto),
                    PickleNumber::Int(v) => v.try_into().map_err(|_| PickleError::CannotTryInto),
                    _ => Err(PickleError::CannotTryInto)
                }
            }
        );
    };
}
pickle_try_into_int!(u8);
pickle_try_into_int!(u16);
pickle_try_into_int!(u32);
pickle_try_into_int!(u64);
pickle_try_into_int!(i8);
pickle_try_into_int!(i16);
pickle_try_into_int!(i32);
pickle_try_into_int!(i64);
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





macro_rules! pickle_from {
    ($type:ty, $var:ident, $inner:block) => {
        impl From<$type> for Pickle {
            fn from($var: $type) -> Pickle {
                $inner
            }
        }
    };
}



pickle_from!(bool, bool, { Pickle::Bool(bool) });

pickle_from!(u8, v, { Pickle::Number(PickleNumber::Uint(v.into())) });
pickle_from!(u16, v, { Pickle::Number(PickleNumber::Uint(v.into())) });
pickle_from!(u32, v, { Pickle::Number(PickleNumber::Uint(v.into())) });
pickle_from!(u64, v, { Pickle::Number(PickleNumber::Uint(v.into())) });
pickle_from!(i8, v, { Pickle::Number(PickleNumber::Int(v.into())) });
pickle_from!(i16, v, { Pickle::Number(PickleNumber::Int(v.into())) });
pickle_from!(i32, v, { Pickle::Number(PickleNumber::Int(v.into())) });
pickle_from!(i64, v, { Pickle::Number(PickleNumber::Int(v.into())) });
pickle_from!(f32, v, { Pickle::Number(PickleNumber::Float(v.into())) });
pickle_from!(f64, v, { Pickle::Number(PickleNumber::Float(v.into())) });

pickle_from!(String, v, { Pickle::String(v) });
pickle_from!(&str, str, { Pickle::String(str.to_owned()) });

pickle_from!(Vec<Pickle>, vec, { Pickle::List(vec) });

pickle_from!(HashMap<String, Pickle>, dict, { Pickle::Dict(dict) });
pickle_from!(HashMap<&str, Pickle>, dict, {
    let mut new = HashMap::new();
    dict.into_iter()
        .for_each(|(key, value)| {
            new.insert(key.to_owned(), value);
        });
    Pickle::Dict(new)
});





#[macro_export]
macro_rules! pickle_extract {
    (
        $pickle:expr;
        ($pattern_fin:pat, $value_fin:expr)
    ) => ({
        match $pickle {
            $pattern_fin => Ok($value_fin.clone()),
            _ => Err(Box::new(crate::util::pickle::error::PickleError::CannotExtract)),
        }
    });
    (
        $pickle:expr;
        $(($pattern:pat, $value:expr)),*;
        ($pattern_fin:pat, $value_fin:expr)
    ) => ('extract: {
        let mut pickle = $pickle.clone();
        $(
            pickle = match pickle {
                $pattern => $value.clone(),
                _ => break 'extract Err(Box::new(crate::util::pickle::error::PickleError::CannotExtract)),
            };
        )*
        pickle_extract!(pickle; ($pattern_fin, $value_fin))
    });
}





#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::hashmap;
    use super::Pickle;

    #[test]
    fn pickle_extract() -> Result<(), Box<dyn Error>> {
        let pickle = Pickle::List(vec![
            Pickle::from(42),
            Pickle::from("Hello, World!"),
            Pickle::from(hashmap![
                ("key1", Pickle::from("Hiii")),
            ]),
        ]);
        assert_eq!(
            pickle_extract!(pickle; 
                (Pickle::List(arr), arr[1]);
                (Pickle::String(str), str)
            )?,
            "Hello, World!"
        );
        assert_eq!(
            pickle_extract!(pickle; 
                (Pickle::List(arr), arr[2]),
                (Pickle::Dict(dict), dict.get("key1").unwrap());
                (Pickle::String(str), str)
            )?,
            "Hiii"
        );
        Ok(())
    }
}


