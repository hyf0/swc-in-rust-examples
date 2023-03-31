use swc_core::{
    common::{self, comments::SingleThreadedComments, Globals, Mark, GLOBALS},
    ecma::{
        ast::{self, Module},
        codegen, parser,
        transforms::{self, react, typescript},
        visit::{self, FoldWith, VisitMutWith, VisitWith},
    },
};
use swc_in_rust::SyntaxContextVisualizer;

fn main() {
    let book_tsx = std::fs::read_to_string("./fixtures/Book.tsx").unwrap();
    // let book_tsx = std::fs::read_to_string("./fixtures/Book-conflicted.tsx").unwrap();
    let cm = common::sync::Lrc::new(common::SourceMap::default());
    let print_code = |ast: &Module| {
        let code = {
            let mut buf = vec![];
            {
                let mut emitter = codegen::Emitter {
                    cfg: Default::default(),
                    cm: cm.clone(),
                    comments: None,
                    wr: codegen::text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None),
                };

                emitter.emit_module(ast).unwrap();
            }
            String::from_utf8_lossy(&buf).to_string()
        };
        println!("------ code ------");
        println!("{}", code)
    };

    let fm = cm.new_source_file(swc_core::common::FileName::Anon, book_tsx);
    let module = parser::parse_file_as_module(
        &fm,
        parser::Syntax::Typescript(parser::TsConfig {
            tsx: true,
            ..Default::default()
        }),
        ast::EsVersion::latest(),
        None,
        &mut vec![],
    )
    .unwrap();

    GLOBALS.set(&Globals::new(), || {
        let top_level_mark = Mark::new();
        let unresolved_mark = Mark::new();
        let module = module.fold_with(&mut transforms::base::resolver(
            unresolved_mark,
            top_level_mark,
            true,
        ));
        {
            let mut module = module.clone();
            module.visit_mut_with(&mut SyntaxContextVisualizer);
            print_code(&module);
        }
        let mut global_ident_finder = GlobalIdentFinder {
            unresolved_mark,
            global_ident_names: Default::default(),
        };
        module.visit_with(&mut global_ident_finder);
        println!(
            "global_ident_names: {:?}",
            global_ident_finder.global_ident_names
        );
        let module = module.fold_with(&mut typescript::strip(top_level_mark));
        print_code(&module);
        let mut module = module.fold_with(&mut react::jsx::<SingleThreadedComments>(
            cm.clone(),
            None,
            react::Options {
                runtime: Some(react::Runtime::Automatic),
                ..Default::default()
            },
            top_level_mark,
            unresolved_mark,
        ));
        {
            let mut module = module.clone();
            module.visit_mut_with(&mut SyntaxContextVisualizer);
            print_code(&module);
        }

        let module = module.fold_with(&mut transforms::base::hygiene::hygiene());

        {
            let mut module = module.clone();
            module.visit_mut_with(&mut SyntaxContextVisualizer);
            print_code(&module);
        }
    });
}

struct GlobalIdentFinder {
    unresolved_mark: Mark,
    global_ident_names: Vec<String>,
}

impl visit::Visit for GlobalIdentFinder {
    fn visit_ident(&mut self, node: &ast::Ident) {
        if node.span.ctxt.has_mark(self.unresolved_mark) {
            self.global_ident_names.push(node.sym.to_string())
        }
    }
}
