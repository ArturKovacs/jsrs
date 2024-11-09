use std::collections::{HashMap, HashSet};

use oxc::{
    ast::{
        ast::{
            Argument, AssignmentExpression, AssignmentOperator, AssignmentTarget, BinaryOperator, BindingPattern, ComputedMemberExpression, Expression, ForStatementInit, ObjectPropertyKind, Program, PropertyKey, SimpleAssignmentTarget, Statement, StaticMemberExpression, UnaryOperator, UpdateExpression, VariableDeclaration, VariableDeclarationKind
        },
        AstKind,
    }, semantic::{AstNode, AstNodes}, syntax::node
};

mod output_prelude;

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

pub fn count_variable_modificiations(nodes: &AstNodes) -> HashMap<String, usize> {
    let variables = nodes.iter().filter_map(|node| {
        match node.kind() {
            AstKind::VariableDeclarator(decl) => {
                Some(decl.id.get_identifier().unwrap().to_string())
            }
            _ => None
        }
    }).collect::<HashSet<String>>();
    let result = HashMap::new();
    // for varialbe in variables {
    //     let modifications = nodes.iter().filter(|node| {
    //         match node.kind() {
    //             AstKind::AssignmentTarget(target) => {
    //                 match target {

    //                 }
    //             }
    //             _ => false
    //         }
    //     }).count();

    // }
    result
}

pub fn node_to_rust_text(node_kind: &AstKind) -> String {
    match node_kind {
        AstKind::Program(program) => {
            let mut result =
                String::with_capacity(program.source_text.len() + OUTPUT_PRELUDE.len());

            result.push_str(OUTPUT_PRELUDE);

            result.push_str("fn main() {\n");
            for statement in program.body.iter() {
                result.push_str(&statement_to_rust_text(statement));
                result.push_str("\n");
            }
            result.push_str("}");
            result
        }
        _ => unimplemented!(),
    }
}

fn statement_to_rust_text(statement: &Statement) -> String {
    match statement {
        Statement::FunctionDeclaration(func) => {
            let name = func.name().unwrap();

            let params = func
                .params
                .items
                .iter()
                .map(|param| format!("{}: JsValue", binding_pattern_to_rust_text(&param.pattern)))
                .join(", ");

            let body = func
                .body
                .as_ref()
                .map(|body| body.statements.iter().map(statement_to_rust_text).join("\n"))
                .unwrap_or_else(String::new);

            format!("let {name} = |{params}| -> JsValue {{ {body} return JsValue::Undefined; }}; ")
        }
        Statement::ReturnStatement(statement) => {
            let expression = statement
                .argument
                .as_ref()
                .map(expression_to_rust_text)
                .unwrap_or_else(String::new);
            format!("return {expression};")
        }
        Statement::VariableDeclaration(statement) => {
            variable_declaration_to_rust_text(&statement)
        }
        Statement::ForStatement(statement) => {
            let init = statement.init.as_ref().map(|statement| {
                if let ForStatementInit::VariableDeclaration(var_decl) = &statement {
                    variable_declaration_to_rust_text(&var_decl)
                } else {
                    let exp = statement.as_expression().unwrap();
                    let mut exp = expression_to_rust_text(exp);
                    exp.push_str(";");
                    exp
                }
            }).unwrap_or("".into());

            let test = statement.test.as_ref().map(|test| {
                let text = expression_to_rust_text(test);
                format!("if ({text}).falsy() {{ break; }}")
            }).unwrap_or("".into());

            let update = statement.update.as_ref().map(|exp| {
                let mut body = expression_to_rust_text(exp);
                body.push_str(";");
                body
            }).unwrap_or("".into());

            let body = statement_to_rust_text(&statement.body);

            format!("{init}\nloop {{\n{test}\n{body}\n{update}}}")
        }
        Statement::BlockStatement(statement) => {
            let body = statement.body.iter().map(statement_to_rust_text).collect::<Vec<String>>().join("\n");
            format!("{{{body}}}")
        }
        Statement::ExpressionStatement(statement) => {
            let expression_text = expression_to_rust_text(&statement.expression);
            format!("{expression_text};")
        }
        _ => unimplemented!("{:#?}", statement),
    }
}

fn update_expression_to_rust_text(expression: &UpdateExpression) -> String {
    use oxc::ast::ast::UpdateOperator::*;
    let name = match &expression.argument {
        SimpleAssignmentTarget::AssignmentTargetIdentifier(identifier) => {
            identifier.name.as_ref()
        }
        _ => unimplemented!()
    };
    
    if expression.prefix {
        match expression.operator {
            Decrement => format!("{{ {name} = {name}.sub(JsValue::Number(1.0)); {name} }}"),
            Increment => format!("{{ {name} = {name}.add(JsValue::Number(1.0)); {name} }}"),
        }
    } else { // postfix
        match expression.operator {
            Decrement => format!("{{ let tmp = ({name}).clone(); {name} = {name}.sub(JsValue::Number(1.0)); tmp }}"),
            Increment => format!("{{ let tmp = ({name}).clone(); {name} = {name}.add(JsValue::Number(1.0)); tmp }}"),
        }
    }
}

