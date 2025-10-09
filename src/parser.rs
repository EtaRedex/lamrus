use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Var(String),
    Abs(String, Rc<AstNode>),
    App(Rc<AstNode>, Rc<AstNode>)
}



#[derive(Debug, Clone, PartialEq)]
pub enum LexItems {
    AbsStart, AbsEnd,
    OParen, CParen,
    Var(String),
}

pub fn lex(src: String) -> Vec<LexItems> {
    let mut output = Vec::new();
    let mut chars = src.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '\\' | 'λ' => { output.push(LexItems::AbsStart); chars.next(); }
            '.'  => { output.push(LexItems::AbsEnd); chars.next(); }
            '('  => { output.push(LexItems::OParen); chars.next(); }
            ')'  => { output.push(LexItems::CParen); chars.next(); }
            c if c.is_whitespace() => { chars.next(); }
            c if c.is_alphanumeric() => {
                let mut ident = String::new();
                while let Some(&c2) = chars.peek() {
                    if c2.is_alphanumeric() { ident.push(c2); chars.next(); }
                    else { break; }
                }
                output.push(LexItems::Var(ident));
            }
            _ => panic!("Unexpected character: {}", ch),
        }
    }
    output
}


pub struct Parser {
    items: Vec<LexItems>,
    pos:usize,
}

impl Parser {
    pub fn new(src: Vec<LexItems>) -> Self {
        Self {
            items: src,
            pos: 0
        }
    }
    pub fn peek(&self) -> Option<&LexItems> {
        self.items.get(self.pos)
    }
    pub fn advance_n(&mut self, n: usize) {
        self.pos = (self.pos + n).min(self.items.len());
    }
    pub fn advance(&mut self) {
        self.advance_n(1);
    }

    pub fn parse(&mut self) -> AstNode {
        match self.peek() {
            Some(LexItems::AbsStart) => self.parse_abstraction(),
            Some(LexItems::Var(_)) | Some(LexItems::OParen) => self.parse_application(),
            Some(LexItems::AbsEnd) => panic!("Unexpected end of abstraction"),
            _ => panic!("Unexpected end of input"),
        }
    }


    pub fn parse_abstraction(&mut self) -> AstNode {
        assert_eq!(self.peek(), Some(&LexItems::AbsStart));
        self.advance();
        let mut args = Vec::<String>::new();
        while let Some(item) = self.peek() {
            match item {
                LexItems::AbsEnd => break,
                LexItems::AbsStart => self.advance(),
                LexItems::Var(s) => args.push(s.clone()),
                s =>{ println!("{:#?}", s); panic!("Parsing error in abstaction arguments...")}
            }
            self.advance();
        };
        assert_eq!(self.peek(), Some(&LexItems::AbsEnd));
        self.advance();
        let body = self.parse();
        let mut term = body;
        for arg in args.iter().rev() {
            term = AstNode::Abs(arg.clone(), Rc::new(term));
        }
        term
    }
    pub fn parse_application(&mut self) -> AstNode {
        let mut term = match self.peek() {
            Some(LexItems::OParen) => {
                self.advance();
                let ab_body = self.parse();
                assert_eq!(self.peek(), Some(&LexItems::CParen));
                self.advance();
                ab_body
            }
            Some(LexItems::Var(s)) => {
                let t = AstNode::Var(s.clone());
                self.advance();
                t
            }
            Some(&LexItems::AbsStart) => self.parse_abstraction(),
            _ => panic!("Unexpected token in application.")
        };
        while matches!(self.peek(), Some(LexItems::Var(_)) | Some(LexItems::OParen) | Some(LexItems::AbsStart)) {
            let next = self.parse();
            term = AstNode::App(Rc::new(term), Rc::new(next));
        };
        term


    }
    
}

#[derive(Debug, Clone, PartialEq)]
pub enum BruijnAst {
    FreeVar(String),
    Var(usize),
    Abs(Box<BruijnAst>),
    App(Box<BruijnAst>, Box<BruijnAst>),
}
/*
pub fn nodes_to_bruijn(nodes: AstNode, ctx: &mut Vec<String>) -> BruijnAst {
    match nodes {
        AstNode::Abs(arg, body) => {
            ctx.push(arg.clone());
            let out = nodes_to_bruijn(*body, ctx);
            ctx.pop();
            BruijnAst::Abs(Box::new(out))
        }
        AstNode::Var(s) => {
            let idx = ctx.iter().position(|u| u == &s);
            if idx.is_some() {
                BruijnAst::Var(ctx.len() - idx.unwrap() - 1)
            } else {
                BruijnAst::FreeVar(s)
            }
        }
        AstNode::App(l, r) => {
            let left = nodes_to_bruijn(*l, ctx);
            let right = nodes_to_bruijn(*r, ctx);
            BruijnAst::App(Box::new(left), Box::new(right))
        }
    }
}
    */