pub mod lexer;

use lexer::{Lexer, Source};

const SOURCE: &'static str = r#"
let result = some_function(5);
"#;

fn main() {
    let source_str = SOURCE.to_string();
    let source = Source::new(&source_str);
    let lexer = Lexer::new(&source);

    for token in lexer {
        println!("{:?}", token);
    }
}
