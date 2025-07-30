use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_parser::{Parser, StringInput, Syntax, lexer::Lexer};
use swc_ecma_ast::Module;

fn main() {
    let code = r#"
    function add(a, b) {
        return a + b;
    }
    "#;

    let module = parse_ts_module(code);
    println!("{:#?}", module);
}

fn parse_ts_module(code: &str) -> Module {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        Lrc::new(FileName::Custom("input.ts".into())), 
        code.to_string()
    );

    let syntax = Syntax::Typescript(Default::default());
    let lexer = Lexer::new(
        syntax,
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    parser.parse_module().expect("Failed to parse TypeScript")
}