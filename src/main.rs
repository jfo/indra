#[macro_use]
extern crate nom;
use nom::*;

type Link = Option<Box<Cell>>;

#[derive(Debug)]
enum Car {
    Token(&'static[u8]),
    // Cell,
}

#[derive(Debug)]
struct Cell {
    car: Car,
    cdr: Link,
}

named!(token, ws!(alpha));

fn make_cell(input: &'static[u8]) -> Cell {
    match token(input) {
        IResult::Done(rest, matched) => Cell {
            car: Car::Token(matched),
            cdr: match token(rest) {
                IResult::Done(_, _) => Some(Box::new(make_cell(rest))),
                IResult::Incomplete(_) => None,
                IResult::Error(_) => panic!(),
            }
        },
        _ => panic!(),
    }
}

fn main(){
    let string = b"holy shit this worked";
    println!("{:?}", make_cell(string));
}
