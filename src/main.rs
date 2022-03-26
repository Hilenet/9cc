mod error;
mod token;

//consume next token
fn consume<'a>(
    token_stream: &mut std::iter::Peekable<impl Iterator<Item = token::Token<'a>>>,
) -> Option<token::Token<'a>> {
    token_stream.next()
}

//check next token, consume if next token is number
fn expect_number<'a>(
    token_stream: &mut std::iter::Peekable<impl Iterator<Item = token::Token<'a>>>,
) -> Result<token::Token<'a>, error::Error> {
    let next = token_stream
        .peek()
        .ok_or(error::Error::TokenNonExists(String::from(
            "expect_number/ cannot consume token",
        )))?;
    (next.kind == token::Kind::NUM)
        .then(|| ())
        .ok_or(error::Error::UnexpectedToken(format!(
            "expect_number/ expect Kind:{}, but found Kind:{}",
            token::Kind::NUM,
            next.kind
        )))?;
    Ok(token_stream.next().unwrap())
}

//consume tokens, output assembly
fn assemble<'a>(
    token_stream: impl Iterator<Item = token::Token<'a>>,
) -> Result<String, error::Error> {
    let mut lines: Vec<String> = vec![];

    lines.push(String::from(".intel_syntax noprefix"));
    lines.push(String::from(".globl main"));
    lines.push(String::from("main:"));

    let mut t_iter = token_stream.peekable();
    lines.push(format!(
        "  mov rax, {}",
        consume(&mut t_iter)
            .ok_or(error::Error::TokenNonExists(String::from("first")))?
            .val
    ));

    // consume, output
    while let Some(mut t) = consume(&mut t_iter) {
        match t.kind {
            token::Kind::EOF => break,
            token::Kind::WHITESPACE => {
                consume(&mut t_iter);
            }
            token::Kind::RESERVED if t.s.peek().unwrap() == &'+' => lines.push(String::from(
                format!("  add rax, {}", expect_number(&mut t_iter)?.val),
            )),
            token::Kind::RESERVED if t.s.peek().unwrap() == &'-' => lines.push(String::from(
                format!("  sub rax, {}", expect_number(&mut t_iter)?.val),
            )),
            _ => {
                return Err(error::Error::UnexpectedToken(format!("{:#?}", t)));
            }
        }
    }
    lines.push(String::from("  ret"));

    Ok(lines.join("\n"))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("invalid args len");
        std::process::exit(1);
    }

    println!(
        "{}",
        assemble(token::tokenize(&mut args[1].chars().peekable())).unwrap()
    );
    //println!(
    //    "{:#?}",
    //    token::tokenize(&mut args[1].chars().peekable()).collect::<Vec<token::Token>>()
    //)
}
