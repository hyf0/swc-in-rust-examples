use swc_core::ecma::{visit, ast};

pub struct SyntaxContextVisualizer;

impl visit::VisitMut for SyntaxContextVisualizer {
    fn visit_mut_ident(&mut self, node: &mut ast::Ident) {
        if node.span.ctxt.as_u32() != 0 {
            node.sym = format!("{}#{}", node.sym, node.span.ctxt.as_u32()).into()
        }
    }
}
