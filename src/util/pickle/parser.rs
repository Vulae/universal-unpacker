
// TODO: Implement all opcodes.

use std::{collections::HashMap, error::Error, io::Read};
use crate::util::read_ext::ReadExt;

use super::{error::PickleError, pickle::{Pickle, PickleClass, PickleModule, PickleNumber}};



#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum PickleOpcode {
    MARK = 0x28, // push special markobject on stack
    STOP = 0x2E, // every pickle ends with STOP
    POP = 0x30, // discard topmost stack item
    POP_MARK = 0x31, // discard stack top through topmost markobject
    DUP = 0x32, // duplicate top stack item
    FLOAT = 0x46, // push float object; decimal string argument
    INT = 0x49, // push integer or bool; decimal string argument
    BININT = 0x4A, // push four-byte signed int
    BININT1 = 0x4B, // push 1-byte unsigned int
    LONG = 0x4C, // push long; decimal string argument
    BININT2 = 0x4D, // push 2-byte unsigned int
    NONE = 0x4E, // push None
    PERSID = 0x50, // push persistent object; id is taken from string arg
    BINPERSID = 0x51, //"       "         "  ;  "  "   "     "  stack
    REDUCE = 0x52, // apply callable to argtuple, both on stack
    STRING = 0x53, // push string; NL-terminated string argument
    BINSTRING = 0x54, // push string; counted binary string argument
    SHORT_BINSTRING = 0x55, //"     "   ;    "      "       "      " < 256 bytes
    UNICODE = 0x56, // push Unicode string; raw-unicode-escaped'd argument
    BINUNICODE = 0x58, // "     "       "  ; counted UTF-8 string argument
    APPEND = 0x61, // append stack top to list below it
    BUILD = 0x62, // call __setstate__ or __dict__.update()
    GLOBAL = 0x63, // push self.find_class(modname, name); 2 string args
    DICT = 0x64, // build a dict from stack items
    EMPTY_DICT = 0x7D, // push empty dict
    APPENDS = 0x65, // extend list on stack by topmost stack slice
    GET = 0x67, // push item from memo on stack; index is string arg
    BINGET = 0x68, // "    "    "    "   "   "  ;   "    " 1-byte arg
    INST = 0x69, // build & push class instance
    LONG_BINGET = 0x6A, // push item from memo on stack; index is 4-byte arg
    LIST = 0x6C, // build list from topmost stack items
    EMPTY_LIST = 0x5D, // push empty list
    OBJ = 0x6F, // build & push class instance
    PUT = 0x70, // store stack top in memo; index is string arg
    BINPUT = 0x71, // "     "    "   "   " ;   "    " 1-byte arg
    LONG_BINPUT = 0x72, // "     "    "   "   " ;   "    " 4-byte arg
    SETITEM = 0x73, // add key+value pair to dict
    TUPLE = 0x74, // build tuple from topmost stack items
    EMPTY_TUPLE = 0x29, // push empty tuple
    SETITEMS = 0x75, // modify dict by adding topmost key+value pairs
    BINFLOAT = 0x47, // push float; arg is 8-byte float encoding
    //  Protocol 2
    PROTO = 0x80, // identify pickle protocol
    NEWOBJ = 0x81, // build object by applying cls.__new__ to argtuple
    EXT1 = 0x82, // push object from extension registry; 1-byte index
    EXT2 = 0x83, // ditto, but 2-byte index
    EXT4 = 0x84, // ditto, but 4-byte index
    TUPLE1 = 0x85, // build 1-tuple from stack top
    TUPLE2 = 0x86, // build 2-tuple from two topmost stack items
    TUPLE3 = 0x87, // build 3-tuple from three topmost stack items
    NEWTRUE = 0x88, // push True
    NEWFALSE = 0x89, // push False
    LONG1 = 0x8a, // push long from < 256 bytes
    LONG4 = 0x8b, // push really big long
    //  Protocol 3 (Python 3.x)
    BINBYTES = 0x42, // push bytes; counted binary string argument
    SHORT_BINBYTES = 0x43, //"     "   ;    "      "       "      " < 256 bytes
    //  Protocol 4
    SHORT_BINUNICODE = 0x8c, // push short string; UTF-8 length < 256 bytes
    BINUNICODE8 = 0x8d, // push very long string
    BINBYTES8 = 0x8e, // push very long bytes string
    EMPTY_SET = 0x8f, // push empty set on the stack
    ADDITEMS = 0x90, // modify set by adding topmost stack items
    FROZENSET = 0x91, // build frozenset from topmost stack items
    NEWOBJ_EX = 0x92, // like NEWOBJ but work with keyword only arguments
    STACK_GLOBAL = 0x93, // same as GLOBAL but using names on the stacks
    MEMOIZE = 0x94, // store top of the stack in memo
    FRAME = 0x95, // indicate the beginning of a new frame
    //  Protocol 5
    BYTEARRAY8 = 0x96, // push bytearray
    NEXT_BUFFER = 0x97, // push next out-of-band buffer
    READONLY_BUFFER = 0x98, // make top of stack readonly
}

