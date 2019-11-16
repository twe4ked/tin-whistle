use std::fmt;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Octave {
    High,
    Low,
}

#[derive(Debug)]
struct Note {
    octave: Octave,
    value: Value,
}

impl Note {
    fn new(octave: Octave, value: Value) -> Self {
        Self { octave, value }
    }

    fn is_high_octave(&self) -> bool {
        match self.octave {
            Octave::High => true,
            Octave::Low => false,
        }
    }
}

#[derive(Debug)]
enum Value {
    D,
    E,
    FSharp,
    G,
    A,
    B,
    CSharp,
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
            'D' => Item::Note(Note::new(Octave::Low, Value::D)),
            'E' => Item::Note(Note::new(Octave::Low, Value::E)),
            'F' => Item::Note(Note::new(Octave::Low, Value::FSharp)),
            'G' => Item::Note(Note::new(Octave::Low, Value::G)),
            'A' => Item::Note(Note::new(Octave::Low, Value::A)),
            'B' => Item::Note(Note::new(Octave::Low, Value::B)),
            'C' => Item::Note(Note::new(Octave::Low, Value::CSharp)),

            'd' => Item::Note(Note::new(Octave::High, Value::D)),
            'e' => Item::Note(Note::new(Octave::High, Value::E)),
            'f' => Item::Note(Note::new(Octave::High, Value::FSharp)),
            'g' => Item::Note(Note::new(Octave::High, Value::G)),
            'a' => Item::Note(Note::new(Octave::High, Value::A)),
            'b' => Item::Note(Note::new(Octave::High, Value::B)),
            'c' => Item::Note(Note::new(Octave::High, Value::CSharp)),

            '|' => Item::Bar,
            ' ' => Item::Space,
            '-' => Item::Gap,

            _ => panic!("bad char \"{}\"", input),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{: <3}", self.as_str())
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
    fn as_str(&self) -> &'static str {
        match self {
            Item::Note(note) => match note.octave {
                Octave::Low => match note.value {
                    Value::D => "D",
                    Value::E => "E",
                    Value::FSharp => "F#",
                    Value::G => "G",
                    Value::A => "A",
                    Value::B => "B",
                    Value::CSharp => "C#",
                },
                Octave::High => match note.value {
                    Value::D => "d",
                    Value::E => "e",
                    Value::FSharp => "f#",
                    Value::G => "g",
                    Value::A => "a",
                    Value::B => "b",
                    Value::CSharp => "c#",
                },
            },
            Item::Bar => "|",
            Item::Gap => "-",
            Item::Space => "",
        }
    }

    fn hole_covered(&self, i: usize) -> bool {
        match self {
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
                        if item.hole_covered(hole) {
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
