# tree-sitter-pine

[Tree-sitter](https://tree-sitter.github.io/tree-sitter/) grammar for [OpenPine](https://openpine.pages.dev/).

## Features

- Full OpenPine syntax parsing (expressions, control flow, type declarations, imports, etc.)
- External scanner for indentation-based blocks (INDENT / DEDENT / NEWLINE)
- Syntax highlighting queries (`queries/highlights.scm`) covering:
  - Built-in namespaces (`ta`, `strategy`, `math`, `color`, `request`, …)
  - Built-in variables (`close`, `open`, `high`, `low`, `bar_index`, …)
  - Built-in functions (`plot`, `indicator`, `input`, `alert`, …)
  - Keywords, types, operators, strings, comments

## Usage in Rust

Add to `Cargo.toml`:

```toml
[dependencies]
tree-sitter-pine = { git = "https://github.com/ylinwind/tree-sitter-pine" }
```

Parse OpenPine code:

```rust
use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_pine::LANGUAGE.into())
        .expect("Error loading OpenPine grammar");

    let code = r#"
indicator("My Script")
length = input(14, "Length")
plot(ta.sma(close, length))
"#;

    let tree = parser.parse(code, None).unwrap();
    println!("{}", tree.root_node().to_sexp());
}
```

## Exports

| Symbol | Type | Description |
|--------|------|-------------|
| `LANGUAGE` | `LanguageFn` | The tree-sitter language, pass to `parser.set_language()` |
| `HIGHLIGHTS_QUERY` | `&str` | Tree-sitter highlight query for syntax highlighting |
| `NODE_TYPES` | `&str` | JSON description of all node types in the grammar |

## Regenerating the Parser

Requires [tree-sitter CLI](https://tree-sitter.github.io/tree-sitter/creating-parsers#installation):

```bash
cargo install tree-sitter-cli
tree-sitter generate
```

## License

MIT