impl PickleOpcode {
    fn new(value: u8) -> Result<PickleOpcode, Box<dyn Error>> {
        Ok(match value {
            0x28 => PickleOpcode::MARK,
            0x2E => PickleOpcode::STOP,
            0x30 => PickleOpcode::POP,
            0x31 => PickleOpcode::POP_MARK,
            0x32 => PickleOpcode::DUP,
            0x46 => PickleOpcode::FLOAT,
            0x49 => PickleOpcode::INT,
            0x4A => PickleOpcode::BININT,
            0x4B => PickleOpcode::BININT1,
            0x4C => PickleOpcode::LONG,
            0x4D => PickleOpcode::BININT2,
            0x4E => PickleOpcode::NONE,
            0x50 => PickleOpcode::PERSID,
            0x51 => PickleOpcode::BINPERSID,
            0x52 => PickleOpcode::REDUCE,
            0x53 => PickleOpcode::STRING,
            0x54 => PickleOpcode::BINSTRING,
            0x55 => PickleOpcode::SHORT_BINSTRING,
            0x56 => PickleOpcode::UNICODE,
            0x58 => PickleOpcode::BINUNICODE,
            0x61 => PickleOpcode::APPEND,
            0x62 => PickleOpcode::BUILD,
            0x63 => PickleOpcode::GLOBAL,
            0x64 => PickleOpcode::DICT,
            0x7D => PickleOpcode::EMPTY_DICT,
            0x65 => PickleOpcode::APPENDS,
            0x67 => PickleOpcode::GET,
            0x68 => PickleOpcode::BINGET,
            0x69 => PickleOpcode::INST,
            0x6A => PickleOpcode::LONG_BINGET,
            0x6C => PickleOpcode::LIST,
            0x5D => PickleOpcode::EMPTY_LIST,
            0x6F => PickleOpcode::OBJ,
            0x70 => PickleOpcode::PUT,
            0x71 => PickleOpcode::BINPUT,
            0x72 => PickleOpcode::LONG_BINPUT,
            0x73 => PickleOpcode::SETITEM,
            0x74 => PickleOpcode::TUPLE,
            0x29 => PickleOpcode::EMPTY_TUPLE,
            0x75 => PickleOpcode::SETITEMS,
            0x47 => PickleOpcode::BINFLOAT,
            0x80 => PickleOpcode::PROTO,
            0x81 => PickleOpcode::NEWOBJ,
            0x82 => PickleOpcode::EXT1,
            0x83 => PickleOpcode::EXT2,
            0x84 => PickleOpcode::EXT4,
            0x85 => PickleOpcode::TUPLE1,
            0x86 => PickleOpcode::TUPLE2,
            0x87 => PickleOpcode::TUPLE3,
            0x88 => PickleOpcode::NEWTRUE,
            0x89 => PickleOpcode::NEWFALSE,
            0x8a => PickleOpcode::LONG1,
            0x8b => PickleOpcode::LONG4,
            0x42 => PickleOpcode::BINBYTES,
            0x43 => PickleOpcode::SHORT_BINBYTES,
            0x8c => PickleOpcode::SHORT_BINUNICODE,
            0x8d => PickleOpcode::BINUNICODE8,
            0x8e => PickleOpcode::BINBYTES8,
            0x8f => PickleOpcode::EMPTY_SET,
            0x90 => PickleOpcode::ADDITEMS,
            0x91 => PickleOpcode::FROZENSET,
            0x92 => PickleOpcode::NEWOBJ_EX,
            0x93 => PickleOpcode::STACK_GLOBAL,
            0x94 => PickleOpcode::MEMOIZE,
            0x95 => PickleOpcode::FRAME,
            0x96 => PickleOpcode::BYTEARRAY8,
            0x97 => PickleOpcode::NEXT_BUFFER,
            0x98 => PickleOpcode::READONLY_BUFFER,
            _ => return Err(Box::new(PickleError::InvalidOperation(value))),
        })
    }
}



