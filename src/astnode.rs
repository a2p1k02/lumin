#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f64),
    Identifier(String),
    BinaryOp(Box<ASTNode>, char, Box<ASTNode>),
    Call(String, Vec<ASTNode>),
    Function(String, Vec<String>, Vec<ASTNode>),
    Program(Vec<ASTNode>),
}
