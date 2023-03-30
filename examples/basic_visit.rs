use swc_core::{ecma::{parser, ast, visit::{self, VisitWith}}, common};

fn main() {
    let a_js = std::fs::read_to_string("./fixtures/js/decl.js").unwrap();
    let cm = common::sync::Lrc::new(common::SourceMap::default());
    let fm = cm.new_source_file(swc_core::common::FileName::Anon, a_js);
    let module = parser::parse_file_as_module(&fm, parser::Syntax::Es(parser::EsConfig {
        ..Default::default()
    }), ast::EsVersion::latest(), None, &mut vec![]).unwrap();


    module.visit_with(&mut HelloKittyTransformer {});

}

struct HelloKittyTransformer {}

impl visit::Visit for HelloKittyTransformer {
    fn visit_fn_decl(&mut self, node: &ast::FnDecl) {
        println!("visit: {node:?}");
        node.visit_children_with(self);
    }

    fn visit_str(&mut self, node: &ast::Str) {
        println!("visit: {node:?}");
    }
}