#[derive(Debug)]
struct PickleStack {
    stack: Vec<PickleStackItem>,
}

impl PickleStack {

    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, item: Pickle) {
        self.stack.push(PickleStackItem::Pickle(item));
    }

    pub fn pop(&mut self) -> Result<Pickle, Box<dyn Error>> {
        match self.stack.pop() {
            Some(PickleStackItem::Pickle(item)) => Ok(item),
            Some(PickleStackItem::Mark) => Err(Box::new(PickleError::StackMark)),
            None => Err(Box::new(PickleError::StackEmpty)),
        }
    }

    pub fn last(&mut self) -> Result<&Pickle, Box<dyn Error>> {
        match self.stack.last() {
            Some(PickleStackItem::Pickle(item)) => Ok(item),
            Some(PickleStackItem::Mark) => Err(Box::new(PickleError::StackMark)),
            None => Err(Box::new(PickleError::StackEmpty)),
        }
    }

    pub fn set_last(&mut self, item: Pickle) -> Result<Pickle, Box<dyn Error>> {
        let removed_item = self.pop()?;
        self.push(item);
        Ok(removed_item)
    }

    pub fn push_mark(&mut self) {
        self.stack.push(PickleStackItem::Mark);
    }

    pub fn pop_mark(&mut self) -> Result<Vec<Pickle>, Box<dyn Error>> {
        let mut items: Vec<Pickle> = Vec::new();

        loop {
            match self.stack.pop() {
                Some(PickleStackItem::Pickle(item)) => items.insert(0, item),
                Some(PickleStackItem::Mark) => break,
                None => return Err(Box::new(PickleError::StackEmpty)),
            }
        }

        Ok(items)
    }

}

#[derive(Debug, Clone)]
enum PickleStackItem {
    Pickle(Pickle),
    Mark,
}



#[derive(Debug)]
struct PickleMemo {
    items: Vec<PickleMemoItem>,
}

impl PickleMemo {

    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn get(&mut self, index: usize) -> Result<&Pickle, Box<dyn Error>> {
        match self.items.get(index) {
            Some(PickleMemoItem::Pickle(item)) => Ok(item),
            Some(PickleMemoItem::Empty) => Err(Box::new(PickleError::MemoIndexEmpty)),
            None => Err(Box::new(PickleError::MemoIndexOutOfBounds)),
        }
    }

    pub fn set(&mut self, index: usize, value: Pickle) -> Result<(), Box<dyn Error>> {
        while index >= self.items.len() {
            self.items.push(PickleMemoItem::Empty);
        }

        self.items[index] = PickleMemoItem::Pickle(value);

        Ok(())
    }

    pub fn push(&mut self, value: Pickle) -> usize {
        self.items.push(PickleMemoItem::Pickle(value));
        self.items.len()
    }

    pub fn last(&mut self) -> Result<&Pickle, Box<dyn Error>> {
        match self.items.last() {
            // Last item should NEVER be empty.
            Some(PickleMemoItem::Pickle(item)) => Ok(item),
            Some(PickleMemoItem::Empty) => Err(Box::new(PickleError::MemoEmpty)),
            None => Err(Box::new(PickleError::MemoEmpty)),
        }
    }

}

#[derive(Debug, Clone)]
enum PickleMemoItem {
    Pickle(Pickle),
    Empty,
}





