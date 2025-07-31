use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

pub fn parse_ts_module(code: &str) -> Module {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(Lrc::new(FileName::Custom("input.ts".into())), code.to_string());

    let lexer = Lexer::new(Syntax::Typescript(Default::default()), Default::default(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);

    parser.parse_module().expect("Failed to parse TypeScript")
}