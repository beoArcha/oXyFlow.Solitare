//! Rule `err_arms`: empty error arms.

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::{Finding, Severity, SourceFile};

pub fn check(file: &SourceFile) -> Vec<Finding> {
    let Ok(syntax) = syn::parse_file(&file.text) else {
        return Vec::new();
    };
    let mut visitor = ArmVisitor {
        file,
        findings: Vec::new(),
    };
    visitor.visit_file(&syntax);
    visitor.findings
}

struct ArmVisitor<'a> {
    file: &'a SourceFile,
    findings: Vec<Finding>,
}

impl<'ast> Visit<'ast> for ArmVisitor<'_> {
    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        for arm in &node.arms {
            if is_err_pat(&arm.pat) && is_empty_expr(&arm.body) {
                self.findings.push(Finding {
                    rule: "err-arms",
                    severity: Severity::Error,
                    file: self.file.path.clone(),
                    line: arm.pat.span().start().line,
                    message: "empty Err arm".to_string(),
                });
            }
        }
        syn::visit::visit_expr_match(self, node);
    }

    fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
        if is_let_err(&node.cond) && is_empty_block(&node.then_branch) {
            self.findings.push(Finding {
                rule: "err-arms",
                severity: Severity::Error,
                file: self.file.path.clone(),
                line: node.if_token.span.start().line,
                message: "empty Err arm".to_string(),
            });
        }
        syn::visit::visit_expr_if(self, node);
    }
}

fn is_err_pat(pat: &syn::Pat) -> bool {
    match pat {
        syn::Pat::TupleStruct(pts) => pts
            .path
            .segments
            .last()
            .is_some_and(|seg| seg.ident == "Err"),
        syn::Pat::Struct(ps) => ps
            .path
            .segments
            .last()
            .is_some_and(|seg| seg.ident == "Err"),
        syn::Pat::Paren(paren) => is_err_pat(&paren.pat),
        syn::Pat::Reference(reference) => is_err_pat(&reference.pat),
        syn::Pat::Or(or_pat) => or_pat.cases.iter().any(is_err_pat),
        _ => false,
    }
}

fn is_let_err(cond: &syn::Expr) -> bool {
    match cond {
        syn::Expr::Let(expr_let) => is_err_pat(&expr_let.pat),
        syn::Expr::Paren(paren) => is_let_err(&paren.expr),
        syn::Expr::Binary(binary) if matches!(binary.op, syn::BinOp::And(_)) => {
            is_let_err(&binary.left) || is_let_err(&binary.right)
        }
        _ => false,
    }
}

fn is_empty_expr(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Tuple(tuple) => tuple.elems.is_empty(),
        syn::Expr::Block(block) => is_empty_block(&block.block),
        syn::Expr::Paren(paren) => is_empty_expr(&paren.expr),
        _ => false,
    }
}

fn is_empty_block(block: &syn::Block) -> bool {
    if block.stmts.is_empty() {
        return true;
    }
    if block.stmts.len() == 1 {
        match &block.stmts[0] {
            syn::Stmt::Expr(expr, _) => is_empty_expr(expr),
            _ => false,
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn source(text: &str) -> SourceFile {
        SourceFile {
            path: PathBuf::from("test.rs"),
            text: text.to_string(),
        }
    }

    #[test]
    fn match_err_empty_block_is_flagged() {
        let code = r#"
            fn f() {
                match res {
                    Err(..) => {}
                    Ok(_) => {}
                }
            }
        "#;
        let findings = check(&source(code));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, "err-arms");
        assert_eq!(findings[0].severity, Severity::Error);
        assert_eq!(findings[0].line, 4);
    }

    #[test]
    fn match_err_empty_tuple_is_flagged() {
        let code = r#"
            fn f() {
                match res {
                    Err(_) => (),
                    Ok(_) => {}
                }
            }
        "#;
        let findings = check(&source(code));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, "err-arms");
        assert_eq!(findings[0].severity, Severity::Error);
        assert_eq!(findings[0].line, 4);
    }

    #[test]
    fn if_let_err_empty_block_is_flagged() {
        let code = r#"
            fn f() {
                if let Err(..) = res {}
            }
        "#;
        let findings = check(&source(code));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, "err-arms");
        assert_eq!(findings[0].severity, Severity::Error);
        assert_eq!(findings[0].line, 3);
    }

    #[test]
    fn if_let_err_with_binding_empty_block_is_flagged() {
        let code = r#"
            fn f() {
                if let Err(e) = res {}
            }
        "#;
        let findings = check(&source(code));
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, "err-arms");
        assert_eq!(findings[0].severity, Severity::Error);
    }

    #[test]
    fn non_empty_err_arms_yield_no_findings() {
        let code = r#"
            fn f() {
                match res {
                    Err(e) => return Err(e.into()),
                }
                match res {
                    Err(e) => log(e),
                }
                if let Err(e) = res {
                    log(e);
                }
            }
        "#;
        let findings = check(&source(code));
        assert!(findings.is_empty());
    }

    #[test]
    fn ok_arms_yield_no_findings() {
        let code = r#"
            fn f() {
                match res {
                    Ok(_) => {}
                }
                if let Ok(_) = res {}
            }
        "#;
        let findings = check(&source(code));
        assert!(findings.is_empty());
    }

    #[test]
    fn unparseable_file_yields_no_findings() {
        let code = "fn invalid_rust { syntax error }";
        let findings = check(&source(code));
        assert!(findings.is_empty());
    }

    #[test]
    fn various_err_patterns_are_flagged() {
        let code = r#"
            fn f() {
                match res {
                    Err(CustomError { .. }) => {}
                    Result::Err(_) => ()
                }
            }
        "#;
        let findings = check(&source(code));
        assert_eq!(findings.len(), 2);
    }
}
