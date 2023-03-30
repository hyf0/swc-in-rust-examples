use swc_core::{ecma::{parser, ast, codegen}, common};

fn main() {
    let a_js = std::fs::read_to_string("./fixtures/js/main.js").unwrap();
    let cm = common::sync::Lrc::new(common::SourceMap::default());
    let fm = cm.new_source_file(swc_core::common::FileName::Anon, a_js);
    let module = parser::parse_file_as_module(&fm, parser::Syntax::Es(parser::EsConfig {
        ..Default::default()
    }), ast::EsVersion::latest(), None, &mut vec![]).unwrap();

    let code = {
        let mut buf = vec![];
        {
            let mut emitter = codegen::Emitter {
                cfg: Default::default(),
                cm: cm.clone(),
                comments: None,
                wr: codegen::text_writer::JsWriter::new(cm, "\n", &mut buf, None),
            };

            emitter.emit_module(&module).unwrap();
        }
        String::from_utf8_lossy(&buf).to_string()
    };
    println!("------ code ------");
    println!("{}", code)
}