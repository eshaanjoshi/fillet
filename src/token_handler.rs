pub mod built_in;
pub mod error_handler;
use std::collections::HashMap;
use built_in::token_enums::LiteralData;
use built_in::token_enums::Tokentype;
use built_in::create_new_keyword_dict;

lazy_static!{static ref  kw:HashMap<String, Tokentype> =create_new_keyword_dict();}

pub struct token
{
    t_type: Tokentype,
    lexeme: String,
    literal:LiteralData,
    line: usize,
}

trait Strrep{
    fn strrep(&self)->String;
}

impl token
{
    fn new(ty: Tokentype, lex:String, obj:LiteralData, li:usize) -> token
    {
        return token{ t_type: (ty), lexeme: (lex), literal: (obj), line: (li) }
    }
    fn _strrep(&mut self) -> String{
        return format!("Token type: {:?} {} {:?} {}", self.t_type, self.lexeme, self.literal, self.line);
    }
}

fn useless_token() -> token
{
    return token::new(Tokentype::USELESS, String::new(), LiteralData::NONE, 0);
}

fn advance(source:&String, current:&mut usize) -> char
{
    let c =  source.as_bytes()[*current] as char;
    *current+=1;
    return c;
}

fn add_token(ty:Tokentype, obj:LiteralData, line:&mut usize, current:usize, start:usize, source:&String)->token
{
    let text:String = source[start..current].to_string();
    let mut t = token::new(ty, text, obj, *line);
    if ty!=Tokentype::USELESS {println!("{}", t._strrep());}
    return t;
}

fn match_next(expected:char, current:&mut usize, start:usize, source:&String) -> bool
{
    if *current>=source.len() {return true;}
    if source.as_bytes()[*current] as char != expected {return false;}
    *current+=1;
    return true;
}

fn match_next_wrap(expected:char, current:&mut usize, start:usize, source:&String, iftrue:Tokentype, iffalse:Tokentype) -> Tokentype
{
    if match_next(expected, current, start, source) {return iftrue;}
    return iffalse;
}

fn peek(current:&mut usize, source:&String)->char
{
    if *current >= source.len() {return '\0'};
    return source.as_bytes()[*current] as char;
}

fn peek_next(current:&mut usize, source:&String) -> char
{
    return peek(&mut (*current+1), source);
}

fn parse_string(current:&mut usize, source:&String, line: &mut usize, start:usize)->token
{
    while peek(current, source)!='"' && (*current < source.len())
    {
        if peek(current, source) == '\n' {*line+=1;}
        advance(source, current);
    }
    if *current >= source.len()
    {
        error_handler::error(*line, "Unterminated String".to_string());
        return useless_token();
    }

    advance(source, current);
    let value: String = source[start+1..*current-1].to_string();
    return add_token(Tokentype::STRING, LiteralData::STR(value) , line, *current, start, source);
}

fn is_digit(c:char)->bool
{
    return c.is_ascii_digit();
}

fn is_alpha(c:char)->bool
{
    return c.is_ascii_alphabetic();
}

fn parse_number(c:char, current:&mut usize, source:&String, start:usize, mut line:usize)->token
{
    let mut is_decimal = false;
    while is_digit(peek(current, source)) {advance(source, current);}

    if peek(current, source) == '.' && is_digit(peek_next(current, source))
    {
        is_decimal=true;
        advance(source, current);
        while is_digit(peek(current, source))
        {
            advance(source, current);
        }
    }
    let value: String = source[start..*current].to_string();
    if is_decimal
    {
         return add_token(Tokentype::NUMBER, LiteralData::FLOAT(value.parse::<f32>().unwrap()), &mut line, *current, start, source);
    }
    return add_token(Tokentype::NUMBER, LiteralData::NUM(value.parse::<i32>().unwrap()), &mut line, *current, start, source);

}

