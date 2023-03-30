use swc_core::{ecma::{parser, ast}, common};

fn main() {
    let a_js = std::fs::read_to_string("./fixtures/js/a.js").unwrap();
    let cm = common::SourceMap::default();
    let fm = cm.new_source_file(swc_core::common::FileName::Anon, a_js);
    let module = parser::parse_file_as_module(&fm, parser::Syntax::Es(parser::EsConfig {
        ..Default::default()
    }), ast::EsVersion::latest(), None, &mut vec![]).unwrap();

    println!("module: {module:#?}");
}