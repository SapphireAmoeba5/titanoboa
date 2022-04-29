/// Replaces all comments with spaces to preserve column count.
/// Comments start with the '#' character
pub fn strip_comments(source: &str) -> String {
    let mut formatted = String::with_capacity(source.len());

    let mut ignore_comment = false;
    let mut escaped = false;
    let mut quotation_char = ' ';

    for line in source.lines() {
        for (column, ch) in line.chars().enumerate() {
            if ch == '#' && !ignore_comment {
                let fill_count = line.len() - column;
                for _ in 0..fill_count {
                    formatted.push(' ');
                }
                // Break to skip parsing the current line and goto the next line
                break;
            } else if ch == '"' {
                if !ignore_comment {
                    quotation_char = '"';
                    ignore_comment = true;
                } else if quotation_char == '"' && !escaped {
                    ignore_comment = false;
                    quotation_char = ' ';
                }
            } else if ch == '\'' {
                if !ignore_comment {
                    quotation_char = '\'';
                    ignore_comment = true;
                } else if quotation_char == '\'' && !escaped {
                    ignore_comment = false;
                    quotation_char = ' ';
                }
            }

            if ch == '\\' {
                escaped = true;
            } else {
                escaped = false;
            }
            formatted.push(ch);
        }
        formatted.push('\n');
    }
    // Pop the final character which is a newline
    formatted.pop();
    formatted
}

#[test]
fn test_strip_comments() {
    let s = "Hello #World\nGoodbye World!";
    let s = strip_comments(s);
    assert_eq!(s, "Hello       \nGoodbye World!");

    let s = "Hello \"#World\"\nGoodbye World!";
    let s = strip_comments(s);
    assert_eq!(s, "Hello \"#World\"\nGoodbye World!");

    let s = "Hello \"#World\"\nGoodbye# World!";
    let s = strip_comments(s);
    assert_eq!(s, "Hello \"#World\"\nGoodbye        ");

    let s = "\"Hello \\\"World\\\"\"#Test";
    let s = strip_comments(s);
    assert_eq!(s, "\"Hello \\\"World\\\"\"     ");

    let s = "\"Hello \\\"World#Test\\\"\"#Test";
    let s = strip_comments(s);
    assert_eq!(s, "\"Hello \\\"World#Test\\\"\"     ");

    let s = "Hello 'a' '#' test";
    let s = strip_comments(s);
    assert_eq!(s, "Hello 'a' '#' test");

    let s = "Hello 'a' '\\''#t";
    let s = strip_comments(s);
    assert_eq!(s, "Hello 'a' '\\''  ");

    let s = "Hello \"'World'#Hello\"#Hello";
    let s = strip_comments(s);
    assert_eq!(s, "Hello \"'World'#Hello\"      ");
}