fn variable_declaration_to_rust_text(declaration: &VariableDeclaration) -> String {
    let mut declaration_texts = String::new();
    for declaration in declaration.declarations.iter() {
        let kind = match declaration.kind {
            VariableDeclarationKind::Const => "let",
            VariableDeclarationKind::Let => "let mut",
            _ => unimplemented!(),
        };
        let var_name = declaration.id.get_identifier().unwrap();

        let init = match &declaration.init {
            Some(init) => format!("= {}", expression_to_rust_text(init)),
            None => String::new(),
        };
        declaration_texts.push_str(&format!("{kind} {var_name} {init};\n"));
    }
    declaration_texts
}

fn binding_pattern_to_rust_text(pattern: &BindingPattern) -> String {
    use oxc::ast::ast::BindingPatternKind::*;
    match &pattern.kind {
        BindingIdentifier(identifier) => identifier.name.to_string(),
        _ => unimplemented!(),
    }
}

fn expression_to_rust_text(expression: &Expression) -> String {
    match expression {
        Expression::AssignmentExpression(exp) => assignment_expression_to_rust_text(exp),
        Expression::BinaryExpression(exp) => {
            let left = expression_to_rust_text(&exp.left);
            let right = expression_to_rust_text(&exp.right);

            let op = binary_operator_to_rust_text(exp.operator);

            format!("({left}).{op}(({right}).clone())")
        }
        Expression::UnaryExpression(exp) => {
            let op = unary_operator_to_rust_text(exp.operator);
            let argument = expression_to_rust_text(&exp.argument);
            format!("{op}({argument})")
        }
        Expression::StaticMemberExpression(exp) => {
            // NOTE:
            // The code should only enter this branch if we are _READING_ this member.
            // This is because StaticMemberExpression is handled as a special case in assignment expressions.

            static_member_read_to_rust_text(exp)
        }
        Expression::ComputedMemberExpression(exp) => {
            // NOTE:
            // The code should only enter this branch if we are _READING_ this member.
            // This is because ComputedMemberExpression is handled as a special case in assignment expressions.

            computed_member_read_to_rust_text(exp)
        }
        Expression::NumericLiteral(literal) => {
            let value = literal.value;
            format!("JsValue::Number({value} as f64)")
        }
        Expression::ObjectExpression(exp) => {
            let mut object_text = String::from("JsValue::from_entries([");
            for entry in exp.properties.iter() {
                if let ObjectPropertyKind::ObjectProperty(property) = entry {
                    if let PropertyKey::StaticIdentifier(identifier) = &property.key {
                        let key = identifier.name.as_str();
                        let value = expression_to_rust_text(&property.value);
                        let entry_text = format!("(\"{key}\".into(), {value}),");
                        object_text.push_str(&entry_text);
                    } else {
                        unimplemented!()
                    }
                } else {
                    unimplemented!("{:?}", entry)
                }
            }
            object_text.push_str("])");

            object_text
        }
        Expression::CallExpression(exp) => {
            let callee = expression_to_rust_text(&exp.callee);

            
            let mut arguments = Vec::<String>::with_capacity(exp.arguments.len());
            for arg in exp.arguments.iter() {
                let arg = arg.as_expression().unwrap();
                let arg = format!("({}).clone()", expression_to_rust_text(arg));
                arguments.push(arg);
            }
            let args_text = arguments.join(", ");
            
            let is_object = is_callee_an_object(&exp.callee);
            if is_object {
                format!("({callee}).call(&[{args_text}])")
            } else {
                format!("{callee}({args_text})")
            }
        }
        Expression::ArrayExpression(exp) => {
            let elements_text = exp
                .elements
                .iter()
                .map(|exp| {
                    let exp = exp.as_expression().unwrap();
                    expression_to_rust_text(exp)
                })
                .collect::<Vec<String>>()
                .join(", ");
            format!("JsValue::new_array(vec![{elements_text}])")
        }
        Expression::UpdateExpression(exp) => {
            update_expression_to_rust_text(exp)
        },
        Expression::Identifier(ident) => ident.name.to_string(),
        Expression::ParenthesizedExpression(exp) => {
            let exp_text = expression_to_rust_text(&exp.expression);
            format!("({exp_text})")
        },
        _ => unimplemented!("{:#?}", expression),
    }
}

fn is_callee_an_object(callee: &Expression) -> bool {
    match callee {
        Expression::FunctionExpression(_) => false,
        Expression::Identifier(_) => false,
        Expression::ArrowFunctionExpression(_) => false,
        Expression::ArrayExpression(_) => true,
        Expression::CallExpression(_) => true,
        Expression::ChainExpression(_) => true,
        Expression::ThisExpression(_) => true,
        Expression::ComputedMemberExpression(_) => true,
        Expression::StaticMemberExpression(_) => true,
        Expression::ParenthesizedExpression(exp) => is_callee_an_object(&exp.expression),
        _ => unreachable!()
    }
}

