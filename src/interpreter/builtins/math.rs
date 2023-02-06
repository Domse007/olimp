use crate::compiler::type_info::TypeInfo;
use crate::fn_collector::FnCollector;
use crate::interpreter::lispobject::LispObject;
use crate::interpreter::stackframe::Stackframe;

pub fn include_math(collector: &mut FnCollector) {
    collector.add_fn(
        "+",
        add,
        TypeInfo::new(
            &[LispObject::number(1.), LispObject::number(2.)],
            LispObject::number(1.),
        ),
    );
    collector.add_fn(
        "-",
        sub,
        TypeInfo::new(
            &[LispObject::number(1.), LispObject::number(2.)],
            LispObject::number(1.),
        ),
    );
}

pub fn add(stack: &mut Stackframe) {
    match (stack.pop(), stack.pop()) {
        (LispObject::Number(num1), LispObject::Number(num2)) => {
            stack.push(LispObject::number(num1 + num2))
        }
        _ => panic!("Wrong type argument."),
    }
}

#[test]
fn test_add() {
    let mut stack = Stackframe::new();
    stack.push(LispObject::number(1.));
    stack.push(LispObject::number(2.));
    add(&mut stack);
    assert_eq!(stack.pop(), LispObject::number(3.));
}

pub fn sub(stack: &mut Stackframe) {
    match (stack.pop(), stack.pop()) {
        (LispObject::Number(num1), LispObject::Number(num2)) => {
            stack.push(LispObject::number(num2 - num1))
        }
        _ => panic!("Wrong type argument."),
    }
}

#[test]
fn test_sub() {
    let mut stack = Stackframe::new();
    stack.push(LispObject::number(2.));

    stack.push(LispObject::number(1.));
    sub(&mut stack);
    assert_eq!(stack.pop(), LispObject::number(1.));
}
