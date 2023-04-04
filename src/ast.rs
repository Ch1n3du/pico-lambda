#[derive(Debug)]
pub enum AST {
    Variable(char),
    Function {
        parameter: char,
        body: Box<AST>,
    },
    Application {
        function: Box<AST>,
        argument: Box<AST>,
    },
}

impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AST::*;
        match self {
            Variable(ident) => write!(f, "{ident}"),
            Function { parameter, body } => write!(f, "λ{parameter}.{body}"),
            Application { function, argument } => write!(f, "({function} {argument})"),
        }
    }
}