fn parse_identifier(c:char, current:&mut usize, source:&String, start:usize, mut line:usize)->token
{
    while peek(current, source).is_ascii_alphanumeric()
    {
        advance(source, current);
    }
    let value: String = source[start..*current].to_string();
    let mut ttype:Tokentype;
    match kw.get(&value)
    {
        Some(ttoken) => ttype = *ttoken,
        None => ttype = Tokentype::IDENTIFIER,
    }
    return add_token(ttype, LiteralData::NONE, &mut line, *current, start, source);
}

fn scan_token(source:&String, current:&mut usize, start:usize, line:&mut usize) -> token
{
    let c:char = advance(source, current);
    println!("Char read {} {}", c, current);
    match c{
        '(' => return add_token(Tokentype::LEFTP, LiteralData::NONE, line, *current, start, source),
        ')' => return add_token(Tokentype::RIGHTP, LiteralData::NONE, line, *current, start, source),
        '[' => return add_token(Tokentype::LEFTB, LiteralData::NONE, line, *current, start, source),
        ']' => return add_token(Tokentype::RIGHTB, LiteralData::NONE, line, *current, start, source),
        ',' => return add_token(Tokentype::COMMA, LiteralData::NONE, line, *current, start, source),
        '.' => return add_token(Tokentype::PERIOD, LiteralData::NONE, line, *current, start, source),
        '-' => return add_token(Tokentype::MIN, LiteralData::NONE, line, *current, start, source),
        '+' => return add_token(Tokentype::PLUS, LiteralData::NONE, line, *current, start, source),
        ';' => return add_token(Tokentype::SEMI, LiteralData::NONE, line, *current, start, source),
        '*' => return add_token(Tokentype::STAR, LiteralData::NONE, line, *current, start, source),
        '!' => return add_token(match_next_wrap('=', current, start, source, Tokentype::BANG_EQUAL, Tokentype::BANG), LiteralData::NONE, line, *current, start, source),
        '=' => return add_token(match_next_wrap('=', current, start, source, Tokentype::EQUAL_EQUAL, Tokentype::EQUAL), LiteralData::NONE, line, *current, start, source),
        '<' => return add_token(match_next_wrap('=', current, start, source, Tokentype::LESS_EQUAL, Tokentype::LESS), LiteralData::NONE, line, *current, start, source),
        '>' => return add_token(match_next_wrap('=', current, start, source, Tokentype::GREATER_EQUAL, Tokentype::GREATER), LiteralData::NONE, line, *current, start, source),
        '/' =>{
            if match_next('/', current, start, source)
            {
                while peek(current, source) != '\n' && *current < source.len()
                {
                    advance(source, current);
                }
                println!("Comment");
                return useless_token();
            }
            else
            {
                return add_token(Tokentype::FSLASH, LiteralData::NONE, line, *current, start, source);
            }
        }
        '"' =>
        {
            return parse_string(current, source, line, start);
        }
        ' ' | '\r' | '\t' =>
        {
            return useless_token();
        }
        '\n' =>{
            *line+=1;
            return useless_token();
        }
        other => {
            if is_digit(c)
            {
                return parse_number(c, current, source, start, *line);
            }
            if is_alpha(c)
            {
                return parse_identifier(c, current, source, start, *line);
            }
            error_handler::error(*line, format!("Unexpected Character {}", other));
            return token::new(Tokentype::USELESS, "lol".to_string(), LiteralData::NONE, 1)
        }
    }
}

fn print_token_list(token_list:&mut Vec<token>)
{
    for tok in token_list.iter_mut(){
        println!("{}", tok._strrep());
    }
}




pub fn scan_tokens(source:String) -> Vec<token>
{
    let mut token_list:Vec<token> = Vec::new();
    let mut start:usize = 0;
    let mut current:usize = 0;
    let size = source.len();
    let mut line:usize=1;
    while !(current >= size)
    {
        start = current;
        let t = scan_token(&source, &mut current, start, &mut line);
        if t.t_type != Tokentype::USELESS {token_list.push(t)}
    }
    token_list.push(token::new(Tokentype::EOF, "".to_string(), LiteralData::NONE, line));
    print_token_list(&mut token_list);
    return token_list;
}