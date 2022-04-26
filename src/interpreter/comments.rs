/// Replaces every comment with whitespace
pub fn strip_comments(source: &str) -> String {
    let mut processed_string = String::with_capacity(source.len());
    let mut previous_char = ' ';

    let mut ignore_comments = false;
    let mut quotation_escaped = false;

    let mut skip = false;
    for ch in source.chars() {
        if ch == '/' && previous_char == '/' && !ignore_comments {
            processed_string.pop();
            skip = true;
        }

        if ch == '"' && !quotation_escaped {
            ignore_comments = !ignore_comments;
        } else if ch == '\\' {
            quotation_escaped = true;
        } else {
            quotation_escaped = false;
        }

        if !skip {
            processed_string.push(ch);
        } else if ch != '\n' {
            processed_string.push(' ');
        } else {
            processed_string.push(ch);
            skip = false;
        }

        previous_char = ch;
    }

    processed_string
}

/// Takes in a string slice where index 0 is either a ' or ".
/// It iterates over the string until it finds a matching ' or " and returns the index.
fn find_matching_quotation(index: usize, source: &str) -> usize {
    let mut escaped = false;
    let starting_char = source.as_bytes()[index] as char;
    for (i, ch) in source[index + 1..].chars().enumerate() {
        if ch == starting_char && !escaped {
            // Add one to 'i' because it starts iterating at index 1
            return i + index + 1;
        } else if ch == '\\' {
            escaped = true;
        } else {
            escaped = false;
        }
    }

    usize::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_quotation() {
        let test_string = "Hello \"World\"";
        let test_return = find_matching_quotation(6, test_string);
        assert_eq!(test_return, 12);

        let test_string = "Hello \"World";
        let test_return = find_matching_quotation(6, test_string);
        assert_eq!(test_return, usize::MAX);

        let test_string = "Hello \"\"World";
        let test_return = find_matching_quotation(6, test_string);
        assert_eq!(test_return, 7);

        let test_string = "Hello \"\"World\"";
        let test_return = find_matching_quotation(6, test_string);
        assert_eq!(test_return, 7);

        let test_string = "Hi \"Hello \\\"There\\\"\"";
        let test_return = find_matching_quotation(3, test_string);
        assert_eq!(test_return, 19);
    }
}
