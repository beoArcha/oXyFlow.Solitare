//! Rule `loc`: file length limits.

use super::{Finding, Severity, SourceFile};

const WARNING_LIMIT: usize = 300;
const ERROR_LIMIT: usize = 500;

pub fn check(file: &SourceFile) -> Vec<Finding> {
    let count = count_lines(&file.text);
    let (severity, limit) = if count > ERROR_LIMIT {
        (Severity::Error, ERROR_LIMIT)
    } else if count > WARNING_LIMIT {
        (Severity::Warning, WARNING_LIMIT)
    } else {
        return Vec::new();
    };

    vec![Finding {
        rule: "loc",
        severity,
        file: file.path.clone(),
        line: 1,
        message: format!("file has {count} lines (limit {limit})"),
    }]
}

fn count_lines(text: &str) -> usize {
    let mut count = 0;
    let mut depth = 0;
    for line in text.lines() {
        if is_code_line(line, &mut depth) {
            count += 1;
        }
    }
    count
}

fn is_code_line(line: &str, depth: &mut usize) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }
    if *depth == 0 && trimmed.starts_with("//") {
        return false;
    }

    let chars: Vec<char> = line.chars().collect();
    let mut has_code = false;
    let mut index = 0;

    while index < chars.len() {
        let current = chars[index];
        let next = chars.get(index + 1).copied();

        if *depth == 0 {
            if current == '/' && next == Some('/') {
                break;
            }
            if current == '/' && next == Some('*') {
                *depth += 1;
                index += 2;
                continue;
            }
            if !current.is_whitespace() {
                has_code = true;
            }
            index += 1;
        } else {
            if current == '/' && next == Some('*') {
                *depth += 1;
                index += 2;
            } else if current == '*' && next == Some('/') {
                *depth -= 1;
                index += 2;
            } else {
                index += 1;
            }
        }
    }

    has_code
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn source_file(text: &str) -> SourceFile {
        SourceFile {
            path: PathBuf::from("test.rs"),
            text: text.to_owned(),
        }
    }

    fn make_lines(count: usize) -> String {
        let mut text = String::new();
        for i in 0..count {
            text.push_str(&format!("let x_{i} = {i};\n"));
        }
        text
    }

    #[test]
    fn test_300_lines_produces_no_findings() {
        let file = source_file(&make_lines(300));
        let findings = check(&file);
        assert!(findings.is_empty());
    }

    #[test]
    fn test_301_lines_produces_warning() {
        let file = source_file(&make_lines(301));
        let findings = check(&file);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, "loc");
        assert_eq!(findings[0].severity, Severity::Warning);
        assert_eq!(findings[0].line, 1);
        assert!(findings[0].message.contains("301"));
        assert!(findings[0].message.contains("300"));
    }

    #[test]
    fn test_500_lines_produces_warning() {
        let file = source_file(&make_lines(500));
        let findings = check(&file);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, "loc");
        assert_eq!(findings[0].severity, Severity::Warning);
        assert_eq!(findings[0].line, 1);
        assert!(findings[0].message.contains("500"));
        assert!(findings[0].message.contains("300"));
    }

    #[test]
    fn test_501_lines_produces_error() {
        let file = source_file(&make_lines(501));
        let findings = check(&file);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, "loc");
        assert_eq!(findings[0].severity, Severity::Error);
        assert_eq!(findings[0].line, 1);
        assert!(findings[0].message.contains("501"));
        assert!(findings[0].message.contains("500"));
    }

    #[test]
    fn test_blank_lines_not_counted() {
        let mut text = make_lines(300);
        text.push_str("\n\n   \n\t\n   \r\n");
        let file = source_file(&text);
        assert!(check(&file).is_empty());
    }

    #[test]
    fn test_line_comments_not_counted() {
        let mut text = make_lines(300);
        text.push_str("// regular comment\n");
        text.push_str("   // indented comment\n");
        text.push_str("/// doc comment\n");
        text.push_str("//! module doc comment\n");
        let file = source_file(&text);
        assert!(check(&file).is_empty());
    }

    #[test]
    fn test_block_comments_not_counted() {
        let mut text = make_lines(300);
        text.push_str("/* single line block comment */\n");
        text.push_str("   /* indented single line block comment */   \n");
        text.push_str("/*\n * multi-line\n * block\n * comment\n */\n");
        text.push_str("/* outer /* nested */ comment */\n");
        let file = source_file(&text);
        assert!(check(&file).is_empty());
    }

    #[test]
    fn test_code_with_comments_counted() {
        let text = "\
let a = 1; // line comment
let b = 2; /* block comment */
/* block comment */ let c = 3;
/* multi
line */ let d = 4;
let e = 5; /* multi
line */
";
        let file = source_file(text);
        assert_eq!(count_lines(&file.text), 5);
    }
}
