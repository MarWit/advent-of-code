use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};

trait Opposite {
    fn opposite( self ) -> Self;
}

impl Opposite for char {
    fn opposite( self ) -> Self {
        if self.is_lowercase() { self.to_uppercase().next().unwrap() }
        else if self.is_uppercase() { self.to_lowercase().next().unwrap() }
        else { self }
    }
}

fn reduce<I: Iterator<Item=char>>( input: I ) -> Vec<char> {
    let mut stack : Vec<char> = vec![];

    for unit in input {
        if let Some( last ) = stack.last() {
            if last.opposite() == unit {
                stack.pop();
            } else {
                stack.push( unit );
            }
        } else {
            stack.push( unit );
        }

    }

    stack
}

fn part1<I: Iterator<Item=String>>( mut input: I ) -> impl Display {
    let polymer = input.next().unwrap();
    let reduced = reduce( polymer.chars() );

    reduced.len()
}

fn part2<I: Iterator<Item=String>>( mut input: I ) -> impl Display {
    let polymer = input.next().unwrap();
    let reduced = reduce( polymer.chars() );

    let mut best = reduced.len();

    for c in (b'a' ..= b'z').map( |c| c as char ) {
        let new_reduced = reduce( reduced.iter().cloned().filter( |&m| m != c && m != c.opposite() ) );

        if new_reduced.len() < best {
            best = new_reduced.len()
        }
    }

    best
}

fn usage() -> &'static str {
    "usage: cargo run --release -- <part1|part2>"
}

fn main() -> Result<(), Box<std::error::Error>> {
    let part = env::args().nth( 1 ).unwrap_or_else( || "part1".into() );
    let first = match part.as_ref() {
        "part1"     => true,
        "part2"     => false,
        _           => return Err( usage().into() )
    };

    let input_file = env::args().nth( 2 ).unwrap_or_else( || "res/input".into() );

    let reader = BufReader::new( fs::File::open( input_file )? );
    let mut lines = reader.lines().map( |l| l.expect( "invalid input" ) );

    if first { println!( "{}", part1( &mut lines ) ) }
    else     { println!( "{}", part2( &mut lines ) ) };


    Ok( () )
}
