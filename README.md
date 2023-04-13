# clippy-c2rust-mapping

Builds on Immunant's C2Rust tool to retrieve C-to-Rust mappings on a line by line basis.

This helps identify code snippets for which the translated Rust output generates an error in the Rust compiler, or a lint error within Clippy.