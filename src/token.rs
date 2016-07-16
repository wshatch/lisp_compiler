use std::rc::Rc;
use std::str;
use std::fmt;
use operations::{sum, subtract, product, divide};

#[derive(PartialEq, Debug)]
pub enum LispyError{
  ErrString(String),
  ErrLispyVal(LispyVal)
}
//TODO: figure out if this is actually needed
pub type LispyVal = Rc<Token>;
pub type LispyRet = Result<LispyVal, LispyError>;


#[derive(PartialEq, Debug)]
pub enum Token{
  Number(i32),
  Func(fn(Vec<LispyVal>) -> LispyRet),
  BeginSexpr,
  EndSexpr,
  List(Vec<LispyVal>),
  Error(String)
}

impl Token{
  //TODO: convert to a build_func function with a standard library
  pub fn build_operator(input: &[u8]) -> Token {
    let operation_string = match str::from_utf8(input){
      Ok(op_str) => op_str,
      Err(_) => return Token::Error("Parse Error: could not convert to string".to_string())
    };

    match operation_string{
      "+" => return Token::Func(sum),
      "-" => return Token::Func(subtract),
      "*" => return Token::Func(product),
      "/" => return Token::Func(divide),
      _ => return Token::Error("Parse Error: bad operation provided".to_string())
    };
  }

  pub fn build_number(input: &[u8]) -> Token {
    let str_input = match str::from_utf8(input) {
      Ok(i) => i,
      Err(_) => return Token::Error("Parser Error: tried to convert something to a number".to_string())
    };
    
    match str_input.parse() {
      Ok(i) => return Token::Number(i),
      Err(_)=> return Token::Error("unable to convert a string to a number".to_string())
    }

  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    let token_string = match *self {
      Token::Number(x) => x.to_string(),
      _ => "Foo".to_string()
    };
    write!(f, "{}", token_string)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use operations::*;

  #[test]
  fn builds_sum(){
    let subject = Token::build_operator(&b"+"[..]);
    let expectation = Token::Func(sum);
    assert_eq!(subject, expectation);
  }

  #[test]
  fn builds_subtract(){
    let subject = Token::build_operator(&b"-"[..]);
    let expectation = Token::Func(subtract);
    assert_eq!(subject, expectation);
  }

  #[test]
  fn builds_number(){
    let subject = Token::build_number(&b"1"[..]);
    let expectation = Token::Number(1);
    assert_eq!(subject, expectation);
  }

}
