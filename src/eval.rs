use nom_parse_funcs::{tokenize};
use nom::IResult::{Done};
use token::{Token};
use ast::{Ast};

pub fn eval(expression: String) -> Token{
  let tokens = match tokenize(expression.as_bytes()){
    Done(rest, tokens) => tokens,
    _ => panic!("AHHH!!! ERRROR!!!")
  };


  let ast = Ast::from_token_vec(tokens).unwrap();
  let ret_token= match ast.eval(){
    Ok(t) => t,
    _ => panic!("AHHH!! BAD EVAL!")
  };

  match *ret_token {
    Token::Number(i) => Token::Number(i),
    _ => panic!("AHH! SOMETHING BAD HAPPENED!")
  }
}

#[cfg(test)]
mod test{
  use super::*;
  use token::{Token};

  #[test]
  fn test_addition(){
    let subject = eval("(+ 1 (- 3 2))".to_string());
    let expectation = Token::Number(2);

    assert_eq!(subject, expectation);
  }
}
