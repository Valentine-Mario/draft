use peg::parser;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Number(u64),
    Sum(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Ref(String),
    Assign(String, Box<Expression>),
    If(Box<Expression>, Vec<Expression>, Vec<Expression>),
}


parser!{
    /// Doc comment
    pub grammar arithmetic() for str {
        /// Top level parser rule
        /// This doc comment has multiple lines to test support for that as well
        pub rule expression() -> Vec<Expression>
            = statements()
    
        rule _() = [' ' | '\n']*

        rule statements() -> Vec<Expression>
           = e:(equal() ** ("\n" _)) "\n"? { e }

        rule equal()->Expression
            = if_statement()
            /i:identifier() _ "=" _ s:sum() {Expression::Assign(i, Box::new(s))} 
            /sum()
        
        rule if_statement() ->Expression
            = "if" _ e:equal() _ "{" _"\n"? _ then_statement:statements() _ "}" _ "\n"? _ "else" _ "{" _ "\n"? _ else_statement:statements() _ "}" {
                Expression::If(Box::new(e), then_statement, else_statement)
            }
    
        rule sum() -> Expression
            = l:product() _ "+" _ r:product() { Expression::Sum(Box::new(l), Box::new(r)) }
            / l:product() _ "-" _ r:product() {Expression::Subtract(Box::new(l), Box::new(r))}
            /"inc" _ l:atom() {Expression::Sum(Box::new(l), Box::new(Expression::Number(1)))}
            /"dec" _ l:atom() {Expression::Subtract(Box::new(l), Box::new(Expression::Number(1)))}
            / product()
           
    
        rule product() -> Expression
            = l:atom() _ "*" _ r:atom() { Expression::Product(Box::new(l), Box::new(r)) }
            / l:atom() _ "/" _ r:atom() { Expression::Division(Box::new(l), Box::new(r)) }
            /"square" _ l:atom() { Expression::Product(Box::new(l.clone()), Box::new(l)) }
            / atom()
            

        rule atom() -> Expression
            = reference()
            / "(" _ v:sum() _ ")" { v }

        rule reference()->Expression
            =l:identifier() {Expression::Ref(l)}
            /number()

        rule identifier()->String
            = n:$(['a'..='z'|'A'..='Z']+){n.to_string()}
            
    
        rule number() -> Expression
            = n:$(['0'..='9']+) { Expression::Number(n.parse().unwrap()) }
    }}