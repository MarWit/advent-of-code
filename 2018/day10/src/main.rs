use std::collections::HashSet;
use std::env;
use std::fmt::Display;
use std::fs;
use std::i64;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use nom::*;

type Scalar = i64;
type Vector = (Scalar, Scalar);

#[derive(Debug)]
struct Light {
    position: Vector,
    velocity: Vector
}

impl Light {
    fn update( &mut self ) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }
}

named!(number(&str) -> i64,
    map_res!(
        recognize!(
            pair!(
                opt!( char!( '-' ) ),
                digit
            )
        ),
        i64::from_str
    )
);

named!(parse_tuple(&str) -> Vector,
    do_parse!(
        char!( '<' )    >>
        opt!( space )   >>
        fst: number     >>
        char!( ',' )    >>
        space           >>
        snd: number     >>
        char!( '>' )    >>
        ((fst, snd))
    )
);

named!(parse_light(&str) -> Light,
    do_parse!(
        tag!( "position=" )     >>
        position: parse_tuple   >>
        space                   >>
        tag!( "velocity=" )     >>
        velocity: parse_tuple   >>
        (Light {
            position,
            velocity
        })
    )
);

fn separated( position: &Vector, positions: &HashSet<Vector> ) -> bool {
    for dy in -1 ..= 1 {
        for dx in -1 ..= 1 {
            if dy == 0 && dx == 0 { continue; }
            if positions.contains( &(position.0 + dx, position.1 + dy) ) {
                return false;
            }
        }
    }

    true
}

fn draw_text( points: & HashSet<Vector> ) {
    let (mut min_x, mut min_y) = (i64::MAX, i64::MAX);
    let (mut max_x, mut max_y) = (i64::MIN, i64::MIN);

    for point in points {
        min_x = min_x.min( point.0 );
        min_y = min_y.min( point.1 );
        max_x = max_x.max( point.0 );
        max_y = max_y.max( point.1 );
    }

    for y in min_y ..= max_y {
        for x in min_x ..= max_x {
            if points.contains( &(x, y) ) {
                print!( "#" );
            } else {
                print!( " " );
            }
        }

        println!( "" );
    }
}

// NOTE: Assumption: _text_ is assembled from not separated points.
// NOTE: Assumption 2: first occurrence of so defined _text_ is the correct one.
fn part1<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let mut lights = input.map( |i| parse_light( &i ).unwrap().1 ).collect::<Vec<_>>();
    let mut positions = HashSet::with_capacity( lights.len() );

    let mut seconds = 0;

    'outer: loop {
        seconds += 1;
        positions.clear();

        for light in &mut lights {
            light.update();
            positions.insert( light.position );
        }

        for position in &positions {
            if separated( position, &positions ) {
                continue 'outer;
            }
        }

        break;
    }

    draw_text( &positions );
    seconds
}

fn part2<I: Iterator<Item=String>>( input: I ) -> impl Display {
    part1( input )
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
