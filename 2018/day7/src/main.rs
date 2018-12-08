use std::collections::BinaryHeap;
use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

use lazy_static::lazy_static;
use regex::Regex;

type Graph = [[bool; 26]; 26];

#[derive(PartialEq, Eq)]
struct Node(usize);

impl PartialOrd for Node {
    fn partial_cmp( &self, other: &Self ) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp( &self.0 )
    }
}

impl Ord for Node {
    fn cmp( &self, other: &Self ) -> std::cmp::Ordering {
        other.0.cmp( &self.0 )
    }
}

fn parse_step( s: &str ) -> (char, char) {
    lazy_static! {
        static ref RE: Regex = Regex::new( r"Step ([A-Z]) must be finished before step ([A-Z]) can begin." ).unwrap();
    }

    let cap = RE.captures( s ).unwrap();
    (cap[ 1 ].chars().next().unwrap(), cap[ 2 ].chars().next().unwrap())
}

fn in_degrees( graph: &Graph ) -> Vec<usize> {
    let mut in_degree = vec![0; graph.len()];

    for l in graph {
        let iter = l.iter()
                    .enumerate()
                    .filter( |(_, &v)| v );

        for (i, _) in iter {
            in_degree[ i ] += 1;
        }
    }

    in_degree
}

fn toposort( graph: &Graph ) -> Vec<usize> {
    let mut in_degree = in_degrees( graph );
    let mut zero_degree = BinaryHeap::from_iter(
        in_degree
            .iter()
            .enumerate()
            .filter( |(_, &n)| n == 0 )
            .map( |(i, _)| Node( i ) )
    );

    let mut sorted = vec![];

    while let Some( Node( node ) ) = zero_degree.pop() {
        sorted.push( node );

        for (i, v) in graph[ node ].iter().enumerate() {
            if !v { continue; }

            in_degree[ i ] -= 1;

            if in_degree[ i ] == 0 {
                zero_degree.push( Node( i ) );
            }
        }

    }

    sorted
}

fn part1<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let mut graph = [[false; 26]; 26];

    for line in input {
        let (from, to) = parse_step( &line );

        graph[ from as usize - 65 ][ to as usize - 65 ] = true;
    }

    let sorted = toposort( & graph )
                        .into_iter()
                        .map( |i| (i as u8 + 65) as char )
                        .collect::<String>();

    sorted
}

// This solution is rather ugly as it is topological sort
// (which is already implemented above) with additional logic..
fn part2<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let mut graph = [[false; 26]; 26];

    for line in input {
        let (from, to) = parse_step( &line );

        graph[ from as usize - 65 ][ to as usize - 65 ] = true;
    }

    let mut in_degree = in_degrees( & graph );
    let mut zero_degree = BinaryHeap::from_iter(
        in_degree
            .iter()
            .enumerate()
            .filter( |(_, &n)| n == 0 )
            .map( |(i, _)| Node( i ) )
    );

    let mut elves = vec![];
    let mut time = 0;

    while zero_degree.len() > 0 || elves.len() > 0 {
        while elves.len() < 5 && zero_degree.len() > 0 {
            let Node( i ) = zero_degree.pop().unwrap();
            elves.push( (i, 0) );
        }

        let (idx, (i, done)) = elves
                                .iter()
                                .cloned()
                                .enumerate()
                                .min_by_key( |&(_, (i, d))| 61 + i - d )
                                .unwrap();


        for (j, v) in graph[ i ].iter().enumerate() {
            if !v { continue; }

            in_degree[ j ] -= 1;

            if in_degree[ j ] == 0 {
                zero_degree.push( Node( j ) );
            }
        }

        elves.remove( idx );
        let diff = i + 61 - done;

        for (_, w) in &mut elves {
            *w += diff;
        }

        time += diff;
    }

    time
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
