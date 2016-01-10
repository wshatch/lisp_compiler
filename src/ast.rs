#![allow(dead_code)]
use token::{Token, LispyVal, LispyRet, LispyError};
use operations::{identity};
use std::slice::Iter;
use std::{str};
use std::rc::Rc;


#[derive(PartialEq, Debug)]
pub struct Ast{
  tags: String,
  contents: LispyVal,
  children: Vec<Ast>
}

impl Ast {
  pub fn new(tags: &str, contents: Token, children: Vec<Ast>) -> Ast {
    return Ast{
      tags: tags.to_string(),
      contents: Rc::new(contents),
      children: children
    }
  }

  pub fn from_token_vec(tokens: Vec<Token>) -> Option<Ast>{
    let mut token_iterator = tokens.iter();
    let first_token = match token_iterator.next(){
      Some(t) => t,
      None => return None
    };
    return Ast::ast_from_token(first_token, &mut token_iterator);
  }

  pub fn eval(&self) -> LispyRet{
    match *self.contents {
      Token::BeginSexpr => return self.eval_sexpr(),
      Token::Func(a) => return self.eval_func(),
      _ => return Ok(self.contents.clone())
    };
  }

  fn eval_sexpr(&self) -> LispyRet{
    let (first, rest) = self.children.split_first().unwrap();
    let identity_ptr = identity as fn(Vec<LispyVal>) -> LispyRet;

    let func = match *first.contents {
      Token::Func(a) => a,
      _ => identity_ptr
    };

    let evald_children = rest.iter().map(|x| x.eval().unwrap()).collect::<Vec<_>>();

    return func(evald_children);
  }

  fn eval_func(&self) -> LispyRet{
    let func = match *self.contents {
      Token::Func(a) => a,
      _ => return Err(LispyError::ErrString("Tried to evaluate a non function".to_string()))
    };
    let evaled_children = self.children.iter().map(|x| x.eval().unwrap()).collect::<Vec<_>>();
    return func(evaled_children);
  }

  fn ast_from_token(token: &Token, rest: &mut Iter<Token>) -> Option<Ast> {
    match *token{
        Token::Number(i) => Some(Ast::build_number(i)),
        Token::BeginSexpr => Ast::build_sexpr(rest),
        Token::EndSexpr => None,
        Token::Func(i) => Some(Ast::new("func", Token::Func(i), vec![])),
        _ => None
    }
  }


  //TODO: refactor these methods into their own structs
  fn build_sexpr(iterator: &mut Iter<Token>) -> Option<Ast>{
    let mut current_ast = Ast::new("sexpr", Token::BeginSexpr, vec![]);
    let mut children: Vec<Ast> = vec![];

    loop {
      let current_token = match iterator.next(){
        Some(t) => t,
        None => break 
      };

      match Ast::ast_from_token(&current_token, iterator){
        Some(t) => children.push(t),
        None => break 
      }
    };

    current_ast.children = children;
    return Some(current_ast);
  }

  fn build_number(input: i32) -> Ast{
      return Ast::new("number", 
                      Token::Number(input), 
                      vec![])
  }

  fn build_operator(input:&[u8]) -> Ast{
      fn operator_func (x: Vec<LispyVal>) -> LispyRet{ Ok(Rc::new(Token::Number(4))) };
      return Ast::new("operator", Token::Func(operator_func), vec![])
  }

}

#[cfg(test)]
mod test{
  use super::*;
  use token::{Token};
  use std::rc::Rc;

  #[test]
  fn test_ast_from_token_vec(){
    // Test case for (+ 1 2 3)
    let expectation = Ast {tags:"sexpr".to_string(),
                           contents: Rc::new(Token::BeginSexpr),
                           children: vec![
                              Ast {tags: "number".to_string(),
                                  contents: Rc::new(Token::Number(1)),
                                  children: vec![]
                              },
                              Ast {tags: "number".to_string(),
                                  contents: Rc::new(Token::Number(2)),
                                  children: vec![]
                              },
                              Ast {tags: "number".to_string(),
                                  contents: Rc::new(Token::Number(3)),
                                  children: vec![]
                              }
                          ]};
    let subject_args = vec![Token::BeginSexpr, 
                            Token::Number(1), 
                            Token::Number(2), 
                            Token::Number(3), 
                            Token::EndSexpr];
    let subject = Ast::from_token_vec(subject_args).unwrap();
    assert_eq!(subject, expectation);
  }

  #[test]
  fn test_ast_eval(){
    // Test case for (+ 1 2 3)
    let subject = Ast {tags:"sexpr".to_string(),
                           contents: Rc::new(Token::BeginSexpr),
                           children: vec![
                              Ast {tags: "plus".to_string(),
                                   contents: Rc::new(Token::build_operator(&b"+"[..])),
                                   children: vec![]
                              },
                              Ast {tags: "number".to_string(),
                                  contents: Rc::new(Token::Number(1)),
                                  children: vec![]
                              },
                              Ast {tags: "number".to_string(),
                                  contents: Rc::new(Token::Number(2)),
                                  children: vec![]
                              },
                              Ast {tags: "number".to_string(),
                                  contents: Rc::new(Token::Number(3)),
                                  children: vec![]
                              }
                          ]};
    let expectation = Token::Number(6);
    assert_eq!(subject.eval(), Ok(Rc::new(expectation)));

  }

  #[test]
  fn test_ast_from_embeded_sexpr(){
    //Test case for (+ 1 2 (3))
    let expectation = Ast {tags:"sexpr".to_string(),
                           contents: Rc::new(Token::BeginSexpr),
                           children: vec![
                              Ast {tags: "number".to_string(),
                                  contents: Rc::new(Token::Number(1)),
                                  children: vec![]
                              },
                              Ast {tags: "number".to_string(),
                                  contents: Rc::new(Token::Number(2)),
                                  children: vec![]
                              },
                              Ast {tags: "sexpr".to_string(),
                                contents: Rc::new(Token::BeginSexpr),
                                children: vec![
                                  Ast {tags: "number".to_string(),
                                    contents: Rc::new(Token::Number(3)),
                                    children: vec![]
                                  }
                                ]
                              }
                          ]};
    let subject_args = vec![Token::BeginSexpr, 
                            Token::Number(1), 
                            Token::Number(2), 
                            Token::BeginSexpr,
                            Token::Number(3), 
                            Token::EndSexpr,
                            Token::EndSexpr];

    let subject = Ast::from_token_vec(subject_args).unwrap();
    assert_eq!(subject, expectation);
  }

}
