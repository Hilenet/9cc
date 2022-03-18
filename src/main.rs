mod error;

fn eat_number(
    p: &mut std::iter::Peekable<std::str::Chars>,
) -> Result<i32, <i32 as std::str::FromStr>::Err> {
    // iter.cloneは参照のコピーっぽい
    // https://github.com/rust-lang/rust/blob/691d1c1e12602c57237e9ccddac406ebd0c54082/library/core/src/str/iter.rs#L361-L373

    // TODO: 新規allocじゃなくてvec参照を舐めたいが
    let mut s = String::new();
    while let Some(&c) = p.peek() {
        if !('0'..'9').contains(&c) {
            break;
        }
        s.push(p.next().unwrap());
    }
    s.parse::<i32>()
}

fn parse(s: &String) -> Result<String, error::Error> {
    let mut lines: Vec<String> = vec![];

    lines.push(String::from(".intel_syntax noprefix"));
    lines.push(String::from(".globl main"));
    lines.push(String::from("main:"));

    //rustc lexer: https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/cursor.rs
    let mut c_iter = s.chars().peekable();
    lines.push(format!("  mov rax, {}", eat_number(&mut c_iter).unwrap()));
    while let Some(c) = c_iter.next() {
        match c {
            '+' => lines.push(String::from(format!(
                "  add rax, {}",
                eat_number(&mut c_iter).unwrap(),
            ))),
            '-' => lines.push(String::from(format!(
                "  sub rax, {}",
                eat_number(&mut c_iter).unwrap(),
            ))),
            _ => {
                return Err(error::Error::UnknownParams(format!("{}", c)));
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

    let assemble = parse(&args[1]);
    println!("{}", assemble.unwrap());
}
