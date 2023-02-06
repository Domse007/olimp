use std::borrow::Borrow;

use crate::interpreter::lispobject::LispObject;

pub struct TypeInfo {
    return_type: LispObject,
    params_type: Vec<LispObject>,
}

impl TypeInfo {
    pub fn new(params_type: &[LispObject], return_type: LispObject) -> Self {
        Self {
            return_type,
            params_type: params_type.into(),
        }
    }

    pub fn match_return_type(&self, other: LispObject) -> bool {
        match (self.return_type.borrow(), other) {
            (LispObject::Number(_), LispObject::Number(_)) => true,
            _ => false,
        }
    }
}

#[test]
fn test_match_return_number() {
    let test = TypeInfo::new(&[], LispObject::Number(1.));
    let other = LispObject::number(2.);
    assert!(test.match_return_type(other));
}
