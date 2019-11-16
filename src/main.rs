mod abc_notation;

use abc_notation::{parse_items, Item, Octave, Value};
use std::io;
use std::io::prelude::*;

const D1: [bool; 6] = [true, true, true, true, true, true];
const E: [bool; 6] = [true, true, true, true, true, false];
const F: [bool; 6] = [true, true, true, true, false, false];
const G: [bool; 6] = [true, true, true, false, false, false];
const A: [bool; 6] = [true, true, false, false, false, false];
const B: [bool; 6] = [true, false, false, false, false, false];
const C: [bool; 6] = [false, false, false, false, false, false];
const D2: [bool; 6] = [false, true, true, true, true, true];

fn hole_covered(item: &Item, i: usize) -> bool {
    match item {
        Item::Note(note) => match note.value {
            Value::E => E[i],
            Value::FSharp => F[i],
            Value::G => G[i],
            Value::A => A[i],
            Value::B => B[i],
            Value::CSharp => C[i],
            Value::D => match note.octave {
                Octave::Low => D1[i],
                Octave::High => D2[i],
            },
        },
        _ => unreachable!(),
    }
}

fn read_stdin() -> String {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    String::from_utf8(input).expect("invalid input")
}

fn main() {
    let input = read_stdin();
    let item_lines = parse_items(input);

    for items in item_lines {
        for item in &items {
            print!("{: <3}", item.as_str())
        }
        println!();

        for item in &items {
            match item {
                Item::Bar => print!("|  "),
                Item::Note(note) => {
                    if note.is_high_octave() {
                        print!(".  ");
                    } else {
                        print!("   ");
                    }
                }
                _ => print!("   "),
            }
        }
        println!();

        (0..6).for_each(|hole| {
            &items.iter().for_each(|item| {
                let c = match item {
                    Item::Note(_) => {
                        if hole_covered(item, hole) {
                            "●"
                        } else {
                            "○"
                        }
                    }
                    Item::Space | Item::Bar => item.as_str(),
                    Item::Gap => "",
                };
                print!("{: <3}", c)
            });
            println!();
        });

        println!();
    }
}
