fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("invalid args len");
        std::process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    // return value should be stored in %RAX
    println!("  mov rax, {}", args[1]);
    println!("  ret");

    std::process::exit(0);
}
