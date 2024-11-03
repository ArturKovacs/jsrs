use oxc::{
    ast::{
        ast::{BinaryOperator, BindingPattern, Expression, Function, Statement},
        AstKind,
    },
    syntax::node,
};

static OUTPUT_PRELUDE: &str = include_str!("./output_prelude.rs");

trait JoinIterator {
    fn join(self, sep: &str) -> String;
}

impl<ItemType, IterType> JoinIterator for IterType
where
    std::vec::Vec<String>: FromIterator<ItemType>,
    IterType: Iterator<Item = ItemType>,
{
    #[inline]
    fn join(self, sep: &str) -> String {
        self.collect::<Vec<String>>().join(sep)
    }
}

pub fn node_to_rust_text(node_kind: &AstKind) -> String {
    match node_kind {
        AstKind::Program(program) => {
            let mut result = String::with_capacity(program.source_text.len() + OUTPUT_PRELUDE.len());

            result.push_str(OUTPUT_PRELUDE);

            for statement in program.body.iter() {
                result.push_str(&statement_to_rust_text(statement));
            }
            result
        }
        _ => unimplemented!(),
    }
}

fn statement_to_rust_text(statement: &Statement) -> String {
    let mut result = String::new();
    match statement {
        Statement::FunctionDeclaration(func) => {
            let name = func.name().unwrap();

            let params = func
                .params
                .items
                .iter()
                .map(|param| binding_patter_to_rust_text(&param.pattern))
                .join(", ");

            let body = func
                .body
                .as_ref()
                .map(|body| body.statements.iter().map(statement_to_rust_text).join(""))
                .unwrap_or_else(String::new);

            result =
                format!("fn {name}({params}) -> JsValue {{ {body} return JsValue::Undefined; }} ");
        }
        Statement::ReturnStatement(statement) => {
            let expression = statement
                .argument
                .as_ref()
                .map(expression_to_rust_text)
                .unwrap_or_else(String::new);
            result = format!("return {expression};");
        }
        _ => unimplemented!(),
    }
    return result;
}

fn binding_patter_to_rust_text(pattern: &BindingPattern) -> String {
    use oxc::ast::ast::BindingPatternKind::*;
    match &pattern.kind {
        BindingIdentifier(identifier) => identifier.name.to_string(),
        _ => unimplemented!(),
    }
}

fn expression_to_rust_text(expression: &Expression) -> String {
    match expression {
        Expression::BinaryExpression(exp) => {
            let left = expression_to_rust_text(&exp.left);
            let right = expression_to_rust_text(&exp.right);

            let op = binary_operator_to_rust_text(&exp.operator);

            format!("({left}).{op}({right})")
        }
        Expression::Identifier(ident) => ident.name.to_string(),
        _ => unimplemented!(),
    }
}

/// This always returns the name of the equivalent function in our custom Rust impl
fn binary_operator_to_rust_text(operator: &BinaryOperator) -> &'static str {
    match operator {
        BinaryOperator::Addition => "add",
        _ => unimplemented!(),
    }
}
