use std::{collections::HashMap, ops::{DerefMut, Deref}};

use crate::{token_enums::LiteralData, token_handler::Token, error_handler::fatal_error, expres::Symbol};
#[derive(Clone)]
pub struct EnvDefinitions{
    value:HashMap<String, LiteralData>,
    enclosing:Box<Option<EnvDefinitions>>,
}

impl EnvDefinitions{
    pub fn new()-> EnvDefinitions{
        return EnvDefinitions{value: HashMap::new(), enclosing:Box::new(None)};
    }
    pub fn new_enclosed(enclosure:&mut EnvDefinitions)->EnvDefinitions{
        return EnvDefinitions{value:HashMap::new(), enclosing:Box::new(Some(enclosure.clone()))};
    }
    pub fn define(&mut self, name:String, val:LiteralData ){
        self.value.insert(name, val);
    }

    pub fn get(&mut self, name:Token)->Option<LiteralData>{
        let val = self.value.get(&name.lexeme);
        match val{
            None=> {
                match *self.enclosing{
                    None=> fatal_error("RUNTIME".to_string(), "Undefined Variable".to_string(), name.line),
                    Some(_)=> {
                        let mut valid = self.enclosing.clone().unwrap();
                        return valid.get(name);
                    }
                }
            }
            _=>(),
        }
        return val.cloned();
    }
    pub fn get_from_string(&mut self, name:String)->Option<LiteralData>{
        let val = self.value.get(&name);
        match val{
            None=> {
                match *self.enclosing{
                    None=> fatal_error("RUNTIME".to_string(), "Undefined Variable".to_string(), 0),
                    Some(_)=> {
                        let mut valid = self.enclosing.clone().unwrap();
                        return valid.get_from_string(name);
                    }
                }
            }
            _=>(),
        }
        return val.cloned();
    }
    pub fn assign_var(&mut self, sym:Symbol, expr:LiteralData){
        match self.value.get(&sym.name){
            Some(_) =>{
                self.value.insert(sym.name, expr);
            }
            None => {
                match *self.enclosing{
                    None=> fatal_error("RUNTIME".to_string(), "Undefined Variable".to_string(), sym.line),
                    Some(_)=> {
                        let mut valid = self.enclosing.clone().unwrap();
                        valid.assign_var(sym, expr);
                    }
                }
            }
        }
    }
}

impl Deref for EnvDefinitions{
    type Target = HashMap<String, LiteralData>;
    fn deref(&self) -> &Self::Target{
        return &self.value;
    }
}

impl DerefMut for EnvDefinitions{
    fn deref_mut(&mut self) -> &mut Self::Target{
        return &mut self.value;
    }
}
