use nom::{IResult,Needed, digit, is_digit, space, alpha, multispace};
use std::{str, i32};
use std::borrow::Cow;

//TODO: each of these has a value
#[derive(PartialEq, Debug)]
pub enum Token<'a>{
  Number(&'a [u8]),
  Comparator(&'a [u8]),
  Operator(&'a [u8]),
  BeginSexpr(&'a [u8]),
  EndSexpr(&'a [u8]),
  Error(String)
}

pub enum Operation{
  Plus,
  List,
  Head,
  Tail,
}

#[derive(PartialEq, Debug)]
struct Ast{
  tags: String,
  contents: String,
  children: Vec<Ast>
}

impl Ast {
  pub fn new(tags: &str, contents: &str, children: Vec<Ast>) -> Ast {
    return Ast{
      tags: tags.to_string(),
      contents: contents.to_string(),
      children: children
    }
  }

  pub fn ast_from_token(token: &Token) -> Option<Ast> {
    match *token{
        Token::Number(ref i) => Some(Ast::build_number(i)),
        Token::Comparator(ref i) => Some(Ast::build_number(i)),
        Token::Operator(ref i) => Some(Ast::build_operator(i)),
        Token::BeginSexpr(_) => Some(Ast::new("sexpr", "(", vec![])),
        Token::EndSexpr(_) => None,
        _ => None
    }
  }

  pub fn from_token_vec(tokens: Vec<Token>) -> Option<Ast>{
    let mut token_iterable = tokens.into_iter();
    let first_token = match token_iterable.next(){
      Some(t) => t,
      None => return None
    };

    let mut current_ast = match Ast::ast_from_token(&first_token) {
      Some(a) => a,
      None => return None
    };

    let mut children : Vec<Ast> = vec![];

    loop {
      let current_token = match token_iterable.next(){
        Some(t) => t,
        None => break 
      };
      match Ast::ast_from_token(&current_token) {
        Some(t) => children.push(t),
        None => break
      }
    };
    current_ast.children = children;
    return Some(current_ast);
  }

  fn build_number(input:&[u8]) -> Ast{
      return Ast::new("number", str::from_utf8(input).unwrap(), vec![])
  }

  fn build_operator(input:&[u8]) -> Ast{
      return Ast::new("operator", str::from_utf8(input).unwrap(), vec![])
  }

}

named!(pub tokenize< Vec<Token> >, 
  many0!( 
      alt!(operator | number | tokenize_comparator | tokenize_beginsexpr | tokenize_endsexpr)
  )
);

named!(pub operator<&[u8], Token>,  chain!(
    opt!(multispace) ~
    val: alt!(tag!("+") | tag!("-") | tag!("/") | tag!("*")),
    || {
      Token::Operator(val)
    }
));

named!(pub number<&[u8], Token>, chain!(
    opt!(multispace) ~
    val: digit,
    || {
      Token::Number(val)
    }
));

named!(pub tokenize_comparator<&[u8], Token>, chain!(
    opt!(multispace) ~
    val: comparator,
    || {
      Token::Comparator(val)
    }
));

named!(pub tokenize_beginsexpr<&[u8], Token>, chain!(
    opt!(multispace) ~
    val: tag!("("),
    || {
        Token::BeginSexpr(val)
    }
));

named!(pub tokenize_endsexpr<&[u8], Token>, chain!(
    opt!(multispace) ~
    val: tag!(")"),
    || {
        Token::EndSexpr(val)
    }
));


/*
   number     : [0-9]
   symbol     : number | alpha | operator | comparator
   comparator : < | > | == | >= | <= | != | & | \|
   operator   : '+' | '-' | '*' | '/'
   expr       : <symbol> | <operator> | <sexpr>'
   sexpr      : '(' <expr> * ')'
   qexpr      : '"' <expr> * '"' //TODO: replace this with a lispier macro language
*/

named!(pub terminal, alt!(digit | operation | comparator | multispace));
named!(pub operation, alt!(tag!("+") | tag!("-") | tag!("*") | tag!("/")));
named!(pub comparator, alt!(tag!("<") | tag!(">") | tag!("==") | tag!("<=") | tag!(">=") | tag!("&") | tag!("|")));
named!(string, delimited!(char!('"'), is_not!("\""), char!('"')));

#[cfg(test)]
mod test{
    use super::*; 
    use super::Ast;
    use nom::IResult;
    use nom::IResult::*;
    use nom::Err;

    #[test]
    fn token_operator(){
      let subject = operator(&b"+ 12"[..]);
      let expectation = IResult::Done(&b" 12"[..], Token::Operator(&b"+"[..]));
      assert_eq!(subject, expectation);
    }

    #[test]
    fn token_number(){
      let subject = number(&b"12 34"[..]);
      let expectation = IResult::Done(&b" 34"[..], Token::Number(&b"12"[..]));
      assert_eq!(subject, expectation);
    }

    #[test]
    fn test_tokenize(){
      let subject = tokenize(&b"+++"[..]);
      let plus = &b"+"[..];
      let expectation = vec![Token::Operator(plus), 
                             Token::Operator(plus), 
                             Token::Operator(plus)];
      assert_eq!(subject, IResult::Done(&b""[..], expectation));
    }
    #[test]
    fn test_tokenize_multiple(){
      let subject = tokenize(&b"+(>)"[..]);
      let plus = &b"+"[..];
      let parens = &b"("[..];
      let arrow = &b">"[..];
      let endparens = &b")"[..];
      let expectation = vec![Token::Operator(plus),
                             Token::BeginSexpr(parens),
                             Token::Comparator(arrow),
                             Token::EndSexpr(endparens)];
      assert_eq!(subject, IResult::Done(&b""[..], expectation));
    }

    #[test]
    fn test_tokenize_numbers(){
      let subject = tokenize(&b"12 34"[..]);
      let expectation = vec![Token::Number(&b"12"[..]),
                             Token::Number(&b"34"[..])
                             ];
      assert_eq!(subject, IResult::Done(&b""[..], expectation));
    }

    #[test]
    fn test_ast_from_token_vec(){
      // Test case for (+ 1 2 3)
      let expectation = Ast {tags:"sexpr".to_string(),
                             contents: "(".to_string(),
                             children: vec![
                                Ast {tags: "number".to_string(),
                                    contents: "1".to_string(),
                                    children: vec![]
                                },
                                Ast {tags: "number".to_string(),
                                    contents: "2".to_string(),
                                    children: vec![]
                                },
                                Ast {tags: "number".to_string(),
                                    contents: "3".to_string(),
                                    children: vec![]
                                }
                            ]};
      let subject_args = vec![Token::BeginSexpr(&b"("[..]), 
                              Token::Number(&b"1"[..]), 
                              Token::Number(&b"2"[..]), 
                              Token::Number(&b"3"[..]), 
                              Token::EndSexpr(&b")"[..])];
      let subject = Ast::from_token_vec(subject_args).unwrap();
      assert_eq!(subject, expectation);
    }
}
