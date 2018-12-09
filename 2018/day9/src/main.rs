use std::collections::VecDeque;
use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
use std::iter::{self, FromIterator};

use lazy_static::lazy_static;
use regex::Regex;

trait Rotate {
    fn rotate_right( &mut self, count: usize );
    fn rotate_left( &mut self, count: usize );
}

impl<T> Rotate for VecDeque<T> {
    fn rotate_right( &mut self, count: usize ) {
        if self.is_empty() { return; }

        for _ in 0 .. count {
            let el = self.pop_back().unwrap();
            self.push_front( el );
        }
    }

    fn rotate_left( &mut self, count: usize ) {
        if self.is_empty() { return; }

        for _ in 0 .. count {
            let el = self.pop_front().unwrap();
            self.push_back( el );
        }
    }
}

fn parse_input( s: &str ) -> (usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new( r"(\d+) players; last marble is worth (\d+) points" ).unwrap();
    }

    let cap = RE.captures( s ).unwrap();
    (cap[ 1 ].parse().unwrap(), cap[ 2 ].parse().unwrap())
}

fn play_game( players: usize, marbles: usize ) -> usize {
    let mut game = VecDeque::from_iter( iter::once( 0 ) );
    let mut scores = vec![0; players];

    for i in 1 ..= marbles {
        if (i % 23) == 0 {
            game.rotate_right( 7 );
            let num = game.pop_back().unwrap();
            scores[ i % players ] += num + i;
            game.rotate_left( 1 );
        } else {
            game.rotate_left( 1 );
            game.push_back( i );
        }
    }

    scores.into_iter().max().unwrap()
}

fn part1<I: Iterator<Item=String>>( mut input: I ) -> impl Display {
    input
        .next()
        .map( |l| parse_input( &l ) )
        .map( |(players, marbles)| play_game( players, marbles ) )
        .unwrap()
}

fn part2<I: Iterator<Item=String>>( mut input: I ) -> impl Display {
    input
        .next()
        .map( |l| parse_input( &l ) )
        .map( |(players, marbles)| play_game( players, marbles * 100 ) )
        .unwrap()
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
