use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[derive(Debug)]
struct InvalidClaim(&'static str);
impl InvalidClaim {
    fn new( s: &'static str ) -> Self {
        InvalidClaim( s )
    }
}

struct Claim {
    id: usize,
    position: (usize, usize),
    size: (usize, usize)
}

impl std::str::FromStr for Claim {
    type Err = InvalidClaim;

    fn from_str( s: &str ) -> Result<Self, Self::Err> {
        if s.chars().next().ok_or( InvalidClaim::new( "empty" ) )? != '#' { return Err( InvalidClaim::new( "doesn't start with hash"  ) ); }

        let mut split = s[ 1.. ].split( " @ " );
        let id = split.next().and_then( |s| s.parse::<usize>().ok() ).ok_or( InvalidClaim::new( "no id" ) )?;

        let mut split = split.next().map( |s| s.split( ": " ) ).ok_or( InvalidClaim::new( "no colon" ) )?;

        let mut pos = split.next().map( |s| s.split( "," ) ).ok_or( InvalidClaim::new( "no comma" ) )?;
        let pos_x = pos.next().and_then( |s| s.parse::<usize>().ok() ).ok_or( InvalidClaim::new( "x pos is invalid" ) )?;
        let pos_y = pos.next().and_then( |s| s.parse::<usize>().ok() ).ok_or( InvalidClaim::new( "y pos is invalid" ) )?;

        let mut size = split.next().map( |s| s.split( "x" ) ).ok_or( InvalidClaim::new( "no 'x' delimiter" ) )?;
        let size_w = size.next().and_then( |s| s.parse::<usize>().ok() ).ok_or( InvalidClaim::new( "width is invalid" ) )?;
        let size_h = size.next().and_then( |s| s.parse::<usize>().ok() ).ok_or( InvalidClaim::new( "height is invalid" ) )?;


        Ok( Claim {
            id:         id,
            position:   (pos_x, pos_y),
            size:       (size_w, size_h)
        } )
    }
}

fn part1<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let mut fabric = [0; 1000 * 1000];

    for line in input {
        let claim = line.parse::<Claim>().unwrap();

        for y in claim.position.1..claim.position.1 + claim.size.1 {
            for x in claim.position.0..claim.position.0 + claim.size.0 {
                fabric[ y * 1000 + x ] += 1;
            }
        }
    }

    fabric.iter().filter( |&v| *v > 1 ).count()
}

fn part2<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let mut fabric = [0; 1000 * 1000];
    let mut not_overlapping = HashSet::<usize>::default();

    for line in input {
        let claim = line.parse::<Claim>().unwrap();
        not_overlapping.insert( claim.id );

        for y in claim.position.1..claim.position.1 + claim.size.1 {
            for x in claim.position.0..claim.position.0 + claim.size.0 {
                if fabric[ y * 1000 + x ] == 0 {
                    fabric[ y * 1000 + x ] = claim.id;
                } else {
                    not_overlapping.remove( & claim.id );
                    not_overlapping.remove( & fabric[ y * 1000 + x ] );
                }
            }
        }
    }

    not_overlapping.into_iter().next().expect( "invalid dataset" )
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
