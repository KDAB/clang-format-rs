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
use thiserror::Error;

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
#[derive(Error, Debug)]
enum ClangFormatError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    // TODO: use ExitStatusError once it is a stable feature
    // https://doc.rust-lang.org/stable/std/process/struct.ExitStatusError.html
    // https://github.com/rust-lang/rust/issues/84908
    #[error("Clang format process exited with a non-zero status")]
    NonZeroExitStatus,
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
) -> Result<String, impl std::error::Error> {
    // Create and try to spawn the command with the specified style
    let clang_binary = env::var("CLANG_FORMAT_BINARY").unwrap_or("clang-format".to_string());
    let mut child = Command::new(clang_binary.as_str())
        .arg(format!("--style={}", style.as_str()))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Write the input to stdin
    //
    // Note we place inside a scope to ensure that stdin is closed
    {
        let mut stdin = child.stdin.take().expect("no stdin handle");
        write!(stdin, "{}", input)?;
    }

    // Wait for the output and parse it
    let output = child.wait_with_output()?;
    // TODO: use exit_ok() once it is a stable feature
    // https://doc.rust-lang.org/stable/std/process/struct.ExitStatus.html#method.exit_ok
    // https://github.com/rust-lang/rust/issues/84908
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(ClangFormatError::NonZeroExitStatus)
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
pub fn clang_format(input: &str) -> Result<String, impl std::error::Error> {
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
