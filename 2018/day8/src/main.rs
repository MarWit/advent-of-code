use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

#[derive(Default)]
struct Node {
    pub nodes: Vec<Node>,
    pub metadata: Vec<usize>
}

impl Node {
    fn parse_node<I: Iterator<Item=usize>>( numbers: &mut I ) -> Self {
        let mut node = Node::default();

        let nodes_num = numbers.next().unwrap();
        let metadata_num = numbers.next().unwrap();

        for _ in 0 .. nodes_num {
            let child = Node::parse_node( numbers );
            node.nodes.push( child );
        }

        for _ in 0 .. metadata_num {
            node.metadata.push( numbers.next().unwrap() );
        }

        node
    }

    pub fn first_check( &self ) -> usize {
        let mut sum = self.metadata.iter().sum();

        for node in &self.nodes {
            sum += node.first_check();
        }

        sum
    }

    pub fn second_check( &self ) -> usize {
        if self.nodes.is_empty() {
            return self.metadata.iter().sum();
        }

        self.metadata
            .iter()
            .filter( |&idx| *idx > 0 )
            .filter_map( |&idx| self.nodes.get( idx - 1 ) )
            .map( |node| node.second_check() )
            .sum()
    }
}

impl FromIterator<usize> for Node {
    fn from_iter<I: IntoIterator<Item=usize>>( iter: I ) -> Self {
        Node::parse_node( &mut iter.into_iter() )
    }
}

fn part1<I: Iterator<Item=String>>( mut input: I ) -> impl Display {
    let line = input.next().unwrap();
    let license = line.split( " " ).map( |n| n.parse::<usize>().unwrap() );

    license.collect::<Node>().first_check()
}

fn part2<I: Iterator<Item=String>>( mut input: I ) -> impl Display {
    let line = input.next().unwrap();
    let license = line.split( " " ).map( |n| n.parse::<usize>().unwrap() );

    license.collect::<Node>().second_check()
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
