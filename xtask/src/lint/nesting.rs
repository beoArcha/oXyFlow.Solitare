use syn::spanned::Spanned;
use syn::visit::Visit;

use super::{Finding, Severity, SourceFile};

pub fn check(file: &SourceFile) -> Vec<Finding> {
    let Ok(syntax_tree) = syn::parse_str::<syn::File>(&file.text) else {
        return Vec::new();
    };

    let mut collector = FunctionCollector::default();
    collector.visit_file(&syntax_tree);

    let mut findings = Vec::new();
    for function in collector.functions {
        let (max_depth, deepest_line) = measure_nesting(function.block);

        let severity = match max_depth {
            0..=4 => continue,
            5 | 6 => Severity::Warning,
            _ => Severity::Error,
        };

        findings.push(Finding {
            rule: "nesting",
            severity,
            file: file.path.clone(),
            line: deepest_line,
            message: format!(
                "function `{}` has nesting depth {}",
                function.name, max_depth
            ),
        });
    }

    findings
}

fn measure_nesting(block: &syn::Block) -> (usize, usize) {
    let mut visitor = NestingVisitor::new();
    visitor.visit_block(block);
    (visitor.max_depth, visitor.deepest_line)
}

struct FunctionToLint<'a> {
    name: String,
    block: &'a syn::Block,
}

#[derive(Default)]
struct FunctionCollector<'a> {
    functions: Vec<FunctionToLint<'a>>,
}

impl<'a> Visit<'a> for FunctionCollector<'a> {
    fn visit_item_fn(&mut self, node: &'a syn::ItemFn) {
        self.functions.push(FunctionToLint {
            name: node.sig.ident.to_string(),
            block: &node.block,
        });
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'a syn::ImplItemFn) {
        self.functions.push(FunctionToLint {
            name: node.sig.ident.to_string(),
            block: &node.block,
        });
        syn::visit::visit_impl_item_fn(self, node);
    }

    fn visit_trait_item_fn(&mut self, node: &'a syn::TraitItemFn) {
        if let Some(ref block) = node.default {
            self.functions.push(FunctionToLint {
                name: node.sig.ident.to_string(),
                block,
            });
        }
        syn::visit::visit_trait_item_fn(self, node);
    }
}

struct NestingVisitor {
    current_depth: usize,
    max_depth: usize,
    deepest_line: usize,
}

impl NestingVisitor {
    fn new() -> Self {
        Self {
            current_depth: 0,
            max_depth: 0,
            deepest_line: 0,
        }
    }

    fn record(&mut self, depth: usize, line: usize) {
        if depth > self.max_depth {
            self.max_depth = depth;
            self.deepest_line = line;
        }
    }

    fn with_depth<F: FnOnce(&mut Self)>(&mut self, depth: usize, f: F) {
        let prev = self.current_depth;
        self.current_depth = depth;
        f(self);
        self.current_depth = prev;
    }

    fn visit_if(&mut self, expr_if: &syn::ExprIf) {
        let if_depth = self.current_depth + 1;
        self.record(if_depth, expr_if.if_token.span().start().line);
        self.visit_expr(&expr_if.cond);
        self.with_depth(if_depth, |v| v.visit_block(&expr_if.then_branch));
        if let Some((_, else_expr)) = &expr_if.else_branch {
            self.visit_else_branch(else_expr, if_depth);
        }
    }

    fn visit_else_branch(&mut self, else_expr: &syn::Expr, branch_depth: usize) {
        match else_expr {
            syn::Expr::If(chained_if) => {
                self.visit_expr(&chained_if.cond);
                self.with_depth(branch_depth, |v| v.visit_block(&chained_if.then_branch));
                if let Some((_, next_else)) = &chained_if.else_branch {
                    self.visit_else_branch(next_else, branch_depth);
                }
            }
            syn::Expr::Block(else_block) => {
                self.with_depth(branch_depth, |v| v.visit_block(&else_block.block));
            }
            other => {
                self.with_depth(branch_depth, |v| v.visit_expr(other));
            }
        }
    }

