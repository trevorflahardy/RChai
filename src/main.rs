pub mod lexer;
pub mod parser;

use lexer::{Lexer, Source};
use parser::Parser;

const SOURCE: &'static str = r#"
let x = z == y;
"#;

fn main() {
    let source_str = SOURCE.to_string();
    let source = Source::new(&source_str);
    let lexer = Lexer::new(&source);

    let tokens = lexer.into_iter().collect::<Vec<_>>();

    let parser = Parser::new(&tokens);
}