#[derive(Debug, PartialEq, Eq)]
pub enum PickleProtocol {
    Unknown,
    Protocol1,
    Protocol2,
    Protocol3,
    Protocol4,
    Protocol5,
}

impl PickleProtocol {

    pub fn from(value: u8) -> Result<Self, Box<dyn Error>> {
        match value {
            1 => Err(Box::new(PickleError::UnsupportedProtocol(PickleProtocol::Protocol1))),
            2 => Ok(PickleProtocol::Protocol2),
            3 => Ok(PickleProtocol::Protocol3),
            4 => Ok(PickleProtocol::Protocol4),
            5 => Ok(PickleProtocol::Protocol5),
            v => Err(Box::new(PickleError::UnknownProtocol(v))),
        }
    }

}





#[derive(Debug)]
pub struct PickleParser {
    protocol: PickleProtocol,
    stack: PickleStack,
    memo: PickleMemo,
}

impl PickleParser {

    pub fn new() -> Self {
        Self {
            protocol: PickleProtocol::Unknown,
            stack: PickleStack::new(),
            memo: PickleMemo::new(),
        }
    }

    fn read_operation(&mut self, data: &mut impl Read) -> Result<PickleOpcode, Box<dyn Error>> {
        let opcode: PickleOpcode = PickleOpcode::new(data.read_primitive()?)?;

        // println!("Opcode: {:#?}", opcode);

        // First opcode MUST be protocol.
        if self.protocol == PickleProtocol::Unknown {
            if opcode == PickleOpcode::PROTO {
                self.protocol = PickleProtocol::from(data.read_primitive()?)?;
                return Ok(opcode);
            } else {
                return Err(Box::new(PickleError::PickleInvalidProtocolOperation));
            }
        }

        // TODO: Sort these.
        match opcode {
            PickleOpcode::PROTO => return Err(Box::new(PickleError::PickleInvalidProtocolOperation)),
            PickleOpcode::FRAME => { data.read_primitive::<u64>()?; }, // Used to indicate how many bytes to preload.
            PickleOpcode::EMPTY_DICT => { self.stack.push(Pickle::Dict(HashMap::new())); }
            // TODO: This may break as they need to be the same item, not cloned.
            PickleOpcode::MEMOIZE => { self.memo.push(self.stack.last()?.clone()); }
            PickleOpcode::SHORT_BINUNICODE => { self.stack.push(Pickle::String(data.read_string::<u8>()?)); },
            PickleOpcode::EMPTY_LIST => { self.stack.push(Pickle::List(Vec::new())); },
            PickleOpcode::BININT => { self.stack.push(Pickle::Number(PickleNumber::Int(data.read_primitive::<i32>()?.into()))) },
            PickleOpcode::SHORT_BINBYTES => {
                let len: u8 = data.read_primitive()?;
                self.stack.push(Pickle::Binary(data.read_to_vec(len as usize)?));
            },
            PickleOpcode::TUPLE3 => {
                let items = (self.stack.pop()?, self.stack.pop()?, self.stack.pop()?);
                self.stack.push(Pickle::Tuple(vec![items.2, items.1, items.0]));
            },
            PickleOpcode::APPEND => {
                let item = self.stack.pop()?;
                let mut list = TryInto::<Vec<Pickle>>::try_into(self.stack.pop()?)?;
                list.push(item);
                self.stack.push(Pickle::List(list));
            },
            PickleOpcode::SETITEM => {
                let item = self.stack.pop()?;
                let key = TryInto::<String>::try_into(self.stack.pop()?)?;
                let mut dict = TryInto::<HashMap<String, Pickle>>::try_into(self.stack.pop()?)?;
                dict.insert(key, item);
                self.stack.push(Pickle::Dict(dict));
            },
            PickleOpcode::STOP => { },
            PickleOpcode::MARK => {
                self.stack.push_mark();
            },
            PickleOpcode::BINGET => {
                let index: u8 = data.read_primitive()?;
                let item = self.memo.get(index as usize)?;
                self.stack.push(item.clone());
            },
            PickleOpcode::SETITEMS => {
                let mut items = self.stack.pop_mark()?;
                let mut dict = self.stack.pop()?;
                while items.len() > 0 {
                    let value = items.pop().unwrap();
                    let key = TryInto::<String>::try_into(items.pop().unwrap())?;
                    match dict {
                        Pickle::Dict(ref mut dict) => dict.insert(key, value),
                        Pickle::Class(ref mut class) => class.data.insert(key, value),
                        _ => return Err(Box::new(PickleError::CannotTryInto)),
                    };
                }
                self.stack.push(dict);
            },
            // TODO: Don't clone, Refactor PickleStack & PickleMemo to use pointers to pickle.
            PickleOpcode::BINPUT => { self.memo.set(data.read_primitive::<u8>()? as usize, self.stack.last()?.clone())?; },
            PickleOpcode::BINUNICODE => { self.stack.push(Pickle::String(data.read_string::<u32>()?)); }
            PickleOpcode::GLOBAL => { self.stack.push(Pickle::Module(PickleModule::new(data.read_terminated_string(0x0A)?, data.read_terminated_string(0x0A)?))) }
            PickleOpcode::TUPLE1 => {
                let item = self.stack.pop()?;
                self.stack.push(Pickle::Tuple(vec![item]));
            },
            // TODO: These aren't the same are they?
            PickleOpcode::REDUCE | PickleOpcode::NEWOBJ => {
                let args = self.stack.pop()?;
                let mut module = TryInto::<PickleModule>::try_into(self.stack.pop()?)?;
                let class = module.class(args);
                self.stack.push(Pickle::Class(class));
            },
            PickleOpcode::EMPTY_TUPLE => { self.stack.push(Pickle::Tuple(Vec::new())); },
            PickleOpcode::NONE => { self.stack.push(Pickle::None); }
            PickleOpcode::TUPLE2 => {
                let items = (self.stack.pop()?, self.stack.pop()?);
                self.stack.push(Pickle::Tuple(vec![items.1, items.0]));
            },
            PickleOpcode::BININT1 => { self.stack.push(Pickle::Number(PickleNumber::Uint(data.read_primitive::<u8>()?.into()))); },
            PickleOpcode::TUPLE => {
                let mut items = self.stack.pop_mark()?;
                items.reverse();
                self.stack.push(Pickle::Tuple(items));
            },
            PickleOpcode::BUILD => {
                let state = self.stack.pop()?;
                let mut class = TryInto::<PickleClass>::try_into(self.stack.pop()?)?;
                class.state = Some(Box::new(state));
                self.stack.push(Pickle::Class(class));
            },
            PickleOpcode::NEWFALSE => { self.stack.push(Pickle::Bool(false)); }
            PickleOpcode::NEWTRUE => { self.stack.push(Pickle::Bool(true)); }
            // TODO: Don't clone, Refactor PickleStack & PickleMemo to use pointers to pickle.
            PickleOpcode::LONG_BINPUT => { self.memo.set(data.read_primitive::<u32>()? as usize, self.stack.last()?.clone())?; },
            // TODO: Don't clone, Refactor PickleStack & PickleMemo to use pointers to pickle.
            PickleOpcode::LONG_BINGET => { self.stack.push(self.memo.get(data.read_primitive::<u32>()? as usize)?.clone()) }
            PickleOpcode::APPENDS => {
                let mut items = self.stack.pop_mark()?;
                let mut list = TryInto::<Vec<Pickle>>::try_into(self.stack.pop()?)?;
                list.append(&mut items);
                self.stack.push(Pickle::List(list));
            },
            opcode => return Err(Box::new(PickleError::UnsupportedOperation(opcode))),
        }

        Ok(opcode)
    }

    pub fn read(&mut self, data: &mut impl Read) -> Result<(), Box<dyn Error>> {
        // TODO: Move protocol version read to here.
        while self.read_operation(data)? != PickleOpcode::STOP { }
        Ok(())
    }

    pub fn parse(data: &mut impl Read) -> Result<Pickle, Box<dyn Error>> {
        let mut reader = PickleParser::new();
        reader.read(data)?;
        Ok(reader.stack.pop()?)
    }

}


