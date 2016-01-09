#![allow(dead_code)]
use token::{Token, LispyVal, LispyError, LispyRet};
use std::rc::Rc;

pub fn identity(vector: Vec<LispyVal>) -> LispyRet{
  let list = vector.iter().map( |x| x.clone()).collect::<Vec<_>>();
  return Ok(Rc::new(Token::List(list)));
}

pub fn sum(vector: Vec<LispyVal>) -> LispyRet {
  fn addition_function(acc: i32, item: &LispyVal) -> i32{
    return acc + convert_to_int(item);
  }

  return arithmetic(vector, addition_function);
}

pub fn subtract(vector: Vec<LispyVal>) -> LispyRet {
  fn addition_function(acc: i32, item: &LispyVal) -> i32{
    return acc - convert_to_int(item);
  }

  return arithmetic(vector, addition_function);
}

pub fn product(vector: Vec<LispyVal>) -> LispyRet {
  fn addition_function(acc: i32, item: &LispyVal) -> i32{
    return acc * convert_to_int(item);
  }

  return arithmetic(vector, addition_function);
}

pub fn divide(vector: Vec<LispyVal>) -> LispyRet {
  fn addition_function(acc: i32, item: &LispyVal) -> i32{
    return acc / convert_to_int(item);
  }

  return arithmetic(vector, addition_function);
}

fn convert_to_int(input: &LispyVal) -> i32 {
  match **input{
    Token::Number(i) => return i,
    _ => return 0
  }
}

fn arithmetic(vector: Vec<LispyVal>, applyFunc: fn(i32, &LispyVal)->i32) -> LispyRet {
  let (first, rest) = match vector.split_first() {
    Some(result) => result,
    None => return Ok(Rc::new(Token::Number(0)))
  };

  //TODO: validate tokens are numbers

  let initial_value = convert_to_int(&first);
  let result : i32 = rest.iter().fold(initial_value, applyFunc);
  return Ok(Rc::new(Token::Number(result)));
}

#[cfg(test)]
mod test {
  use super::*;
  use std::rc::Rc;
  use token::Token;

  #[test]
  fn sum_nothing(){
    let subject = sum(vec![]);
    let expectation = Rc::new(Token::Number(0));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn sum_one_number(){
    let subject = sum(vec![
      Rc::new(Token::Number(1))
    ]);
    let expectation = Rc::new(Token::Number(1));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn sum_two_numbers(){
    let subject = sum(vec![
      Rc::new(Token::Number(1)),
      Rc::new(Token::Number(2))
    ]);
    let expectation = Rc::new(Token::Number(3));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn sum_three_numbers(){
    let subject = sum(vec![
      Rc::new(Token::Number(1)),
      Rc::new(Token::Number(2)),
      Rc::new(Token::Number(3))
    ]);
    let expectation = Rc::new(Token::Number(6));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn subtract_nothing(){
    let subject = subtract(vec![]);
    let expectation = Rc::new(Token::Number(0));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn subtract_one_number(){
    let subject = subtract(vec![
      Rc::new(Token::Number(1))
    ]);
    let expectation = Rc::new(Token::Number(1));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn subtract_two_numbers(){
    let subject = subtract(vec![
      Rc::new(Token::Number(1)),
      Rc::new(Token::Number(2)),
    ]);
    let expectation = Rc::new(Token::Number(-1));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn subtract_three_numbers(){
    let subject = subtract(vec![
      Rc::new(Token::Number(4)),
      Rc::new(Token::Number(2)),
      Rc::new(Token::Number(1)),
    ]);
    let expectation = Rc::new(Token::Number(1));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn multiply_nothing(){
    let subject = product(vec![]);
    let expectation = Rc::new(Token::Number(0));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn multiply_one_number(){
    let subject = product(vec![
      Rc::new(Token::Number(1))
    ]);
    let expectation = Rc::new(Token::Number(1));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn multiply_two_numbers(){
    let subject = product(vec![
      Rc::new(Token::Number(2)),
      Rc::new(Token::Number(3)),
    ]);
    let expectation = Rc::new(Token::Number(6));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn multiply_three_numbers(){
    let subject = product(vec![
      Rc::new(Token::Number(2)),
      Rc::new(Token::Number(3)),
      Rc::new(Token::Number(4)),
    ]);
    let expectation = Rc::new(Token::Number(24));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn divide_nothing(){
    let subject = divide(vec![]);
    let expectation = Rc::new(Token::Number(0));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn divide_one_number(){
    let subject = divide(vec![
      Rc::new(Token::Number(1))
    ]);
    let expectation = Rc::new(Token::Number(1));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn divide_two_numbers(){
    let subject = divide(vec![
      Rc::new(Token::Number(4)),
      Rc::new(Token::Number(2)),
    ]);
    let expectation = Rc::new(Token::Number(2));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn divide_ignore_remainder(){
    let subject = divide(vec![
      Rc::new(Token::Number(3)),
      Rc::new(Token::Number(2)),
    ]);
    let expectation = Rc::new(Token::Number(1));
    assert_eq!(subject.unwrap(), expectation);
  }

  #[test]
  fn divide_three_numbers(){
    let subject = divide(vec![
      Rc::new(Token::Number(8)),
      Rc::new(Token::Number(2)),
      Rc::new(Token::Number(2)),
    ]);
    let expectation = Rc::new(Token::Number(2));
    assert_eq!(subject.unwrap(), expectation);
  }


}
