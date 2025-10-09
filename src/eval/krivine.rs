use crate::parser::AstNode;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


type Env = RefCell<HashMap<String, Rc<Closure>>>;

#[derive(Clone, Debug)]
pub struct Closure {
    pub code: Rc<AstNode>,
    pub env: Rc<Env>,
}

pub fn krivine(
    closure: Rc<Closure>,
    stack: &mut Vec<Rc<Closure>>,
) -> Rc<Closure> {
    match &*closure.code {
        AstNode::Var(name) => {
            if let Some(v) = closure.env.borrow().get(name) {
                Rc::clone(v)
            } else {
                closure
            }
        }
        AstNode::App(fn_expr, arg_expr) => {
            stack.push(Rc::new(Closure {
                code: Rc::clone(arg_expr),
                env: Rc::clone(&closure.env),
            }));
            Rc::new(Closure {
                code: Rc::clone(fn_expr),
                env: Rc::clone(&closure.env),
            })
        }
        AstNode::Abs(param, body) => {
            if stack.is_empty() {
                closure
            } else {
                let arg_closure = stack.pop().unwrap();
                let new_env = Rc::new(RefCell::new(closure.env.borrow().clone()));
                new_env.borrow_mut().insert(param.clone(), arg_closure);
                Rc::new(Closure {
                    code: Rc::clone(body),
                    env: new_env,
                })
            }
        }
    }
}
pub fn eval(mut closure: Rc<Closure>) -> (Rc<Closure>, Vec<Rc<Closure>>) {
    let mut stack: Vec<Rc<Closure>> = Vec::new();

    loop {
        let new_closure = krivine(Rc::clone(&closure), &mut stack);
        if Rc::ptr_eq(&closure, &new_closure) {
            break;
        }
        closure = new_closure;
    }

    (closure, stack)
}

