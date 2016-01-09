#![allow(dead_code)]
use nom::{digit, multispace};
use std::rc::Rc;
use token::{Token, LispyVal, LispyRet};

/*
   number     : [0-9]
   symbol     : number | alpha | operator | comparator
   comparator : < | > | == | >= | <= | != | & | \|
   operator   : '+' | '-' | '*' | '/'
   expr       : <symbol> | <operator> | <sexpr>'
   sexpr      : '(' <expr> * ')'
   qexpr      : '"' <expr> * '"' //TODO: replace this with a lispier macro language
*/
named!(pub tokenize< Vec<Token> >, 
  many0!( 
      alt!(operator | number | tokenize_beginsexpr | tokenize_endsexpr)
  )
);

named!(pub operator<&[u8], Token>,  chain!(
    opt!(multispace) ~
    val: alt!(tag!("+") | tag!("-") | tag!("/") | tag!("*")),
    || {
      Token::build_operator(val)
    }
));

named!(pub number<&[u8], Token>, chain!(
    opt!(multispace) ~
    val: digit,
    || {
      Token::build_number(val)
    }
));

named!(pub tokenize_beginsexpr<&[u8], Token>, chain!(
    opt!(multispace) ~
    val: tag!("("),
    || {
        Token::BeginSexpr
    }
));

named!(pub tokenize_endsexpr<&[u8], Token>, chain!(
    opt!(multispace) ~
    val: tag!(")"),
    || {
        Token::EndSexpr
    }
));

/*
named!(pub tokenize_comparator<&[u8], Token>, chain!(
    opt!(multispace) ~
    val: comparator,
    || {
      Token::Comparator(val)
    }
));


named!(pub terminal, alt!(digit | operation | comparator | multispace));
named!(pub operation, alt!(tag!("+") | tag!("-") | tag!("*") | tag!("/")));
named!(pub comparator, alt!(tag!("<") | tag!(">") | tag!("==") | tag!("<=") | tag!(">=") | tag!("&") | tag!("|")));
named!(string, delimited!(char!('"'), is_not!("\""), char!('"')));
*/
#[cfg(test)]
mod test{
    use nom::IResult;
    use nom::IResult::*;
    use token::Token;

/*
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
*/

}