fn assignment_expression_to_rust_text(exp: &AssignmentExpression) -> String {
    let source = expression_to_rust_text(&exp.right);
    let operator = exp.operator;

    match &exp.left {
        AssignmentTarget::AssignmentTargetIdentifier(identifier) => {
            let target = identifier.name.as_str();

            let source = match operator {
                AssignmentOperator::Assign => source,
                AssignmentOperator::Addition => format!("{target}.add({source})"),
                AssignmentOperator::Subtraction => format!("{target}.sub({source})"),
                AssignmentOperator::Division => format!("{target}.div({source})"),
                AssignmentOperator::Multiplication => format!("{target}.mult({source})"),
                _ => unimplemented!()
            };

            format!("{target} = {source}")
        }
        AssignmentTarget::StaticMemberExpression(exp) => {
            let member_read = static_member_read_to_rust_text(exp);
            let source = match operator {
                AssignmentOperator::Assign => source,
                AssignmentOperator::Addition => format!("{member_read}.add({source})"),
                _ => unimplemented!()
            };
            static_member_write_to_rust_text(exp, &source)
        }
        AssignmentTarget::ComputedMemberExpression(exp) => {
            assert!(matches!(operator, AssignmentOperator::Assign));
            computed_member_write_to_rust_text(exp, &source)
        }
        _ => unimplemented!(),
    }
}

fn computed_member_read_to_rust_text(exp: &ComputedMemberExpression) -> String {
    let object = expression_to_rust_text(&exp.object);;
    let prop_name_value = expression_to_rust_text(&exp.expression);

    format!("{object}.get_prop(({prop_name_value}).clone())")
}

fn computed_member_write_to_rust_text(exp: &ComputedMemberExpression, value_expr: &str) -> String {
    let object = expression_to_rust_text(&exp.object);
    let prop_name_value = expression_to_rust_text(&exp.expression);

    format!("{object}.set_prop(({prop_name_value}).clone(), {value_expr})")
}

fn static_member_read_to_rust_text(exp: &StaticMemberExpression) -> String {
    let prop_name = exp.property.name.as_str();

    // Special cases for the Javascript standard "library"
    // TODO possibly a better approach is to actually create a static global `Math` object that the 
    // rust code can access much the same way as the JS version would do it.
    if let Expression::Identifier(ident) = &exp.object {
        if ident.name == "Math" {
            match prop_name {
                "PI" => return String::from("math().PI"),
                "sqrt" => return String::from("math().sqrt"),
                _ => ()
            }
        } else if ident.name == "process" {
            match prop_name {
                "argv" => return String::from("process().argv"),
                _ => ()
            }
        } else if ident.name == "console" {
            match prop_name {
                "log" => return String::from("console().log"),
                _ => ()
            }
        }
    }

    let object = expression_to_rust_text(&exp.object);
    let prop_name_value = format!("JsValue::from(\"{prop_name}\")");

    format!("{object}.get_prop({prop_name_value})")
}

fn static_member_write_to_rust_text(exp: &StaticMemberExpression, value_expr: &str) -> String {
    let object = expression_to_rust_text(&exp.object);
    let prop_name = exp.property.name.as_str();
    let prop_name_value = format!("JsValue::from(\"{prop_name}\")");

    format!("{object}.set_prop({prop_name_value}, {value_expr})")
}

/// This always returns the name of the equivalent function in our custom Rust impl
fn binary_operator_to_rust_text(operator: BinaryOperator) -> &'static str {
    match operator {
        BinaryOperator::Addition => "add",
        BinaryOperator::Subtraction => "sub",
        BinaryOperator::Division => "divide",
        BinaryOperator::LessThan => "less",
        BinaryOperator::Multiplication => "mult",
        _ => unimplemented!("{:?}", operator),
    }
}

fn unary_operator_to_rust_text(operator: UnaryOperator) -> &'static str {
    match operator {
        UnaryOperator::UnaryNegation => "negate",
        UnaryOperator::UnaryPlus => "plus",
        _ => unimplemented!(),
    }
}

fn assignment_operator_to_rust_text(operator: AssignmentOperator) -> &'static str {
    match operator {
        AssignmentOperator::Assign => "=",
        AssignmentOperator::Addition => "+=",
        AssignmentOperator::Subtraction => "-=",
        AssignmentOperator::Multiplication => "*=",
        _ => unimplemented!(),
    }
}

// fn assignment_tartet_to_rust_text(target: &AssignmentTarget) -> String {
//     match target {
//         AssignmentTarget::AssignmentTargetIdentifier(target) => {
//             target.name.to_string()
//         }
//         AssignmentTarget::StaticMemberExpression(exp) => {
//             static_member_expression_to_rust_text(exp, Write)
//         }
//         _ => unimplemented!()
//     }
// }
