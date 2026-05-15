fn main() {
    let src = std::path::Path::new("src");
    println!("cargo:rerun-if-changed=src/parser.c");
    println!("cargo:rerun-if-changed=src/scanner.c");
    cc::Build::new()
        .include(&src)
        .std("c11")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs")
        .file(src.join("parser.c"))
        .file(src.join("scanner.c"))
        .compile("tree-sitter-pine");
}
