use std::{fs, path::Path};

use oxc::{
    allocator::Allocator,
    parser::{Parser, ParserReturn},
    semantic::{SemanticBuilder, SemanticBuilderReturn},
    span::SourceType,
};

mod rust;

fn main() {
    // In real code, this will likely come from a file read from disk.
    let source_path = Path::new("./misc/add.js");
    let source_text = fs::read_to_string(source_path).unwrap();

    // Memory arena where AST nodes are allocated.
    let allocator = Allocator::default();
    // Infer source type (TS/JS/ESM/JSX/etc) based on file extension
    let source_type = SourceType::from_path(source_path).unwrap();
    let mut errors = Vec::new();

    // Step 1: Parsing
    // Parse the TSX file into an AST. The root AST node is a `Program` struct.
    let ParserReturn {
        program,
        errors: parser_errors,
        panicked,
        irregular_whitespaces: _,
    } = Parser::new(&allocator, &source_text, source_type).parse();
    errors.extend(parser_errors);

    // Parsing failed completely. `program` is empty and `errors` isn't. If the
    // parser could recover from errors, `program` will be a valid AST and
    // `errors` will be populated. We can still perform semantic analysis in
    // such cases (if we want).
    if panicked {
        for error in &errors {
            eprintln!("{error:?}");
        }
        panic!("Parsing failed.");
    }

    // Step 2: Semantic analysis.
    // Some of the more expensive syntax checks are deferred to this stage, and are
    // enabled using `with_check_syntax_error`. You are not required to enable
    // these, and they are disabled by default.
    let SemanticBuilderReturn {
        semantic,
        errors: semantic_errors,
    } = SemanticBuilder::new()
        .with_check_syntax_error(true) // Enable extra syntax error checking
        .with_build_jsdoc(true) // Enable JSDoc parsing
        .with_cfg(true) // Build a Control Flow Graph
        .build(&program); // Produce the `Semantic`

    errors.extend(semantic_errors);
    if errors.is_empty() {
        println!("parsing and semantic analysis completed successfully.");
    } else {
        for error in errors {
            eprintln!("{error:?}");
        }
        panic!("Failed to build Semantic for Counter component.");
    }

    println!("{:#?}", semantic.nodes().root_node().unwrap());

    println!();
    println!();
    println!();
    println!();

    let root = semantic.nodes().root_node().unwrap();
    println!("{}", rust::node_to_rust_text(&root.kind()));
}
