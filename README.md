<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# clang-format-rs

A basic clang-format Rust wrapper.

This allows for formatting a given input using `clang-format` from the system.
By default it uses `clang-format` binary but this can be changed by setting the
`CLANG_FORMAT_BINARY` environment variable, for example,
`CLANG_FORMAT_BINARY=clang-format-16`

```rust
use clang_format::{clang_format_with_style, ClangFormatStyle};

fn main() {
    let input = r#"
        struct Test {
        };
    "#;
    let output = clang_format_with_style(input, ClangFormatStyle::Mozilla);
    assert!(output.is_ok());
    assert_eq!(output.unwrap(), "\nstruct Test\n{};\n");
}
```

# Tests

The test suite can be executed using the `tests.sh` script.

```bash
./tests.sh
```

# Licensing

clang-format-rs is Copyright (C) 2021, Klarälvdalens Datakonsult AB, and is available under
the terms of the [MIT](https://github.com/KDAB/clang-format-rs/blob/main/LICENSES/MIT.txt)
or the [Apache-2.0](https://github.com/KDAB/clang-format-rs/blob/main/LICENSES/Apache-2.0.txt)
licenses.

Contact KDAB at <info@kdab.com> to inquire about additional features or
services related to this project.

# About KDAB

clang-format-rs is supported and maintained by Klarälvdalens Datakonsult AB (KDAB).

The KDAB Group is the global No.1 software consultancy for Qt, C++ and
OpenGL applications across desktop, embedded and mobile platforms.

The KDAB Group provides consulting and mentoring for developing Qt applications
from scratch and in porting from all popular and legacy frameworks to Qt.
We continue to help develop parts of Qt and are one of the major contributors
to the Qt Project. We can give advanced or standard trainings anywhere
around the globe on Qt as well as C++, OpenGL, 3D and more.

Please visit https://www.kdab.com to meet the people who write code like this.
