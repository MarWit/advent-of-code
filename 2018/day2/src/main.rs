use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

fn part1<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let mut two_times = 0;
    let mut three_times = 0;

    for id in input {
        let local = id.chars().fold( HashMap::<char, usize>::default(), |mut a, e| {
            *a.entry( e ).or_insert( 0 ) += 1;
            a
        } ).values().cloned().collect::<HashSet<_>>();

        if local.contains( &2 ) {
            two_times += 1;
        }

        if local.contains( &3 ) {
            three_times += 1;
        }
    }

    two_times * three_times
}

fn differ_by_one( a: &str, b: &str ) -> Option<usize> {
    let mut found = None;

    for (i, (x, y)) in a.chars().zip( b.chars() ).enumerate() {
        if x != y {
            if found.is_none() {
                found = Some( i );
            } else {
                return None;
            }
        }
    }

    found
}

fn part2<I: Iterator<Item=String>>( input: I ) -> impl Display {
    // NOTE: Dataset is so small that O(n^2) solution is good enough

    let strings = input.collect::<Vec<_>>();

    for j in 0..strings.len() {
        for i in 0..strings.len() {
            if i == j { continue; }

            let diff = differ_by_one( & strings[ j ], & strings[ i ] );
            if let Some( k ) = diff {
                return strings[ i ]
                            .chars()
                            .enumerate()
                            .filter_map( |(i, c)| if i != k { Some( c ) } else { None } )
                            .collect::<String>();
            }
        }
    }

    // NOTE: Problem states that there exists exactly one such pair
    unreachable!()
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
