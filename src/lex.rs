
#[derive(Debug, Eq, PartialEq, Clone)]
#[warn(dead_code)]
pub enum Token {
    END,

    DOLLAR,

    AT,
    HASH,

    PLUS,
    MINUS,
    STAR,
    DIV,

    LCURLY,
    RCURLY,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,

    DOT,
    COMMA,
    COLON,
    SEMICOLON,

    EQUAL,

    NUMBER(std::string::String),
    STRING(std::string::String),
    IDENTIFIER(std::string::String),

    VAR,

    MUT,
    CONST,

    PUB,
    PRIV,

    U32,
    I32,
    F32,
    BOOL,
    FN,
    TYPE,

    TRUE,
    FALSE,

    IF,
    ELSE,
    FOR,

    RET

}

// impl std::fmt::Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         // Use `self.number` to refer to each positional data point.
//         write!(f, "{}", self)
//     }
// }


pub struct Lexer {
    pub current: usize,
    pub program: Box<std::string::String>,
    // todo this is bad practice
    pub tokens: Box<Vec<Token>>
}

impl Lexer {

    pub fn new() -> Lexer {
        Lexer {
            current: 0, 
            program: Box::new(std::string::String::from("")),
            tokens: Box::new(vec!())
        }
    }

    pub fn lex(&mut self, program: Box<std::string::String>) {//-> Box<Vec<Token>>{

        self.program = program;

        while !self.end() {
            match self.program.chars().nth(self.current).unwrap() {
                '\n' => {},
                '\t' => {},
                '\r' => {},
                '$' => self.tokens.push(Token::DOLLAR),
                '@' => self.tokens.push(Token::AT),
                '#' => self.tokens.push(Token::HASH),
                '+' => self.tokens.push(Token::PLUS),
                '-' => self.tokens.push(Token::MINUS),
                '*' => self.tokens.push(Token::STAR),
                '/' => {
                    if self.program.chars().nth(self.current+1).unwrap() == '/' {
                        self.single_line_comment();
                    }else{
                        self.tokens.push(Token::DIV);
                    }
                },
                '{' => self.tokens.push(Token::LCURLY),
                '}' => self.tokens.push(Token::RCURLY),
                '(' => self.tokens.push(Token::LPAREN),
                ')' => self.tokens.push(Token::RPAREN),
                '[' => self.tokens.push(Token::LBRACKET),
                ']' => self.tokens.push(Token::RBRACKET),
                '.' => self.tokens.push(Token::DOT),
                ',' => self.tokens.push(Token::COMMA),
                ':' => self.tokens.push(Token::COLON),
                ';' => self.tokens.push(Token::SEMICOLON),
                '=' => self.tokens.push(Token::EQUAL),
                'b' => {
                    if self.is_keyword("bool".to_string()) {
                        self.tokens.push(Token::BOOL);
                        self.current+=3; // its only 3 because we + 1 later
                    }else{
                        // todo do identifier
                        self.other();
                        continue;
                    }
                },
                'c' => {
                    if self.is_keyword("const".to_string()) {
                        self.tokens.push(Token::CONST);
                        self.current+=4; // its only 3 because we + 1 later
                    }else{
                        // todo do identifier
                        self.other();
                        continue;
                    }
                },
                'e' => {
                    if self.is_keyword("else".to_string()) {
                        self.tokens.push(Token::ELSE);
                        self.current+=3; // its only 3 because we + 1 later
                    }else{
                        // todo do identifier
                        self.other();
                        continue;
                    }
                },
                'f' => {
                    if self.is_keyword("false".to_string()) {
                        self.tokens.push(Token::FALSE);
                        self.current+=4; // its only 2 because we + 1 later
                    }else if self.is_keyword("fn".to_string()) {
                        self.tokens.push(Token::FN);
                        self.current+=1; // its only 2 because we + 1 later
                    }else if self.is_keyword("f32".to_string()) {
                        self.tokens.push(Token::F32);
                        self.current+=2; // its only 2 because we + 1 later
                    }else if self.is_keyword("for".to_string()) {
                        self.tokens.push(Token::FOR);
                        self.current+=2; // its only 2 because we + 1 later
                    }else{
                        // todo do identifier
                        self.other();
                        continue;
                    }
                }
                'i' => {
                    if self.is_keyword("i32".to_string()) {
                        self.tokens.push(Token::I32);
                        self.current+=2; // its only 2 because we + 1 later
                    }else if self.is_keyword("if".to_string()) {
                        self.tokens.push(Token::IF);
                        self.current+=1; // its only 2 because we + 1 later
                    }else{
                        // todo do identifier
                        self.other();
                        continue;
                    }
                },
                'm' => {
                    if self.is_keyword("mut".to_string()) {
                        self.tokens.push(Token::MUT);
                        self.current+=2; // its only 3 because we + 1 later
                    }else{
                        // todo do identifier
                        self.other();
                        continue;
                    }
                },
                'p' => {
                    if self.is_keyword("pub".to_string()) {
                        self.tokens.push(Token::PUB);
                        self.current+=2; // its only 3 because we + 1 later
                    }else if self.is_keyword("priv".to_string()) {
                        self.tokens.push(Token::PRIV);
                        self.current+=3; // its only 3 because we + 1 later
                    }else{
                        // todo do identifier
                        self.other();
                        continue;
                    }
                },
                'r' => {
                    if self.is_keyword("ret".to_string()) {
                        self.tokens.push(Token::RET);
                        self.current+=2; // its only 2 because we + 1 later
                    }else{
                        // todo do identifier
                        self.other();
                        continue;
                    }
                },
                't' => {
                    if self.is_keyword("true".to_string()) {
                        self.tokens.push(Token::TRUE);
                        self.current+=3; // its only 2 because we + 1 later
                    }else if self.is_keyword("type".to_string()) {
                        self.tokens.push(Token::TYPE);
                        self.current+=3; // its only 2 because we + 1 later
                    }else{
                        // todo do identifier
                        self.other();
                        continue;
                    }
                },
                'u' => {
                    if self.is_keyword("u32".to_string()) {
                        self.tokens.push(Token::U32);
                        self.current+=2; // its only 2 because we + 1 later
                    }else{
                        // todo do identifier
                        self.other();
                        continue;
                    }
                },
                'v' => {
                    if self.is_keyword("var".to_string()) {
                        self.tokens.push(Token::VAR);
                        self.current+=2; // its only 2 because we + 1 later
                    }else{
                        // todo do identifier
                        self.other();
                        continue;
                    }
                },
                ' ' => {},
                _ => {
                    // todo do identifier
                    self.other();
                    continue;
                }
            }

            self.current+=1;

        }

        for token in self.tokens.iter() {
            println!("token {:?}", token);
        }

    }

