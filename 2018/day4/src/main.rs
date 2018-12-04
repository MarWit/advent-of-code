#![feature(euclidean_division)]

use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::collections::HashMap;

use nom::*;

#[derive(Debug)]
enum Message {
    BeginsShift(usize),
    FallsAsleep,
    WakesUp
}

#[derive(Debug)]
struct LogEntry {
    date: (usize, usize, usize),
    time: (usize, usize),
    message: Message
}

named!(parse_message(&str) -> Message,
    do_parse!(
        result: alt!(
            do_parse!(
                tag!( "Guard #" ) >>
                guard: map_res!( digit, usize::from_str ) >>
                (Message::BeginsShift( guard ))
            ) |
            do_parse!(
                tag!( "falls asleep" ) >>
                (Message::FallsAsleep)
            ) |
            do_parse!(
                tag!( "wakes up" ) >>
                (Message::WakesUp)
            )
        ) >> (result)
    )
);

named!(parse_log(&str) -> LogEntry,
    do_parse!(
        char!( '[' )                                    >>
        year: map_res!( digit, usize::from_str )        >>
        char!( '-' )                                    >>
        month: map_res!( digit, usize::from_str )       >>
        char!( '-' )                                    >>
        day: map_res!( digit, usize::from_str )         >>
        many1!( space )                                 >>
        hour: map_res!( digit, usize::from_str )        >>
        char!( ':' )                                    >>
        minutes: map_res!( digit, usize::from_str )     >>
        char!( ']' )                                    >>
        many1!( space )                                 >>
        message: parse_message                          >>
        (LogEntry {
            date: (year, month, day),
            time: (hour, minutes),
            message: message
        })
    )
);

fn part1<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let mut guard = None;
    let mut sleeping = 0;

    let mut guards = HashMap::<usize, (usize, Vec<usize>)>::default();

    let mut logs = input.collect::<Vec<_>>();
    logs.sort();

    for entry in logs.into_iter().map( |l| parse_log( &l ).unwrap().1 ) {
        if let Some( gid ) = guard {
            match entry.message {
                Message::FallsAsleep        => sleeping = entry.time.1,
                Message::WakesUp            => {
                    let e = guards.entry( gid ).or_insert( (0, vec![0; 60]) );
                    (*e).0 += entry.time.1 - sleeping;
                    for i in sleeping .. entry.time.1 {
                        ((*e).1)[ i ] += 1;
                    }
                },
                Message::BeginsShift( new_gid ) => guard = Some( new_gid )
            }
        } else {
            if let Message::BeginsShift( gid ) = entry.message {
                guard = Some( gid );
            }
        }
    }

    let (gid, (_, schedule)) = guards.into_iter().max_by_key( |v| (v.1).0 ).unwrap();

    gid * schedule.into_iter().enumerate().max_by_key( |&(_, v)| v ).unwrap().0
}

fn part2<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let mut guard = None;
    let mut sleeping = 0;

    let mut guards = HashMap::<usize, Vec<usize>>::default();

    let mut logs = input.collect::<Vec<_>>();
    logs.sort();

    for entry in logs.into_iter().map( |l| parse_log( &l ).unwrap().1 ) {
        if let Some( gid ) = guard {
            match entry.message {
                Message::FallsAsleep        => sleeping = entry.time.1,
                Message::WakesUp            => {
                    let e = guards.entry( gid ).or_insert( vec![0; 60] );
                    for i in sleeping .. entry.time.1 {
                        (*e)[ i ] += 1;
                    }
                },
                Message::BeginsShift( new_gid ) => guard = Some( new_gid )
            }
        } else {
            if let Message::BeginsShift( gid ) = entry.message {
                guard = Some( gid );
            }
        }
    }

    guards
        .into_iter()
        .map( |(guard, schedule)| (guard, schedule.into_iter().enumerate().max_by_key( |&(_, v)| v ).unwrap()) )
        .max_by_key( |&(_, (_, v))| v )
        .map( |(g, (m, _))| g * m )
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
