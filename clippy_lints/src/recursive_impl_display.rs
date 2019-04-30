use crate::utils::paths::DISPLAY_TRAIT;
use crate::utils::{is_expn_of, method_chain_args, span_lint_and_then, walk_ptrs_ty};
use if_chain::if_chain;
use rustc::hir;
use rustc::lint::{LateContext, LateLintPass, LintArray, LintPass};
use rustc::ty::{self, Ty};
use rustc::{declare_lint_pass, declare_tool_lint};
use syntax_pos::Span;
use syntax_pos::symbol::Symbol;

declare_clippy_lint! {
    pub RECURSIVE_IMPL_DISPLAY,
    correctness,
    ""
}

declare_lint_pass!(RecursiveImplDisplay => [RECURSIVE_IMPL_DISPLAY]);

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for RecursiveImplDisplay {
    fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
        // check for `impl Display for ..`
        let impl_def_id = cx.tcx.hir().local_def_id_from_hir_id(item.hir_id);
        if_chain! {
            if let hir::ItemKind::Impl(.., ref impl_items) = item.node;
            if let Some(impl_trait_ref) =
                cx.tcx.impl_trait_ref(impl_def_id);
            if cx.match_def_path(impl_trait_ref.def_id, &DISPLAY_TRAIT);
            then {
                lint_impl_body(cx, item.span, impl_items);
            }
        }
    }
}

fn lint_impl_body<'a, 'tcx>(cx: &LateContext<'a, 'tcx>, impl_span: Span, impl_items: &hir::HirVec<hir::ImplItemRef>) {
    use rustc::hir::intravisit::{self, NestedVisitorMap, Visitor};
    use rustc::hir::*;
    // check for `fn fmt(&self, f: &mut Formatter) -> Result ..`
    for impl_item in impl_items {
        if_chain! {
            if impl_item.ident.name == "fmt";
            if let ImplItemKind::Method(method_sig, body_id) =
                &cx.tcx.hir().impl_item(impl_item.id).node;
            if method_sig.decl.inputs.len() == 2;
            if let TyKind::Rptr(_, first_input) =
                &method_sig.decl.inputs[0].node;
            if let TyKind::Path(QPath::Resolved(_, first_path)) =
                &first_input.ty.node;
            if first_path.segments.len() == 1;
            then {
                dbg!(&first_path.segments[0].ident.name);
            }
        }
    }
}