    fn single_line_comment(&mut self) {
        self.current += 2;
        while self.program.chars().nth(self.current).unwrap() != '\n'
        && self.program.chars().nth(self.current).unwrap() != '\r' {
            self.current += 1;
        }
        //self.current+=1;
    }

    fn is_keyword(&self, keyword: std::string::String) -> bool {
        let mut matched = true;
        for i in 0..keyword.chars().count(){
            if self.program.chars().nth(self.current+i).unwrap() != 
                keyword.chars().nth(i).unwrap() {
                    matched = false;
            }
        }
        matched
    }

    fn end(&self) -> bool {
        return self.current >= self.program.chars().count();
    }

    fn other(&mut self) {
        let c = self.program.chars().nth(self.current).unwrap();
        if c.is_digit(10) {
            self.number();
        }else if c.is_alphabetic(){
            self.identifier();
        }else if c.eq_ignore_ascii_case(&'"') || c.eq_ignore_ascii_case(&'\''){
            self.string();
        }
    }

    fn number(&mut self){
        let mut s = std::string::String::from("");
        while !self.end() && 
            (self.program.chars().nth(self.current).unwrap().is_digit(10) 
            || (self.program.chars().nth(self.current).unwrap()=='.')){
            s.push(self.program.chars().nth(self.current).unwrap());
            self.current+=1;
        }
        self.tokens.push(Token::NUMBER(s));
    }

    fn identifier(&mut self){
        let mut s = std::string::String::from("");
        //println!("doing identifier for char {}.", self.program.chars().nth(self.current).unwrap());
        while !self.end() && (
            self.program.chars().nth(self.current).unwrap().is_alphabetic()
        || self.program.chars().nth(self.current).unwrap()=='_') {
            s.push(self.program.chars().nth(self.current).unwrap());
            self.current+=1;
        }
        //println!("s is {}.", s);
        self.tokens.push(Token::IDENTIFIER(s));
    }

    fn string(&mut self){
        let first_char = self.program.chars().nth(self.current).unwrap();
        self.current+=1;
        let mut s = std::string::String::from("");
        while !self.end() && 
        !self.program.chars().nth(self.current).unwrap().eq_ignore_ascii_case(&first_char){
            match self.program.chars().nth(self.current).unwrap(){
                '\\' => {
                    println!("found backslash!");
                    match self.program.chars().nth(self.current+1).unwrap() {
                        '"' => {s.push_str("\\\""); self.current+=2;},
                        '\'' => {s.push_str("\\'"); self.current+=2;},
                        'n' => {s.push_str("\\n"); self.current+=2;},
                        't' => {s.push_str("\\t"); self.current+=2;},
                        _ => {panic!()}
                    }

                },
                _ => {
                    s.push(self.program.chars().nth(self.current).unwrap());
                    self.current+=1;
                }
            }
        }
        self.current+=1;
        self.tokens.push(Token::STRING(s));
    }
}