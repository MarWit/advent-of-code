use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use nom::*;
use bitvec::*;

type Rules = [bool; 32];

fn to_bitvec( input: &str ) -> BitVec {
    input
        .chars()
        .fold( BitVec::default(), |mut a, c| {
            a.push( if c == '#' { true } else { false } );
            a
        } )
}

fn to_bits( input: &str ) -> usize {
    input
        .chars()
        .fold( 0, |a, c| {
            (a << 1) | (if c == '#' { 1 } else { 0 })
        } )
}

named!(parse_header(&str) -> BitVec,
    do_parse!(
        tag!( "initial state: " )   >>
        initial: rest               >>
        (to_bitvec( initial ))
    )
);

named!(parse_rule(&str) -> (usize, bool),
    do_parse!(
        when: take!( 5 )                            >>
        tag!( " => " )                              >>
        what: alt!( char!( '#' ) | char!( '.' ) )   >>
        ((
            to_bits( when ),
            if what == '#' { true } else { false }
        ))
    )
);

fn leading_zeros( bits: &BitSlice ) -> usize {
    let mut num = 0;

    for bit in bits {
        if ! bit {
            num += 1;
        } else {
            break;
        }
    }

    num
}

fn trailing_zeros( bits: &BitSlice ) -> usize {
    let mut num = 0;

    for i in (0 .. bits.len()).rev() {
        if ! bits[ i ] {
            num += 1;
        } else {
            break;
        }
    }

    num
}

fn one_step( mut state: BitVec, rules: &Rules ) -> (i64, BitVec) {
    state >>= 3;
    (0 .. 3).for_each( |_| state.push( false ) );

    let mut now = 0;

    for i in 0 .. 5 {
        now <<= 1;
        now |= state[ i ] as u8;
    }

    let mut new_state = state.clone();

    for i in 0 .. state.len() - 4 {
        now = (now | state[ i + 4 ] as u8) & 0x1f;
        new_state.set( i + 2, rules[ now as usize ] );
        now <<= 1;
    }

    let leading = leading_zeros( &new_state );
    let trailing = trailing_zeros( &new_state );

    new_state <<= leading;
    (0 .. trailing).for_each( |_| { new_state.pop(); } );

    (3 - leading as i64, new_state)
}

fn part1<I: Iterator<Item=String>>( mut input: I ) -> impl Display {
    let mut state = input
                        .next()
                        .map( |s| parse_header( &s ).unwrap().1 )
                        .unwrap();

    input.next();

    let mut rules = [false; 32];

    for (rule, plant) in input.map( |s| parse_rule( &s ).unwrap().1 ) {
        rules[ rule ] = plant;
    }

    let mut index_shift = 0;

    for _ in 0 .. 20 {
        let (shift, new_state) = one_step( state, &rules );

        state = new_state;
        index_shift -= shift;
    }

    state
        .into_iter()
        .enumerate()
        .filter( |&(_, v)| v )
        .map( |(i, _)| i as i64 + index_shift )
        .sum::<i64>()
}

fn part2<I: Iterator<Item=String>>( mut input: I ) -> impl Display {
    let mut state = input
                        .next()
                        .map( |s| parse_header( &s ).unwrap().1 )
                        .unwrap();

    input.next();

    let mut rules = [false; 32];

    for (rule, plant) in input.map( |s| parse_rule( &s ).unwrap().1 ) {
        rules[ rule ] = plant;
    }

    let mut index_shift = 0;
    let mut cache = HashMap::<BitVec, (usize, i64)>::default();

    let mut cycle_i = 0;

    for i in 1 ..= 50_000_000_000 {
        let (shift, new_state) = one_step( state, &rules );

        state = new_state;
        index_shift -= shift;

        if cache.contains_key( &state ) {
            cycle_i = i;
            break;
        }

        cache.insert( state.clone(), (i, index_shift) );
    }

    let (old_i, old_shift) = *cache.get( &state ).unwrap();
    let (rel_shift, cycle_len) = (index_shift - old_shift, cycle_i - old_i);

    let remaining = 50_000_000_000 - cycle_i;
    index_shift += rel_shift * (remaining / cycle_len) as i64;

    for _ in 0 .. remaining % cycle_len {
        let (shift, new_state) = one_step( state, &rules );
        state = new_state;
        index_shift -= shift;
    }

    state
        .into_iter()
        .enumerate()
        .filter( |&(_, v)| v )
        .map( |(i, _)| i as i64 + index_shift )
        .sum::<i64>()
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
