type PeekableChars<'a> = std::iter::Peekable<std::str::Chars<'a>>;

#[derive(Debug, PartialEq)]
pub enum Kind {
    RESERVED, //記号，将来的には予約語？
    NUM,
    WHITESPACE,
    EOF,
}
impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Kind::RESERVED => write!(f, "RESERVED"),
            Kind::NUM => write!(f, "NUM"),
            Kind::WHITESPACE => write!(f, "WHITESPACE"),
            Kind::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: Kind,
    pub val: i32,
    pub s: PeekableChars<'a>,
}

pub fn tokenize<'a>(p: &'a mut PeekableChars<'a>) -> impl Iterator<Item = Token<'a>> {
    std::iter::from_fn(|| match p.peek() {
        None => Some(Token {
            kind: Kind::EOF,
            val: -1,
            s: p.clone(),
        }),
        Some(' ') => {
            let _p = p.clone();
            p.next();
            Some(Token {
                kind: Kind::WHITESPACE,
                val: -1,
                s: _p,
            })
        }
        Some(n) if ('0'..='9').contains(n) => {
            let _p = p.clone();
            Some(Token {
                kind: Kind::NUM,
                val: eat_number(p),
                s: p.clone(),
            })
        }
        Some(r) if ['+', '-'].contains(r) => {
            let _p = p.clone();
            p.next();
            Some(Token {
                kind: Kind::RESERVED,
                val: -1,
                s: _p,
            })
        }
        _ => panic!("unknown"),
    })
}

fn eat_number(p: &mut PeekableChars) -> i32 {
    // TODO: 新規allocじゃなくてvec参照を舐めたいが
    let mut s = String::new();
    while let Some(&c) = p.peek() {
        if !('0'..'9').contains(&c) {
            break;
        }
        s.push(p.next().unwrap());
    }
    s.parse::<i32>().unwrap()
}
