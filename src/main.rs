use ir::process_ast;
use parser::parse_ts_module;

fn main() {
    let code = r#"
        function add(a, b) {
        return a + b;
    }
    "#;

    let ast = parse_ts_module(code);
    process_ast(ast);
}
