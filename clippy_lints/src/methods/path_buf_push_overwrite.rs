use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::res::MaybeDef as _;
use clippy_utils::sym;
use rustc_ast::ast::LitKind;
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::LateContext;

use super::PATH_BUF_PUSH_OVERWRITE;

pub(super) fn check<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>, arg: &'tcx Expr<'_>) {
    if let Some(method_id) = cx.typeck_results().type_dependent_def_id(expr.hir_id)
        && let Some(impl_id) = cx.tcx.impl_of_assoc(method_id)
        && cx
            .tcx
            .type_of(impl_id)
            .instantiate_identity()
            .skip_norm_wip()
            .is_diag_item(cx, sym::PathBuf)
        && let ExprKind::Lit(lit) = arg.kind
        && let LitKind::Str(path_lit, _) = lit.node
        && let path_str = path_lit.as_str()
        && path_str.starts_with(['/', '\\'])
    {
        span_lint_and_sugg(
            cx,
            PATH_BUF_PUSH_OVERWRITE,
            lit.span,
            "calling `push` with '/' or '\\' (file system root) will overwrite the previous path definition",
            "try",
            format!("\"{}\"", path_str.trim_start_matches(['/', '\\'])),
            Applicability::MaybeIncorrect,
        );
    }
}
