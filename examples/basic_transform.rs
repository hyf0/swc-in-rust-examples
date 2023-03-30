use swc_core::{ecma::{parser, ast, codegen, visit::{self, VisitWith, VisitMutWith}}, common};

fn main() {
    let a_js = std::fs::read_to_string("./fixtures/js/main.js").unwrap();
    let cm = common::sync::Lrc::new(common::SourceMap::default());
    let fm = cm.new_source_file(swc_core::common::FileName::Anon, a_js);
    let mut module = parser::parse_file_as_module(&fm, parser::Syntax::Es(parser::EsConfig {
        ..Default::default()
    }), ast::EsVersion::latest(), None, &mut vec![]).unwrap();


    module.visit_with(&mut HelloKittyTransformer {});

    // 变量 `module` 需要用 `mut` 修饰
    module.visit_mut_with(&mut HelloKittyTransformer {});

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

struct HelloKittyTransformer {}

impl visit::Visit for HelloKittyTransformer {
    fn visit_expr(&mut self, node: &ast::Expr) {
        println!("visit: {node:#?}");
        // error: cannot assign to `node.value`, which is behind a `&` reference 
        // `node` is a `&` reference, so the data it refers to cannot be written
        // node.value = "hello, kitty".into()
    }
}

impl visit::VisitMut for HelloKittyTransformer {
    fn visit_mut_expr(&mut self, node: &mut ast::Expr) {
        if let ast::Expr::Call(call_expr) = node {
            if let Some(callee) = call_expr.callee.as_mut_expr() {
                if let Some(callee_ident) = callee.as_mut_ident() {
                    callee_ident.sym = "helloKitty".into()
                }
            }
        }
    }
}

impl visit::Fold for HelloKittyTransformer {
    fn fold_expr(&mut self, mut node: ast::Expr) -> ast::Expr {
        if let ast::Expr::Call(call_expr) = &mut node {
            if let Some(callee) = call_expr.callee.as_mut_expr() {
                if let Some(callee_ident) = callee.as_mut_ident() {
                    callee_ident.sym = "helloKitty".into();
                }
            }
        }
        node
    }
}