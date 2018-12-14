use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};

fn part1<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let count = input
                    .into_iter()
                    .next()
                    .and_then( |l| l.parse::<usize>().ok() )
                    .unwrap();

    let mut recipes = vec![3u8, 7u8];
    let (mut elf_a, mut elf_b) = (0, 1);

    while recipes.len() < count + 10 {
        let recipe = recipes[ elf_a ] + recipes[ elf_b ];

        if recipe >= 10 {
            recipes.push( recipe / 10 );
        }
        recipes.push( recipe % 10 );

        elf_a = (elf_a + recipes[ elf_a ] as usize + 1) % recipes.len();
        elf_b = (elf_b + recipes[ elf_b ] as usize + 1) % recipes.len();
    }

    recipes[ count.. ]
        .into_iter()
        .take( 10 )
        .map( |n| (n + 0x30) as char )
        .collect::<String>()
}

fn part2<I: Iterator<Item=String>>( input: I ) -> impl Display {
    let search = input
                    .into_iter()
                    .next()
                    .unwrap()
                    .chars()
                    .map( |c| c as u8 - 0x30 )
                    .collect::<Vec<_>>();

    let mut recipes = vec![3u8, 7u8];
    let (mut elf_a, mut elf_b) = (0, 1);
    let mut iter = 0;

    loop {
        let recipe = recipes[ elf_a ] + recipes[ elf_b ];

        if recipe >= 10 {
            if (recipe / 10) != search[ iter ] {
                iter = 0;
            }

            if (recipe / 10) == search[ iter ] {
                iter += 1;

                if iter == search.len() {
                    return recipes.len() + 1 - search.len();
                }
            }

            recipes.push( recipe / 10 );
        }

        if (recipe % 10) != search[ iter ] {
            iter = 0;
        }

        if (recipe % 10) == search[ iter ] {
            iter += 1;

            if iter == search.len() {
                return recipes.len() + 1 - search.len();
            }
        }

        recipes.push( recipe % 10 );

        elf_a = (elf_a + recipes[ elf_a ] as usize + 1) % recipes.len();
        elf_b = (elf_b + recipes[ elf_b ] as usize + 1) % recipes.len();
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
