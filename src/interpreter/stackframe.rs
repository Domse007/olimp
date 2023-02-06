use super::lispobject::LispObject;

pub struct Stackframe {
    local_stack: Vec<LispObject>,
}

impl Stackframe {
    pub fn new() -> Self {
        Self {
            local_stack: vec![],
        }
    }

    pub fn push(&mut self, elem: LispObject) {
        self.local_stack.push(elem);
    }

    pub fn pop(&mut self) -> LispObject {
        self.local_stack
            .pop()
            .expect("Stack error. Missing element.")
    }

    pub fn rel_clone(&mut self, rel: usize) -> LispObject {
        self.local_stack
            .get(self.local_stack.len() - 1 - rel)
            .expect("Stack error. Missing element.")
            .clone()
    }

    #[cfg(test)]
    pub fn size(&self) -> usize {
        self.local_stack.len()
    }
}

#[test]
fn test_stackframe() {
    let mut test = Stackframe::new();
    test.push(LispObject::number(1.));
    test.push(LispObject::number(2.));
    test.push(LispObject::number(3.));
    assert_eq!(test.size(), 3);
    let res = test.rel_clone(1);
    assert_eq!(res, LispObject::number(2.));
    let _ = test.pop();
    assert_eq!(test.size(), 2);
}
