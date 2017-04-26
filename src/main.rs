#[macro_use]
extern crate nom;
use nom::*;

type Link = Option<Box<Cell>>;

#[derive(Debug)]
enum Val {
    Token(&'static[u8]),
    List(Link),
}

#[derive(Debug)]
struct Cell {
    car: Val,
    cdr: Link,
}

enum MatchReturn {
    Token(&'static[u8]),
    List(&'static[u8]),
}

named!(
    token(&'static[u8]) -> MatchReturn,
    do_parse!(
        x: ws!(alpha) >>
        (MatchReturn::Token(x))
    )
);
named!(
    list(&'static[u8]) -> MatchReturn,
    do_parse!(
        x: ws!(delimited!(tag!("("), take_until!(")"), tag!(")"))) >>
        (MatchReturn::List(x))
    )
);
named!(
    cell(&'static[u8]) -> MatchReturn,
    alt!(list | token)
);

fn make_cell(input: &'static[u8]) -> Cell {
    match cell(input) {
        IResult::Done(rest, matched) => Cell {
            car: match matched {
                MatchReturn::Token(x) => Val::Token(x),
                MatchReturn::List(x) => Val::List(Some(Box::new(make_cell(x))))
            },
            cdr: match cell(rest) {
                IResult::Done(_, _) => Some(Box::new(make_cell(rest))),
                IResult::Incomplete(_) => None,
                IResult::Error(_) => None
            }
        },
        _ => panic!(),
    }
}

fn main(){
    let string = b"(holy) crap it (works)";
    println!("{:?}", make_cell(string));
}
