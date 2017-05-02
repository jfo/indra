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

impl Cell {
    // pub fn to_string(self) -> u8 {
    // }
}


named!( token <&'static[u8], Val >, do_parse!(x : ws!(alpha) >> (Val::Token(x))));
named!( parens   <&'static[u8], Link>, delimited!(ws!(tag!("(")), items, ws!(tag!(")"))));
named!( list  <&'static[u8], Val >, do_parse!(x : parens     >> (Val::List(x))));

named!(
    items(&'static[u8]) -> Link,
    do_parse!(
        car : alt_complete!( token | list )  >>
        cdr : opt!(items) >>
        (Some(Box::new(Cell {
            car: car,
            cdr: match cdr {
                Some(x) => x,
                None => None,
            }
        })))
        )
    );


fn main(){
    let string = b"(
        plus (minus two one) one
    )";
    println!("{:?}", match parens(string) {
        IResult::Done(_, x) => x.unwrap().car.to_string(),
        _ => panic!()
    });
}
