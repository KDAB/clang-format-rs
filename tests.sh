#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

cargo test --all-targets --all-features --manifest-path Cargo.toml
cargo test --doc --manifest-path Cargo.toml
cargo clippy --all-targets --all-features --manifest-path Cargo.toml -- -D warnings
cargo fmt --manifest-path Cargo.toml -- --check
reuse lint
