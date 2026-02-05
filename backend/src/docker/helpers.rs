use std::process::Output;

pub fn parse_docker_json<T>(output: Output) -> Result<T, anyhow::Error>
where
    T: serde::de::DeserializeOwned,
{
    let stdout_string = String::from_utf8(output.stdout)
        .map_err(|e| anyhow::anyhow!("Failed to convert output to string: {}", e))?;
    let stdout_string = json_object_lines_to_array(&stdout_string);

    let stdout_string = strip_double_array_wrapping(&stdout_string);

    let results = serde_json::from_str::<T>(&stdout_string)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

    Ok(results)
}

pub fn json_object_lines_to_array(input: &str) -> String {
    let mut lines = input.lines().map(str::trim).filter(|line| !line.is_empty());
    let first_line = lines.next().unwrap_or("");
    let mut result = String::from("[");
    result.push_str(first_line);

    for line in lines {
        result.push_str(",");
        result.push_str(line);
    }

    result.push(']');
    result
}

fn strip_double_array_wrapping(input: &str) -> String {
    let trimmed = input.trim();

    if trimmed.starts_with("[[") && trimmed.ends_with("]]") {
        let without_outer = &trimmed[1..trimmed.len() - 1];
        without_outer.trim().to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_object_lines_to_array_empty() {
        let input = "";
        let expected = "[]";
        let result = json_object_lines_to_array(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_json_object_lines_to_array_single_line() {
        let input = r#"{"Name":"stack1","Services":"2"}"#;
        let expected = r#"[{"Name":"stack1","Services":"2"}]"#;
        let result = json_object_lines_to_array(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_json_object_lines_to_array_multiple_lines() {
        let input = r#"{"Name":"stack1","Services":"2"}
{"Name":"stack2","Services":"3"}"#;
        let expected = r#"[{"Name":"stack1","Services":"2"},{"Name":"stack2","Services":"3"}]"#;
        let result = json_object_lines_to_array(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_json_object_lines_to_array_trailing_newline() {
        let input = r#"{"Name":"stack1","Services":"2"}
{"Name":"stack2","Services":"3"}
"#;
        let expected = r#"[{"Name":"stack1","Services":"2"},{"Name":"stack2","Services":"3"}]"#;
        let result = json_object_lines_to_array(input);
        assert_eq!(result, expected);
    }
}
