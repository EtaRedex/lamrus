use lamrus::parser::{lex, Parser};
use lamrus::eval::krivine::{Closure, eval};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

fn main() {
    let y = "(λx. (λy. y x)) 5 9";
    let lexed = lex(y.into());
    let mut parsed = Parser::new(lexed);
    let outputed = parsed.parse();
    //println!("{:#?}", outputed.clone());
    let cls = Rc::new(Closure {
        code: Rc::new(outputed),
        env: Rc::new(RefCell::new(HashMap::new()))
    });
    let out_kr = eval(cls);
    println!("{:#?}", out_kr);
}