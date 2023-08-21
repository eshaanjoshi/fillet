use std::collections::HashMap;
pub mod token_enums;
use token_enums::Tokentype;
pub fn create_new_keyword_dict() -> HashMap<String, Tokentype> {
    let mut keyword_dict = HashMap::new();
    keyword_dict.insert("and".to_string(), Tokentype::AND);
    keyword_dict.insert("class".to_string(), Tokentype::CLASS);
    keyword_dict.insert("else".to_string(), Tokentype::ELSE);
    keyword_dict.insert("false".to_string(), Tokentype::FALSE);
    keyword_dict.insert("for".to_string(), Tokentype::FOR);
    keyword_dict.insert("fun".to_string(), Tokentype::FUN);
    keyword_dict.insert("if".to_string(), Tokentype::IF);
    keyword_dict.insert("nil".to_string(), Tokentype::NIL);
    keyword_dict.insert("or".to_string(), Tokentype::OR);
    keyword_dict.insert("print".to_string(), Tokentype::PRINT);
    keyword_dict.insert("return".to_string(), Tokentype::RETURN);
    keyword_dict.insert("super".to_string(), Tokentype::SUPER);
    keyword_dict.insert("this".to_string(), Tokentype::THIS);
    keyword_dict.insert("true".to_string(), Tokentype::TRUE);
    keyword_dict.insert("var".to_string(), Tokentype::VAR);
    keyword_dict.insert("while".to_string(), Tokentype::WHILE);

    return keyword_dict;
}
