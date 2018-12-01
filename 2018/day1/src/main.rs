use std::collections::BTreeSet;
use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};

fn part1<I: Iterator<Item=String>>( input: I ) -> impl Display {
    input
        .map( |n| n.parse::<i32>().unwrap() )
        .sum::<i32>()
}

fn part2<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let data = input.map( |n| n.parse::<i32>().unwrap() ).collect::<Vec<_>>();
    let mut now = 0;
    let mut tree = BTreeSet::default();
    tree.insert( 0 );

    for num in data.into_iter().cycle() {
        now += num;
        if ! tree.insert( now ) { break; }
    }

    now
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