    fn visit_match(&mut self, expr_match: &syn::ExprMatch) {
        let match_depth = self.current_depth + 1;
        self.record(match_depth, expr_match.match_token.span().start().line);
        self.visit_expr(&expr_match.expr);
        for arm in &expr_match.arms {
            self.visit_arm_contents(arm, match_depth);
        }
    }

    fn visit_arm_contents(&mut self, arm: &syn::Arm, arm_depth: usize) {
        self.with_depth(arm_depth, |v| v.visit_pat(&arm.pat));
        match &*arm.body {
            syn::Expr::Block(arm_block) => {
                self.with_depth(arm_depth, |v| v.visit_block(&arm_block.block));
            }
            body => {
                self.with_depth(arm_depth, |v| v.visit_expr(body));
            }
        }
    }

    fn visit_for(&mut self, expr_for: &syn::ExprForLoop) {
        let next_depth = self.current_depth + 1;
        self.record(next_depth, expr_for.for_token.span().start().line);
        self.visit_expr(&expr_for.expr);
        self.with_depth(next_depth, |v| v.visit_block(&expr_for.body));
    }

    fn visit_while(&mut self, expr_while: &syn::ExprWhile) {
        let next_depth = self.current_depth + 1;
        self.record(next_depth, expr_while.while_token.span().start().line);
        self.visit_expr(&expr_while.cond);
        self.with_depth(next_depth, |v| v.visit_block(&expr_while.body));
    }

    fn visit_loop(&mut self, expr_loop: &syn::ExprLoop) {
        let next_depth = self.current_depth + 1;
        self.record(next_depth, expr_loop.loop_token.span().start().line);
        self.with_depth(next_depth, |v| v.visit_block(&expr_loop.body));
    }

    fn visit_unsafe(&mut self, expr_unsafe: &syn::ExprUnsafe) {
        let next_depth = self.current_depth + 1;
        self.record(next_depth, expr_unsafe.unsafe_token.span().start().line);
        self.with_depth(next_depth, |v| v.visit_block(&expr_unsafe.block));
    }

    fn visit_async(&mut self, expr_async: &syn::ExprAsync) {
        let next_depth = self.current_depth + 1;
        self.record(next_depth, expr_async.async_token.span().start().line);
        self.with_depth(next_depth, |v| v.visit_block(&expr_async.block));
    }

    fn visit_closure(&mut self, expr_closure: &syn::ExprClosure) {
        if let syn::Expr::Block(block_expr) = &*expr_closure.body {
            let next_depth = self.current_depth + 1;
            self.record(next_depth, expr_closure.span().start().line);
            self.with_depth(next_depth, |v| v.visit_block(&block_expr.block));
        } else {
            self.visit_expr(&expr_closure.body);
        }
    }

    fn visit_plain_block(&mut self, expr_block: &syn::ExprBlock) {
        let next_depth = self.current_depth + 1;
        self.record(next_depth, expr_block.span().start().line);
        self.with_depth(next_depth, |v| v.visit_block(&expr_block.block));
    }
}

