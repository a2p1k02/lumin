use crate::compiler::Compiler;

pub mod lexer;
pub mod parser;
pub mod astnode;
pub mod context;
pub mod compiler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let compiler = Compiler::from(args[1].clone());
        compiler.run();
    } else {
        println!("Usage: lumin [script]");
    }
}
