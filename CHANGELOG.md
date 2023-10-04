<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/KDAB/clang-format-rs/compare/v0.3.0...HEAD)

## [0.3.0](https://github.com/KDAB/clang-format-rs/compare/v0.2.0...v0.3.0) - 2023-10-06

### Added

- It is now possible to specify a custom clang-format style using `ClangFormatStyle::Custom(String)`
- Support for `GNU` as a clang-format style
- Support for `Microsoft` as a clang-format style

### Changed

- `ClangFormatStyle` enum is now marked as `non_exhaustive` to allow for more styles in the future

### Removed

- `ClangFormatError` is now private, `thiserror` is used internally, and a `impl Error` is returned

## [0.2.0](https://github.com/KDAB/clang-format-rs/compare/v0.1.3...v0.2.0) - 2023-08-02

### Added

- `clang_format_with_style` method where the style is given

### Changed

- `clang_format` now uses `ClangFormatStyle::Default` instead of reading from `CLANG_FORMAT_STYLE`

### Removed

- `CLANG_FORMAT_STYLE` OnceCell

## [0.1.3](https://github.com/KDAB/clang-format-rs/compare/v0.1.2...v0.1.3) - 2023-04-17

### Added

- Read `CLANG_FORMAT_BINARY` env var for custom `clang-format` locations
