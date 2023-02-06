const NUMBER: u8 = 0;
const STRING: u8 = 1;
const SYMBOL: u8 = 2;
const CONS: u8 = 3;
const LIST: u8 = 4;

type SizeIdent = u16;

#[derive(Clone, Debug, PartialEq)]
pub enum LispObject {
    Number(f64),
    String(String),
    Symbol(String),
    Cons(Box<(Self, Self)>),
    List(Vec<Self>),
}

impl LispObject {
    pub fn number(num: f64) -> Self {
        Self::Number(num)
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        match self {
            Self::Number(data) => {
                const SIZE: u8 = std::mem::size_of::<f64>() as u8;
                let mut vec = vec![NUMBER, SIZE];
                vec.extend(data.to_le_bytes());
                vec
            }
            Self::String(data) => {
                let data = data.as_bytes();
                let size = data.len() as SizeIdent;
                let mut vec = vec![STRING];
                assert!(data.len() <= SizeIdent::MAX as usize);
                vec.extend_from_slice(&size.to_le_bytes());
                vec.extend_from_slice(data);
                vec
            }
            Self::Symbol(data) => {
                let data = data.as_bytes();
                let size = data.len() as SizeIdent;
                assert!(data.len() <= SizeIdent::MAX as usize);
                let mut vec = vec![SYMBOL];
                vec.extend_from_slice(&size.to_le_bytes());
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
                vec.extend_from_slice(&size.to_le_bytes());
                for entry in data {
                    vec.extend(entry.into_bytes());
                }
                vec
            }
        }
    }
}
