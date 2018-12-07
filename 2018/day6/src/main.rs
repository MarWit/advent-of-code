use std::env;
use std::fmt::Display;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

struct ParseCoordError(Option<ParseIntError>);

impl fmt::Debug for ParseCoordError {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        if let Some( e ) = &self.0 {
            fmt::Debug::fmt( e,  f )
        } else {
            write!( f, "invalid coordinates" )
        }
    }
}

impl std::default::Default for ParseCoordError {
    fn default() -> Self {
        ParseCoordError( None )
    }
}

impl From<ParseIntError> for ParseCoordError {
    fn from( error: ParseIntError ) -> Self {
        ParseCoordError( Some( error ) )
    }
}

struct Coord(i64, i64);

impl FromStr for Coord {
    type Err = ParseCoordError;

    fn from_str( s: &str ) -> Result<Self, Self::Err> {
        let mut split = s.split( ", " );

        Ok( Coord(
            split.next().ok_or( ParseCoordError::default() ).and_then( |l| l.parse().map_err( Into::into ) )?,
            split.next().ok_or( ParseCoordError::default() ).and_then( |l| l.parse().map_err( Into::into ) )?
        ) )
    }
}

fn points_bbox( points: &[Coord] ) -> (Coord, Coord) {
    let (mut xmin, mut xmax) = (points[ 0 ].0, points[ 0 ].0);
    let (mut ymin, mut ymax) = (points[ 0 ].1, points[ 0 ].1);

    for point in points {
        xmin = i64::min( xmin, point.0 );
        xmax = i64::max( xmax, point.0 );
        ymin = i64::min( ymin, point.1 );
        ymax = i64::max( ymax, point.1 );
    }

    (Coord( xmin, xmax ), Coord( ymin, ymax ))
}

fn part1<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let points = input
                    .map( |l| l.parse::<Coord>().unwrap() )
                    .collect::<Vec<_>>();

    let (Coord( xmin, xmax ), Coord( ymin, ymax )) = points_bbox( & points );
    let mut size = vec![0; points.len()];

    for y in ymin ..= ymax {
        for x in xmin ..= xmax {
            let mut dists = points
                                .iter()
                                .enumerate()
                                .map( |(i,p)| ((x - p.0).abs() + (y - p.1).abs(), i) )
                                .collect::<Vec<_>>();

            dists.sort();

            if dists[ 0 ].0 != dists[ 1 ].0 {
                if x == xmin || x == xmax || y == ymin || y == ymax {
                    size[ dists[ 0 ].1 ] = -1;
                } else if size[ dists[ 0 ].1 ] != -1 {
                    size[ dists[ 0 ].1 ] += 1;
                }
            }
        }
    }

    size.into_iter()
        .max()
        .unwrap()
}

fn part2<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let points = input
                    .map( |l| l.parse::<Coord>().unwrap() )
                    .collect::<Vec<_>>();

    let (Coord( xmin, xmax ), Coord( ymin, ymax )) = points_bbox( & points );
    let mut size = 0;

    for y in ymin ..= ymax {
        for x in xmin ..= xmax {
            let dists = points
                            .iter()
                            .map( |p| (x - p.0).abs() + (y - p.1).abs() )
                            .sum::<i64>();

            if dists < 10_000 {
                size += 1;
            }
        }
    }

    size
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
