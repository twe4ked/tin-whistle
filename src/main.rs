use std::fmt;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Note {
    D1,
    E1,
    F1Sharp,
    G1,
    A1,
    B1,
    C1Sharp,

    D2,
    E2,
    F2Sharp,
    G2,
    A2,
    B2,
    C2Sharp,
}

#[derive(Debug)]
enum Item {
    Note(Note),
    Bar,
    Space,
    Gap,
}

impl From<char> for Item {
    fn from(input: char) -> Self {
        match input {
            'd' => Item::Note(Note::D1),
            'e' => Item::Note(Note::E1),
            'f' => Item::Note(Note::F1Sharp),
            'g' => Item::Note(Note::G1),
            'a' => Item::Note(Note::A1),
            'b' => Item::Note(Note::B1),
            'c' => Item::Note(Note::C1Sharp),

            'D' => Item::Note(Note::D2),
            'E' => Item::Note(Note::E2),
            'F' => Item::Note(Note::F2Sharp),
            'G' => Item::Note(Note::G2),
            'A' => Item::Note(Note::A2),
            'B' => Item::Note(Note::B2),
            'C' => Item::Note(Note::C2Sharp),

            '|' => Item::Bar,
            ' ' => Item::Space,
            '-' => Item::Gap,

            _ => panic!("bad char \"{}\"", input),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{: <3}",
            match self {
                Item::Note(note) => match note {
                    Note::D1 => "d",
                    Note::E1 => "e",
                    Note::F1Sharp => "f#",
                    Note::G1 => "g",
                    Note::A1 => "a",
                    Note::B1 => "b",
                    Note::C1Sharp => "c#",

                    Note::D2 => "D",
                    Note::E2 => "E",
                    Note::F2Sharp => "F#",
                    Note::G2 => "G",
                    Note::A2 => "A",
                    Note::B2 => "B",
                    Note::C2Sharp => "C#",
                },
                Item::Bar => "|",
                Item::Gap => "-",
                Item::Space => "",
            }
        )
    }
}

const D1: [bool; 6] = [true, true, true, true, true, true];
const E: [bool; 6] = [true, true, true, true, true, false];
const F: [bool; 6] = [true, true, true, true, false, false];
const G: [bool; 6] = [true, true, true, false, false, false];
const A: [bool; 6] = [true, true, false, false, false, false];
const B: [bool; 6] = [true, false, false, false, false, false];
const C: [bool; 6] = [false, false, false, false, false, false];
const D2: [bool; 6] = [false, true, true, true, true, true];

impl Item {
    fn is_high_octave(&self) -> bool {
        match self {
            Item::Note(note) => match note {
                Note::D1
                | Note::E1
                | Note::F1Sharp
                | Note::G1
                | Note::A1
                | Note::B1
                | Note::C1Sharp => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn hole_covered(&self, i: usize) -> bool {
        match self {
            Item::Note(note) => match note {
                Note::D1 => D1[i],
                Note::E1 | Note::E2 => E[i],
                Note::F1Sharp | Note::F2Sharp => F[i],
                Note::G1 | Note::G2 => G[i],
                Note::A1 | Note::A2 => A[i],
                Note::B1 | Note::B2 => B[i],
                Note::C1Sharp | Note::C2Sharp => C[i],
                Note::D2 => D2[i],
            },
            _ => unreachable!(),
        }
    }
}

fn read_stdin() -> String {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    String::from_utf8(input).expect("invalid input")
}

fn parse_items(input: String) -> Vec<Vec<Item>> {
    input
        .lines()
        .filter(|line| !line.starts_with("#") && !line.starts_with("T:"))
        .map(|line| line.chars().map(|c| Item::from(c)).collect())
        .collect()
}

fn main() {
    let input = read_stdin();
    let item_lines = parse_items(input);

    for items in item_lines {
        for item in &items {
            print!("{}", item);
        }
        println!();

        for item in &items {
            match item {
                Item::Bar => print!("|  "),
                _ => {
                    if item.is_high_octave() {
                        print!(".  ");
                    } else {
                        print!("   ");
                    }
                }
            }
        }
        println!();

        (0..6).for_each(|hole| {
            &items.iter().for_each(|item| match item {
                Item::Note(_) => {
                    if item.hole_covered(hole) {
                        print!("●  ");
                    } else {
                        print!("○  ");
                    }
                }
                Item::Space | Item::Bar => print!("{}", item),
                Item::Gap => print!("   "),
            });
            println!();
        });

        println!();
    }
}