impl<'ast> Visit<'ast> for NestingVisitor {
    fn visit_stmt(&mut self, stmt: &'ast syn::Stmt) {
        if matches!(stmt, syn::Stmt::Item(_)) {
            return;
        }
        syn::visit::visit_stmt(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        match expr {
            syn::Expr::If(expr_if) => self.visit_if(expr_if),
            syn::Expr::Match(expr_match) => self.visit_match(expr_match),
            syn::Expr::ForLoop(expr_for) => self.visit_for(expr_for),
            syn::Expr::While(expr_while) => self.visit_while(expr_while),
            syn::Expr::Loop(expr_loop) => self.visit_loop(expr_loop),
            syn::Expr::Unsafe(expr_unsafe) => self.visit_unsafe(expr_unsafe),
            syn::Expr::Async(expr_async) => self.visit_async(expr_async),
            syn::Expr::Closure(expr_closure) => self.visit_closure(expr_closure),
            syn::Expr::Block(expr_block) => self.visit_plain_block(expr_block),
            _ => syn::visit::visit_expr(self, expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn first_function_depth(code: &str) -> usize {
        let file: syn::File = syn::parse_str(code).unwrap();
        let mut collector = FunctionCollector::default();
        collector.visit_file(&file);
        let first = collector.functions.first().unwrap();
        let (depth, _) = measure_nesting(first.block);
        depth
    }

    fn check_source(code: &str) -> Vec<Finding> {
        let file = SourceFile {
            path: PathBuf::from("test.rs"),
            text: code.to_owned(),
        };
        check(&file)
    }

    #[test]
    fn match_in_for_in_if_is_three() {
        let code = r#"
            fn sample() {
                if a {
                    for b in c {
                        match d {
                            _ => {}
                        }
                    }
                }
            }
        "#;
        assert_eq!(first_function_depth(code), 3);
    }

    #[test]
    fn else_if_chain_is_one() {
        let code = r#"
            fn sample() {
                if a {
                    1
                } else if b {
                    2
                } else if c {
                    3
                } else {
                    4
                }
            }
        "#;
        assert_eq!(first_function_depth(code), 1);
    }

    #[test]
    fn block_bodied_closure_adds_one() {
        let block_closure = r#"
            fn sample() {
                let f = || {
                    let x = 1;
                };
            }
        "#;
        assert_eq!(first_function_depth(block_closure), 1);

        let non_block_closure = r#"
            fn sample() {
                let f = |x| x + 1;
            }
        "#;
        assert_eq!(first_function_depth(non_block_closure), 0);
    }

    #[test]
    fn nested_function_does_not_add_to_parent() {
        let code = r#"
            fn parent() {
                if a {
                    fn child() {
                        if b {
                            if c {
                            }
                        }
                    }
                }
            }
        "#;
        let file: syn::File = syn::parse_str(code).unwrap();
        let mut collector = FunctionCollector::default();
        collector.visit_file(&file);
        assert_eq!(collector.functions.len(), 2);

        let parent = &collector.functions[0];
        assert_eq!(parent.name, "parent");
        assert_eq!(measure_nesting(parent.block).0, 1);

        let child = &collector.functions[1];
        assert_eq!(child.name, "child");
        assert_eq!(measure_nesting(child.block).0, 2);
    }

    #[test]
    fn depth_four_yields_no_findings() {
        let code = r#"
            fn depth_4() {
                if a {
                    if b {
                        if c {
                            if d {
                            }
                        }
                    }
                }
            }
        "#;
        assert_eq!(check_source(code), Vec::new());
    }

    #[test]
    fn depth_five_yields_warning() {
        let code = "fn depth_5() {\n    if a {\n        if b {\n            if c {\n                if d {\n                    if e {\n                    }\n                }\n            }\n        }\n    }\n}";
        let findings = check_source(code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, "nesting");
        assert_eq!(findings[0].severity, Severity::Warning);
        assert_eq!(findings[0].line, 6);
        assert!(findings[0].message.contains("depth_5"));
        assert!(findings[0].message.contains('5'));
    }

    #[test]
    fn depth_seven_yields_error() {
        let code = "fn depth_7() {\n    if a {\n        if b {\n            if c {\n                if d {\n                    if e {\n                        if f {\n                            if g {\n                            }\n                        }\n                    }\n                }\n            }\n        }\n    }\n}";
        let findings = check_source(code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, "nesting");
        assert_eq!(findings[0].severity, Severity::Error);
        assert_eq!(findings[0].line, 8);
        assert!(findings[0].message.contains("depth_7"));
        assert!(findings[0].message.contains('7'));
    }

    #[test]
    fn parse_failure_yields_no_findings() {
        let code = "fn broken( {";
        assert_eq!(check_source(code), Vec::new());
    }

    #[test]
    fn plain_unsafe_and_async_blocks_add_depth() {
        let code = r#"
            fn sample() {
                {
                    unsafe {
                        async {
                        }
                    }
                }
            }
        "#;
        assert_eq!(first_function_depth(code), 3);
    }
}
