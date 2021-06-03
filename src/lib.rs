use once_cell::sync::OnceCell;
use std::io::Write;
use std::process::{Command, Stdio};

/// The style to use for clang-format, use set to choose your default format
///
/// # Example
///
/// ```
/// # use clang_format::{CLANG_FORMAT_STYLE, ClangFormatStyle};
/// # fn main() {
/// CLANG_FORMAT_STYLE.set(ClangFormatStyle::Mozilla);
///
/// assert_eq!(CLANG_FORMAT_STYLE.get().unwrap(), &ClangFormatStyle::Mozilla);
/// # }
/// ```
pub static CLANG_FORMAT_STYLE: OnceCell<ClangFormatStyle> = OnceCell::new();

/// Describes the style to pass to clang-format
#[derive(Debug, PartialEq)]
pub enum ClangFormatStyle {
    Chromium,
    Default,
    // TODO: add File ? but can you specify where the file comes from?
    Google,
    Llvm,
    Mozilla,
    WebKit,
}

impl ClangFormatStyle {
    /// Converts the enum ClangFormatStyle to a string that clang-format expects
    fn as_str(&self) -> &'static str {
        match self {
            Self::Chromium => "Chromium",
            Self::Default => "",
            Self::Google => "Google",
            Self::Llvm => "LLVM",
            Self::Mozilla => "Mozilla",
            Self::WebKit => "WebKit",
        }
    }
}

/// Describes which error spawning clang-format failed with
#[derive(Debug)]
pub enum ClangFormatError {
    SpawnFailure,
    StdInFailure,
    StdInWriteFailure,
    Utf8FormatError,
    WaitFailure,
}

/// Execute clang-format with the given input, using the given style, and collect the output
fn clang_format_with_style(
    input: &str,
    style: &ClangFormatStyle,
) -> Result<String, ClangFormatError> {
    // Create the clang-format command
    let mut command = &mut Command::new("clang-format");
    // Determine if there is a style specified
    if style != &ClangFormatStyle::Default {
        command = command.arg(format!("--style={}", style.as_str()));
    }

    // Try to spawn the command
    if let Ok(mut child) = command.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
        // Try to take the stdin pipe
        if let Some(mut stdin) = child.stdin.take() {
            // Write the input to the stdin
            if write!(stdin, "{}", input).is_err() {
                return Err(ClangFormatError::StdInWriteFailure);
            }
        } else {
            return Err(ClangFormatError::StdInFailure);
        }

        // Wait for the output
        //
        // Note this cannot be inside the stdin block, as stdin is only closed
        // when it goes out of scope
        if let Ok(output) = child.wait_with_output() {
            // Parse the output into a String
            //
            // TODO: do we need to check stderr or exitcode?
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                Ok(stdout)
            } else {
                Err(ClangFormatError::Utf8FormatError)
            }
        } else {
            Err(ClangFormatError::WaitFailure)
        }
    } else {
        Err(ClangFormatError::SpawnFailure)
    }
}

/// Execute clang-format with the given input and collect the output
pub fn clang_format(input: &str) -> Result<String, ClangFormatError> {
    // Retrieve the style to use
    let style = CLANG_FORMAT_STYLE.get_or_init(|| ClangFormatStyle::Default);

    clang_format_with_style(input, style)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_default() {
        let input = r#"
            struct Test {

            };
        "#;
        let output = clang_format_with_style(input, &ClangFormatStyle::Default);
        assert_eq!(output.is_ok(), true);
        assert_eq!(output.unwrap(), "\nstruct Test {};\n");
    }

    #[test]
    fn format_mozilla() {
        let input = r#"
            struct Test {

            };
        "#;
        let output = clang_format_with_style(input, &ClangFormatStyle::Mozilla);
        assert_eq!(output.is_ok(), true);
        assert_eq!(output.unwrap(), "\nstruct Test\n{};\n");
    }
}
