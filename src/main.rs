use std::{collections::HashSet, fs, path::Path};

use oxc::{
    allocator::Allocator,
    ast::AstKind,
    parser::{Parser, ParserReturn},
    semantic::{AstNodes, SemanticBuilder, SemanticBuilderReturn},
    span::SourceType,
};

mod rust;

fn main() {
    // In real code, this will likely come from a file read from disk.
    let source_path = Path::new("./misc/n-body.js");
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

    // println!("{:#?}", semantic.nodes().root_node().unwrap());

    println!();
    println!();
    println!();
    println!();

    let root = semantic.nodes().root_node().unwrap();
    println!("{}", rust::node_to_rust_text(&root.kind()));
}

fn print_nodes(ast_nodes: &AstNodes) {
    let mut node_kinds = HashSet::new();

    for node in ast_nodes.iter() {
        match &node.kind() {
            AstKind::BooleanLiteral(_) => {
                node_kinds.insert("BooleanLiteral");
            }
            AstKind::NullLiteral(_) => {
                node_kinds.insert("NullLiteral");
            }
            AstKind::NumericLiteral(_) => {
                node_kinds.insert("NumericLiteral");
            }
            AstKind::BigIntLiteral(_) => {
                node_kinds.insert("BigIntLiteral");
            }
            AstKind::RegExpLiteral(_) => {
                node_kinds.insert("RegExpLiteral");
            }
            AstKind::StringLiteral(_) => {
                node_kinds.insert("StringLiteral");
            }
            AstKind::Program(_) => {
                node_kinds.insert("Program");
            }
            AstKind::IdentifierName(_) => {
                node_kinds.insert("IdentifierName");
            }
            AstKind::IdentifierReference(_) => {
                node_kinds.insert("IdentifierReference");
            }
            AstKind::BindingIdentifier(_) => {
                node_kinds.insert("BindingIdentifier");
            }
            AstKind::LabelIdentifier(_) => {
                node_kinds.insert("LabelIdentifier");
            }
            AstKind::ThisExpression(_) => {
                node_kinds.insert("ThisExpression");
            }
            AstKind::ArrayExpression(_) => {
                node_kinds.insert("ArrayExpression");
            }
            AstKind::ArrayExpressionElement(_) => {
                node_kinds.insert("ArrayExpressionElement");
            }
            AstKind::Elision(_) => {
                node_kinds.insert("Elision");
            }
            AstKind::ObjectExpression(_) => {
                node_kinds.insert("ObjectExpression");
            }
            AstKind::ObjectProperty(_) => {
                node_kinds.insert("ObjectProperty");
            }
            AstKind::PropertyKey(_) => {
                node_kinds.insert("PropertyKey");
            }
            AstKind::TemplateLiteral(_) => {
                node_kinds.insert("TemplateLiteral");
            }
            AstKind::TaggedTemplateExpression(_) => {
                node_kinds.insert("TaggedTemplateExpression");
            }
            AstKind::MemberExpression(_) => {
                node_kinds.insert("MemberExpression");
            }
            AstKind::CallExpression(_) => {
                node_kinds.insert("CallExpression");
            }
            AstKind::NewExpression(_) => {
                node_kinds.insert("NewExpression");
            }
            AstKind::MetaProperty(_) => {
                node_kinds.insert("MetaProperty");
            }
            AstKind::SpreadElement(_) => {
                node_kinds.insert("SpreadElement");
            }
            AstKind::Argument(_) => {
                node_kinds.insert("Argument");
            }
            AstKind::UpdateExpression(_) => {
                node_kinds.insert("UpdateExpression");
            }
            AstKind::UnaryExpression(_) => {
                node_kinds.insert("UnaryExpression");
            }
            AstKind::BinaryExpression(_) => {
                node_kinds.insert("BinaryExpression");
            }
            AstKind::PrivateInExpression(_) => {
                node_kinds.insert("PrivateInExpression");
            }
            AstKind::LogicalExpression(_) => {
                node_kinds.insert("LogicalExpression");
            }
            AstKind::ConditionalExpression(_) => {
                node_kinds.insert("ConditionalExpression");
            }
            AstKind::AssignmentExpression(_) => {
                node_kinds.insert("AssignmentExpression");
            }
            AstKind::AssignmentTarget(_) => {
                node_kinds.insert("AssignmentTarget");
            }
            AstKind::SimpleAssignmentTarget(_) => {
                node_kinds.insert("SimpleAssignmentTarget");
            }
            AstKind::AssignmentTargetPattern(_) => {
                node_kinds.insert("AssignmentTargetPattern");
            }
            AstKind::ArrayAssignmentTarget(_) => {
                node_kinds.insert("ArrayAssignmentTarget");
            }
            AstKind::ObjectAssignmentTarget(_) => {
                node_kinds.insert("ObjectAssignmentTarget");
            }
            AstKind::AssignmentTargetWithDefault(_) => {
                node_kinds.insert("AssignmentTargetWithDefault");
            }
            AstKind::SequenceExpression(_) => {
                node_kinds.insert("SequenceExpression");
            }
            AstKind::Super(_) => {
                node_kinds.insert("Super");
            }
            AstKind::AwaitExpression(_) => {
                node_kinds.insert("AwaitExpression");
            }
            AstKind::ChainExpression(_) => {
                node_kinds.insert("ChainExpression");
            }
            AstKind::ParenthesizedExpression(_) => {
                node_kinds.insert("ParenthesizedExpression");
            }
            AstKind::Directive(_) => {
                node_kinds.insert("Directive");
            }
            AstKind::Hashbang(_) => {
                node_kinds.insert("Hashbang");
            }
            AstKind::BlockStatement(_) => {
                node_kinds.insert("BlockStatement");
            }
            AstKind::VariableDeclaration(_) => {
                node_kinds.insert("VariableDeclaration");
            }
            AstKind::VariableDeclarator(_) => {
                node_kinds.insert("VariableDeclarator");
            }
            AstKind::EmptyStatement(_) => {
                node_kinds.insert("EmptyStatement");
            }
            AstKind::ExpressionStatement(_) => {
                node_kinds.insert("ExpressionStatement");
            }
            AstKind::IfStatement(_) => {
                node_kinds.insert("IfStatement");
            }
            AstKind::DoWhileStatement(_) => {
                node_kinds.insert("DoWhileStatement");
            }
            AstKind::WhileStatement(_) => {
                node_kinds.insert("WhileStatement");
            }
            AstKind::ForStatement(_) => {
                node_kinds.insert("ForStatement");
            }
            AstKind::ForStatementInit(_) => {
                node_kinds.insert("ForStatementInit");
            }
            AstKind::ForInStatement(_) => {
                node_kinds.insert("ForInStatement");
            }
            AstKind::ForOfStatement(_) => {
                node_kinds.insert("ForOfStatement");
            }
            AstKind::ContinueStatement(_) => {
                node_kinds.insert("ContinueStatement");
            }
            AstKind::BreakStatement(_) => {
                node_kinds.insert("BreakStatement");
            }
            AstKind::ReturnStatement(_) => {
                node_kinds.insert("ReturnStatement");
            }
            AstKind::WithStatement(_) => {
                node_kinds.insert("WithStatement");
            }
            AstKind::SwitchStatement(_) => {
                node_kinds.insert("SwitchStatement");
            }
            AstKind::SwitchCase(_) => {
                node_kinds.insert("SwitchCase");
            }
            AstKind::LabeledStatement(_) => {
                node_kinds.insert("LabeledStatement");
            }
            AstKind::ThrowStatement(_) => {
                node_kinds.insert("ThrowStatement");
            }
            AstKind::TryStatement(_) => {
                node_kinds.insert("TryStatement");
            }
            AstKind::CatchClause(_) => {
                node_kinds.insert("CatchClause");
            }
            AstKind::CatchParameter(_) => {
                node_kinds.insert("CatchParameter");
            }
            AstKind::DebuggerStatement(_) => {
                node_kinds.insert("DebuggerStatement");
            }
            AstKind::AssignmentPattern(_) => {
                node_kinds.insert("AssignmentPattern");
            }
            AstKind::ObjectPattern(_) => {
                node_kinds.insert("ObjectPattern");
            }
            AstKind::ArrayPattern(_) => {
                node_kinds.insert("ArrayPattern");
            }
            AstKind::BindingRestElement(_) => {
                node_kinds.insert("BindingRestElement");
            }
            AstKind::Function(_) => {
                node_kinds.insert("Function");
            }
            AstKind::FormalParameters(_) => {
                node_kinds.insert("FormalParameters");
            }
            AstKind::FormalParameter(_) => {
                node_kinds.insert("FormalParameter");
            }
            AstKind::FunctionBody(_) => {
                node_kinds.insert("FunctionBody");
            }
            AstKind::ArrowFunctionExpression(_) => {
                node_kinds.insert("ArrowFunctionExpression");
            }
            AstKind::YieldExpression(_) => {
                node_kinds.insert("YieldExpression");
            }
            AstKind::Class(_) => {
                node_kinds.insert("Class");
            }
            AstKind::ClassBody(_) => {
                node_kinds.insert("ClassBody");
            }
            AstKind::MethodDefinition(_) => {
                node_kinds.insert("MethodDefinition");
            }
            AstKind::PropertyDefinition(_) => {
                node_kinds.insert("PropertyDefinition");
            }
            AstKind::PrivateIdentifier(_) => {
                node_kinds.insert("PrivateIdentifier");
            }
            AstKind::StaticBlock(_) => {
                node_kinds.insert("StaticBlock");
            }
            AstKind::ModuleDeclaration(_) => {
                node_kinds.insert("ModuleDeclaration");
            }
            AstKind::ImportExpression(_) => {
                node_kinds.insert("ImportExpression");
            }
            AstKind::ImportDeclaration(_) => {
                node_kinds.insert("ImportDeclaration");
            }
            AstKind::ImportSpecifier(_) => {
                node_kinds.insert("ImportSpecifier");
            }
            AstKind::ImportDefaultSpecifier(_) => {
                node_kinds.insert("ImportDefaultSpecifier");
            }
            AstKind::ImportNamespaceSpecifier(_) => {
                node_kinds.insert("ImportNamespaceSpecifier");
            }
            AstKind::ExportNamedDeclaration(_) => {
                node_kinds.insert("ExportNamedDeclaration");
            }
            AstKind::ExportDefaultDeclaration(_) => {
                node_kinds.insert("ExportDefaultDeclaration");
            }
            AstKind::ExportAllDeclaration(_) => {
                node_kinds.insert("ExportAllDeclaration");
            }
            AstKind::ExportSpecifier(_) => {
                node_kinds.insert("ExportSpecifier");
            }
            AstKind::Decorator(_) => {
                node_kinds.insert("Decorator");
            }
            _ => unimplemented!(),
        }
    }

    println!("Node kinds: ");
    for kind in node_kinds {
        println!("{}", kind);
    }
}
