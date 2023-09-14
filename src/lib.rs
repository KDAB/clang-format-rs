// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

/// Describes the style to pass to clang-format
#[derive(Debug, PartialEq)]
pub enum ClangFormatStyle {
    Chromium,
    Default,
    File,
    Google,
    Llvm,
    Mozilla,
    WebKit,
}

impl ClangFormatStyle {
    /// Converts the enum ClangFormatStyle to a string that clang-format expects
    fn as_str(&self) -> &str {
        match self {
            Self::Chromium => "Chromium",
            // Will use clang-format default options
            Self::Default => "{}",
            // Will look in parent directories for a .clang-format file
            Self::File => "file",
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
///
/// # Example
///
/// ```
/// # use clang_format::{clang_format_with_style, ClangFormatStyle};
/// # fn main() {
/// let input = r#"
///     struct Test {
///
///     };
/// "#;
/// let output = clang_format_with_style(input, &ClangFormatStyle::Mozilla);
/// assert!(output.is_ok());
/// assert_eq!(output.unwrap(), "\nstruct Test\n{};\n");
/// # }
/// ```
pub fn clang_format_with_style(
    input: &str,
    style: &ClangFormatStyle,
) -> Result<String, ClangFormatError> {
    // Create and try to spawn the command with the specified style
    let clang_binary = env::var("CLANG_FORMAT_BINARY").unwrap_or("clang-format".to_string());
    if let Ok(mut child) = Command::new(clang_binary.as_str())
        .arg(format!("--style={}", style.as_str()))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
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
///
/// Note that this uses `ClangFormatStyle::Default` as the style.
///
/// # Example
///
/// ```
/// # use clang_format::clang_format;
/// # fn main() {
/// let input = r#"
///     struct Test {
///
///     };
/// "#;
/// let output = clang_format(input);
/// assert!(output.is_ok());
/// assert_eq!(output.unwrap(), "\nstruct Test {};\n");
/// # }
/// ```
pub fn clang_format(input: &str) -> Result<String, ClangFormatError> {
    clang_format_with_style(input, &ClangFormatStyle::Default)
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
        assert!(output.is_ok());
        assert_eq!(output.unwrap(), "\nstruct Test {};\n");
    }

    #[test]
    fn format_mozilla() {
        let input = r#"
            struct Test {

            };
        "#;
        let output = clang_format_with_style(input, &ClangFormatStyle::Mozilla);
        assert!(output.is_ok());
        assert_eq!(output.unwrap(), "\nstruct Test\n{};\n");
    }
}
