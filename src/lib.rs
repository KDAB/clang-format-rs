// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(missing_docs)]

//! A basic clang-format Rust wrapper.
//!
//! This allows for formatting a given input using `clang-format` from the system.

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

/// Describes the style to pass to clang-format
///
/// This list is created from
/// <https://clang.llvm.org/docs/ClangFormatStyleOptions.html#basedonstyle>
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum ClangFormatStyle {
    /// A style complying with [Chromium’s style guide](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/styleguide/styleguide.md)
    Chromium,
    /// Use the default clang-format style
    Default,
    /// clang-format will try to find the .clang-format file located in the closest parent directory of the current directory.
    File,
    /// A style complying with the [GNU coding standards](https://www.gnu.org/prep/standards/standards.html)
    ///
    /// Since clang-format 11
    GNU,
    /// A style complying with [Google’s C++ style guide](https://google.github.io/styleguide/cppguide.html)
    Google,
    /// A style complying with the [LLVM coding standards](https://llvm.org/docs/CodingStandards.html)
    Llvm,
    /// A style complying with [Microsoft’s style guide](https://docs.microsoft.com/en-us/visualstudio/ide/editorconfig-code-style-settings-reference)
    ///
    /// Since clang-format 9
    Microsoft,
    /// A style complying with [Mozilla’s style guide](https://firefox-source-docs.mozilla.org/code-quality/coding-style/index.html)
    Mozilla,
    /// A style complying with [WebKit’s style guide](https://www.webkit.org/coding/coding-style.html)
    WebKit,
    /// Specify a custom input to the `--style` argument of clang-format
    ///
    /// # Example
    ///
    /// ```
    /// # use clang_format::{clang_format_with_style, ClangFormatStyle};
    /// # fn main() {
    /// # let input = r#"
    /// #     struct Test {
    /// #         bool field;
    /// #     };
    /// # "#;
    /// let style = ClangFormatStyle::Custom("{ BasedOnStyle: Mozilla, IndentWidth: 8 }".to_string());
    /// # let output = clang_format_with_style(input, &style);
    /// # assert!(output.is_ok());
    /// # assert_eq!(output.unwrap(), "\nstruct Test\n{\n        bool field;\n};\n");
    /// # }
    /// ```
    Custom(String),
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
            Self::GNU => "GNU",
            Self::Google => "Google",
            Self::Llvm => "LLVM",
            Self::Microsoft => "Microsoft",
            Self::Mozilla => "Mozilla",
            Self::WebKit => "WebKit",
            // Custom style arguments
            Self::Custom(custom) => custom.as_str(),
        }
    }
}

/// Describes which error spawning clang-format failed with
#[derive(Debug)]
pub enum ClangFormatError {
    /// Failed to spawn the clang-format process
    SpawnFailure,
    /// Failed to retrieve the stdin handle
    StdInFailure,
    /// Failed to write the input to the stdin handle
    StdInWriteFailure,
    /// Failed to convert the clang-format stdout to UTF-8
    Utf8FormatError,
    /// Failed to wait for the process to end with output
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

    #[test]
    fn format_custom() {
        let input = r#"
            struct Test {
                bool field;
            };
        "#;

        // Test multiple lines and single quotes
        {
            let output = clang_format_with_style(
                input,
                &ClangFormatStyle::Custom(
                    "{BasedOnStyle: 'Mozilla',
                    IndentWidth: 8}"
                        .to_string(),
                ),
            );
            assert!(output.is_ok());
            assert_eq!(
                output.unwrap(),
                "\nstruct Test\n{\n        bool field;\n};\n"
            );
        }

        // Test single line and double quotes
        {
            let output = clang_format_with_style(
                input,
                &ClangFormatStyle::Custom(
                    "{ BasedOnStyle: \"Mozilla\", IndentWidth: 4 }".to_string(),
                ),
            );
            assert!(output.is_ok());
            assert_eq!(output.unwrap(), "\nstruct Test\n{\n    bool field;\n};\n");
        }
    }
}
