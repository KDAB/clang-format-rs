<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/KDAB/clang-format-rs/compare/v0.1.3...HEAD)

### Added

- `clang_format_with_style` method where the style is given

### Changed

- `clang_format` now uses `ClangFormatStyle::Default` instead of reading from `CLANG_FORMAT_STYLE`

### Removed

- `CLANG_FORMAT_STYLE` OnceCell

## [0.1.3](https://github.com/KDAB/clang-format-rs/compare/v0.1.2...v0.1.3) - 2023-04-17

### Added

- Read `CLANG_FORMAT_BINARY` env var for custom `clang-format` locations
