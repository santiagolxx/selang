use std::collections::HashMap;

#[derive(Debug)]
pub enum PreprocessorError {
    InvalidDefine(String),
    CircularReference(String),
    IoError(String),
}

impl std::fmt::Display for PreprocessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreprocessorError::InvalidDefine(s) => write!(f, "Invalid #define: {}", s),
            PreprocessorError::CircularReference(s) => {
                write!(f, "Circular reference in macro: {}", s)
            }
            PreprocessorError::IoError(s) => write!(f, "IO error: {}", s),
        }
    }
}

impl std::error::Error for PreprocessorError {}

/// Preprocesa el código fuente, manejando #define y otros directives
pub fn preprocess_source(source: &str) -> Result<String, PreprocessorError> {
    let mut defines = HashMap::new();
    let mut result = String::new();

    for line in source.lines() {
        let line = line.trim();

        if line.starts_with("#define") {
            process_define(line, &mut defines)?;
        } else if line.starts_with("//") || line.is_empty() {
            // Ignorar comentarios y líneas vacías
            continue;
        } else {
            // Expandir macros en la línea
            let expanded = expand_macros(line, &defines)?;
            if !expanded.trim().is_empty() {
                result.push_str(&expanded);
                result.push('\n');
            }
        }
    }

    Ok(result)
}

fn process_define(
    line: &str,
    defines: &mut HashMap<String, String>,
) -> Result<(), PreprocessorError> {
    let parts: Vec<&str> = line.splitn(3, ' ').collect();

    if parts.len() < 3 {
        return Err(PreprocessorError::InvalidDefine(line.to_string()));
    }

    let macro_name = parts[1].to_string();
    let macro_value = parts[2].to_string();

    // Validar nombre de macro
    if !is_valid_identifier(&macro_name) {
        return Err(PreprocessorError::InvalidDefine(format!(
            "Invalid macro name: {}",
            macro_name
        )));
    }

    defines.insert(macro_name, macro_value);
    Ok(())
}

fn expand_macros(
    line: &str,
    defines: &HashMap<String, String>,
) -> Result<String, PreprocessorError> {
    let mut result = line.to_string();
    let mut expanded_macros = std::collections::HashSet::new();

    loop {
        let mut changed = false;

        for (macro_name, macro_value) in defines {
            if result.contains(macro_name) {
                // Detectar referencias circulares
                if expanded_macros.contains(macro_name) {
                    return Err(PreprocessorError::CircularReference(macro_name.clone()));
                }

                result = result.replace(macro_name, macro_value);
                expanded_macros.insert(macro_name.clone());
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    Ok(result)
}

fn is_valid_identifier(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
        && !name.chars().next().unwrap().is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_define() {
        let source = r#"
#define MAX_SIZE 100
PUSH MAX_SIZE
PRINT
"#;

        let result = preprocess_source(source).unwrap();
        assert!(result.contains("PUSH 100"));
    }

    #[test]
    fn test_multiple_defines() {
        let source = r#"
#define A 10
#define B 20
PUSH A
PUSH B
ADD
"#;

        let result = preprocess_source(source).unwrap();
        assert!(result.contains("PUSH 10"));
        assert!(result.contains("PUSH 20"));
    }

    #[test]
    fn test_macro_in_macro() {
        let source = r#"
#define INNER 42
#define OUTER INNER
PUSH OUTER
"#;

        let result = preprocess_source(source).unwrap();
        assert!(result.contains("PUSH 42"));
    }
}
