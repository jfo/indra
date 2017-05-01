#[macro_use]
extern crate nom;
use nom::*;

type Link = Option<Box<Cell>>;

#[derive(Debug)]
struct Cell {
    car: Val,
    cdr: Link,
}

#[derive(Debug)]
enum Val {
    Token(&'static[u8]),
    List(Link)
}


named!(tokenval<&'static[u8], Val>, do_parse!(x : ws!(alpha) >> (Val::Token(x))));
named!(listval<&'static[u8], Val>, do_parse!(x : parens >> (Val::List(x))));

named!(tokens(&'static[u8]) -> Link, do_parse!(
        car : alt!(tokenval | listval)  >>
        cdr : opt!(tokens) >>
        (Some(Box::new(Cell {
            car: car,
            cdr: match cdr {
                Some(x) => x,
                None => None,
            }
        })))));

named!(parens<&'static[u8], Link>, delimited!(
        ws!(tag!("(")),
        alt!(tokens | parens),
        tag!(")")));

fn main(){
    let string = b"((a s) a)";
    println!("{:?}", parens(string));
}
