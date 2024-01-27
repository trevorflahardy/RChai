pub mod lexer;
pub mod parser;

use lexer::{Lexer, Source};

const SOURCE: &'static str = r#"
let x = 1;
"#;

fn main() {
    let source_str = SOURCE.to_string();
    let source = Source::new(&source_str);
    let lexer = Lexer::new(&source);

    for token in lexer {
        println!("{:?}", token);
    }
}
