#[derive(Debug)]
pub enum Octave {
    High,
    Low,
}

#[derive(Debug)]
pub struct Note {
    pub octave: Octave,
    pub value: Value,
}

impl Note {
    fn new(octave: Octave, value: Value) -> Self {
        Self { octave, value }
    }

    pub fn is_high_octave(&self) -> bool {
        match self.octave {
            Octave::High => true,
            Octave::Low => false,
        }
    }
}

#[derive(Debug)]
pub enum Value {
    D,
    E,
    FSharp,
    G,
    A,
    B,
    CSharp,
}

#[derive(Debug)]
pub enum Item {
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

impl Item {
    pub fn as_str(&self) -> &'static str {
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
}

pub fn parse_items(input: String) -> Vec<Vec<Item>> {
    input
        .lines()
        .filter(|line| !line.starts_with("#") && !line.starts_with("T:"))
        .map(|line| line.chars().map(|c| Item::from(c)).collect())
        .collect()
}
