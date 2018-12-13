use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};

type Scalar = i64;
type Vector = (Scalar, Scalar);

#[derive(Debug)]
struct Cart {
    pub position: Vector,
    pub velocity: Vector,
    pub alive: bool,
    pub counter: usize
}

impl Cart {
    pub fn new( position: Vector, velocity: Vector ) -> Self {
        Cart {
            position,
            velocity,
            alive: true,
            counter: 0
        }
    }

    pub fn from_char( position: Vector, c: char ) -> Self {
        let velocity = match c {
            '^' => (0, -1),
            'v' => (0,  1),
            '>' => (1,  0),
            '<' => (-1, 0),
            _   => panic!( "Invalid character" )
        };

        Self::new( position, velocity )
    }

    pub fn step( &mut self ) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }

    pub fn crossroad( &mut self ) {
        if self.counter == 0 {
            self.turn_left();
        } else if self.counter == 2 {
            self.turn_right();
        }

        self.counter = (self.counter + 1) % 3;
    }

    pub fn turn_right( &mut self ) {
        self.velocity = (
            -self.velocity.1,
            self.velocity.0
        );
    }

    pub fn turn_left( &mut self ) {
        self.velocity = (
            self.velocity.1,
            -self.velocity.0
        );
    }
}

fn parse_input<I: Iterator<Item=String>>( input: I ) -> (Vec<Cart>, HashMap<Vector, char>) {
    let mut carts = vec![];
    let mut map = HashMap::default();

    for (y, l) in input.enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '>' | '<' | 'v' | '^'   => { carts.push( Cart::from_char( (x as i64, y as i64), c ) ); },
                '/' | '\\' | '+'        => { map.insert( (x as i64, y as i64), c ); },
                _                       => continue
            }
        }
    }

    (carts, map)
}

fn part1<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let (mut carts, map) = parse_input( input );
    let mut positions = HashSet::<Vector>::default();

    loop {
        positions.clear();
        carts.sort_by_key( |c| (c.position.1, c.position.0) );

        for cart in &mut carts {
            if positions.contains( &cart.position ) {
                return format!( "{},{}", cart.position.0, cart.position.1 );
            }

            // XXX: This should probably be factored out..
            if let Some( c ) = map.get( &cart.position ) {
                match c {
                    '/' => {
                        if cart.velocity.1 != 0 {
                            cart.turn_right();
                        } else {
                            cart.turn_left();
                        }
                    },
                    '\\' => {
                        if cart.velocity.0 != 0 {
                            cart.turn_right();
                        } else {
                            cart.turn_left();
                        }
                    },
                    '+' => cart.crossroad(),
                    _ => {}
                }
            }

            cart.step();
            if ! positions.insert( cart.position ) {
                return format!( "{},{}", cart.position.0, cart.position.1 );
            }
        }
    }
}

fn part2<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let (mut carts, map) = parse_input( input );
    let mut positions = HashMap::<Vector, usize>::default();
    let mut crashed = 0;

    loop {
        if crashed == carts.len() - 1 {
            let (x, y) = carts
                            .into_iter()
                            .find( |c| c.alive )
                            .map( |c| c.position )
                            .unwrap();

            return format!( "{},{}", x, y );
        }

        positions.clear();
        carts.sort_by_key( |c| (c.position.1, c.position.0) );

        for i in 0 .. carts.len() {
            if ! carts[ i ].alive {
                continue;
            }

            // NOTE: This won't work when one cart is right after another
            if let Some( &old_i ) = positions.get( &carts[ i ].position ) {
                if carts[ old_i ].alive {
                    carts[ old_i ].alive = false;
                    crashed += 1;
                }

                carts[ i ].alive = false;
                crashed += 1;
                continue;
            }

            if let Some( c ) = map.get( &carts[ i ].position ) {
                match c {
                    '/' => {
                        if carts[ i ].velocity.1 != 0 {
                            carts[ i ].turn_right();
                        } else {
                            carts[ i ].turn_left();
                        }
                    },
                    '\\' => {
                        if carts[ i ].velocity.0 != 0 {
                            carts[ i ].turn_right();
                        } else {
                            carts[ i ].turn_left();
                        }
                    },
                    '+' => carts[ i ].crossroad(),
                    _ => {}
                }
            }

            carts[ i ].step();

            if let Some( old_i ) = positions.insert( carts[ i ].position, i ) {
                carts[ i ].alive = false;
                crashed += 1;

                if carts[ old_i ].alive {
                    carts[ old_i ].alive = false;
                    crashed += 1;
                }
            }
        }
    }
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
