use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
use std::i64;

struct Grid([[i64; 300]; 300]);

// Dynamic Programming approach
//
// DP[y][x] = DP[y - 1][x] + sum i=0..x A[y][i] | if 0 <= y - 1 < HEIGHT
//            sum i=0..x A[y][i]                | otherwise
//
// Then, sum of sub square at (x,y) of size (s) can be obtained in O(1) by
// DP[y + s - 1][x + s - 1] - DP[y - 1][x + s - 1] - DP[y + s - 1][x - 1] + DP[y - 1][x - 1]
// where DP[y][x] = 0 if x, y are out of bounds

impl Grid {
    fn power_level( x: i64, y: i64, serial: i64 ) -> i64 {
        let rack = x + 10;
        let mut power = rack * y;

        power += serial;
        power *= rack;
        power = ( power / 100 ) % 10;
        power -= 5;

        power
    }

    pub fn from_serial( serial: i64 ) -> Self {
        let mut grid = [[0; 300]; 300];

        for y in 0 .. 300 {
            let mut x_value = 0;

            for x in 0 .. 300 {
                x_value += Self::power_level( x as i64 + 1, y as i64 + 1, serial );

                grid[ y ][ x ] = x_value + grid
                                            .get( y - 1 )
                                            .and_then( |a| a.get( x ) )
                                            .cloned()
                                            .unwrap_or( 0 );
            }
        }

        Grid( grid )
    }

    pub fn get_square_power( &self, x: usize, y: usize, offset: usize ) -> i64 {
        assert!( x + offset < 300 );
        assert!( y + offset < 300 );

        (self.0)[ y + offset ][ x + offset ] -
            (self.0).get( y - 1 ).map( |a| a[ x + offset ] ).unwrap_or( 0 ) -
            (self.0)[ y + offset ].get( x - 1 ).cloned().unwrap_or( 0 ) +
            (self.0).get( y - 1 ).and_then( |a| a.get( x - 1 ) ).cloned().unwrap_or( 0 )
    }
}

fn part1<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let serial = input
                    .into_iter()
                    .next()
                    .and_then( |l| l.parse::<i64>().ok() )
                    .unwrap();

    let grid = Grid::from_serial( serial );

    (0 .. 298)
        .map( |y| (0 .. 298).map( move |x| (y, x) ) )
        .flatten()
        .map( |(y, x)| (grid.get_square_power( x, y, 2 ), (y, x)) )
        .max()
        .map( |(_, (y, x))| format!( "{},{}", x + 1, y + 1 ) )
        .unwrap()
}

fn part2<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let serial = input
                    .into_iter()
                    .next()
                    .and_then( |l| l.parse::<i64>().ok() )
                    .unwrap();

    let grid = Grid::from_serial( serial );

    let mut max = i64::MIN;
    let mut answer = (301, 301, 301);

    for offset in 0 .. 300 {
        for y in 0 .. 300 - offset {
            for x in 0 .. 300 - offset {
                let power = grid.get_square_power( x, y, offset );
                if power > max {
                    max = power;
                    answer = (x + 1, y + 1, offset + 1);
                }
            }
        }
    }

    format!( "{},{},{}", answer.0, answer.1, answer.2 )
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
