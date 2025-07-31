use swc_ecma_ast::{Module, ModuleItem};

pub fn process_ast(module: Module) {
    fn transform_node(item: &ModuleItem) {
        match item {
            ModuleItem::ModuleDecl(_) => transform_decl(item),
            ModuleItem::Stmt(_) => transform_stmt(item),
        }
    }

    fn transform_decl(item: &ModuleItem) {
        match item {
            _ => {}
        }
    }

    fn transform_stmt(item: &ModuleItem) {
        match item {
            _ => {}
        }
    }

    for item in &module.body {
        transform_node(item);
    }
}
