use super::utils::{read_to_f64, read_to_u16};

const NUMBER: TypeHint = 0;
const STRING: TypeHint = 1;
const SYMBOL: TypeHint = 2;
const CONS: TypeHint = 3;
const LIST: TypeHint = 4;
const NIL: TypeHint = 5;

type SizeIdent = u16;
type TypeHint = u8;

#[derive(Clone, Debug, PartialEq)]
pub enum LispObject {
    Number(f64),
    String(String),
    Symbol(String),
    Cons(Box<(Self, Self)>),
    List(Vec<Self>),
    Nil,
}

impl LispObject {
    pub fn number(num: f64) -> Self {
        Self::Number(num)
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        match self {
            Self::Number(data) => {
                let mut vec = vec![NUMBER];
                vec.extend(data.to_ne_bytes());
                vec
            }
            Self::String(data) => {
                let data = data.as_bytes();
                let size = data.len() as SizeIdent;
                let mut vec = vec![STRING];
                assert!(data.len() <= SizeIdent::MAX as usize);
                vec.extend_from_slice(&size.to_ne_bytes());
                vec.extend_from_slice(data);
                vec
            }
            Self::Symbol(data) => {
                let data = data.as_bytes();
                let size = data.len() as SizeIdent;
                assert!(data.len() <= SizeIdent::MAX as usize);
                let mut vec = vec![SYMBOL];
                vec.extend_from_slice(&size.to_ne_bytes());
                vec.extend_from_slice(data);
                vec
            }
            Self::Cons(data) => {
                let mut vec = vec![CONS];
                vec.extend(data.0.into_bytes());
                vec.extend(data.1.into_bytes());
                vec
            }
            Self::List(data) => {
                let mut vec = vec![LIST];
                let size = data.len() as SizeIdent;
                assert!(data.len() <= SizeIdent::MAX as usize);
                vec.extend_from_slice(&size.to_ne_bytes());
                for entry in data {
                    vec.extend(entry.into_bytes());
                }
                vec
            }
            Self::Nil => vec![NIL],
        }
    }

    pub fn build<T: Iterator<Item = u8>>(data: &mut T) -> LispObject {
        let ident = data.next().unwrap();
        match ident {
            NUMBER => LispObject::Number(read_to_f64(data)),
            STRING => {
                let size: SizeIdent = read_to_u16(data);
                let str_bytes = data.take(size as usize).collect::<Vec<u8>>();
                LispObject::String(String::from_utf8(str_bytes).unwrap())
            }
            SYMBOL => {
                let size: SizeIdent = read_to_u16(data);
                let str_bytes = data.take(size as usize).collect::<Vec<u8>>();
                LispObject::Symbol(String::from_utf8(str_bytes).unwrap())
            }
            CONS => LispObject::Cons(Box::new((LispObject::build(data), LispObject::build(data)))),
            LIST => {
                let size = read_to_u16(data);
                let mut vec = Vec::new();
                for _ in 0..size {
                    vec.push(LispObject::build(data));
                }
                LispObject::List(vec)
            }
            num => panic!("{num} is not a valid identifier."),
        }
    }
}

#[test]
fn test_number_conversion() {
    let obj = LispObject::Number(143.234);
    let bytes = obj.into_bytes();
    let reassembled = LispObject::build(&mut bytes.into_iter());
    assert_eq!(reassembled, obj);
}

#[test]
fn test_string_conversion() {
    let obj = LispObject::String("Hello World!".to_string());
    let bytes = obj.into_bytes();
    let reassembled = LispObject::build(&mut bytes.into_iter());
    assert_eq!(reassembled, obj);
}

#[test]
fn test_symbol_conversion() {
    let obj = LispObject::Symbol("do-something".to_string());
    let bytes = obj.into_bytes();
    let reassembled = LispObject::build(&mut bytes.into_iter());
    assert_eq!(reassembled, obj);
}

#[test]
fn test_cons_conversion() {
    let obj = LispObject::Cons(Box::new((
        LispObject::String("Hello World!".to_string()),
        LispObject::Number(23.2),
    )));
    let bytes = obj.into_bytes();
    let reassembled = LispObject::build(&mut bytes.into_iter());
    assert_eq!(reassembled, obj);
}

#[test]
fn test_list_conversion() {
    let obj = LispObject::List(vec![
        LispObject::String("This.".to_string()),
        LispObject::Number(12.1),
        LispObject::Cons(Box::new((
            LispObject::Number(12.),
            LispObject::Symbol("do".to_string()),
        ))),
    ]);
    let bytes = obj.into_bytes();
    let reassembled = LispObject::build(&mut bytes.into_iter());
    assert_eq!(reassembled, obj);
}
