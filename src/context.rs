use std::collections::HashMap;
use crate::astnode::ASTNode;

pub struct Context {
    pub functions: HashMap<String, ASTNode>,
    pub variables: HashMap<String, f64>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new()
        }
    }
    
    pub fn evaluate(&mut self, node: &ASTNode) -> f64 {
        match node {
            ASTNode::Number(n) => *n,
            ASTNode::Identifier(name) => *self.variables.get(name).unwrap_or(&0.0),
            ASTNode::BinaryOp(left, op, right) => {
                let left_val = self.evaluate(left);
                let right_val = self.evaluate(right);
                match op {
                    '+' => left_val + right_val,
                    '-' => left_val - right_val,
                    '*' => left_val * right_val,
                    '/' => left_val / right_val,
                    _ => panic!("unbelievable operator")
                }
            }
            ASTNode::Call(name, args) => {
                let func = self.functions.get(name).unwrap_or_else(
                    || panic!("function doesn't exist: {}", name)).clone();
                if let ASTNode::Function(_, params, body) = func {
                    let mut new_context = Context {
                        functions: self.functions.clone(),
                        variables: HashMap::new()
                    };
                    for (param, arg) in params.iter().zip(args.iter()) {
                        let arg_val = self.evaluate(arg);
                        new_context.variables.insert(param.clone(), arg_val);
                    }
                    let mut result = 0.0;
                    for expr in body {
                        result = new_context.evaluate(&expr);
                    }
                    result
                } else {
                    panic!("it's not a function: {}", name);
                }
            }
            ASTNode::Program(nodes) => {
                let mut result = 0.0;
                for node in nodes{
                    match node {
                        ASTNode::Function(name, _, _) => {
                            self.functions.insert(name.clone(), node.clone());
                        }
                        _ => {
                            result = self.evaluate(node);
                        }
                    }
                }
                result
            }
            _ => panic!("cannot calculate!")
        }
    }
}
