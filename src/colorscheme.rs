
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;


pub const NUM_COLORS: i32 = 16;


pub struct ColorScheme {
    pub base03: (String, String),
    pub base02: (String, String),
    pub base01: (String, String),
    pub base00: (String, String),
    pub base0: (String, String),
    pub base1: (String, String),
    pub base2: (String, String),
    pub base3: (String, String),
    pub yellow: (String, String),
    pub orange: (String, String),
    pub red: (String, String),
    pub magenta: (String, String),
    pub violet: (String, String),
    pub blue: (String, String),
    pub cyan: (String, String),
    pub green: (String, String),
}

impl Default for ColorScheme {
    fn default() -> ColorScheme {
        ColorScheme {
            base03: ("base_OC".to_string(), "fff".to_string()),
            base02: ("base_OB".to_string(), "fff".to_string()),
            base01: ("base_OA".to_string(), "fff".to_string()),
            base00: ("base_OO".to_string(), "fff".to_string()),
            base0: ("base_O1".to_string(), "fff".to_string()),
            base1: ("base_A1".to_string(), "fff".to_string()),
            base2: ("base_B2".to_string(), "fff".to_string()),
            base3: ("base_C3".to_string(), "fff".to_string()),
            yellow: ("yellow".to_string(), "fff".to_string()),
            orange: ("orange".to_string(), "fff".to_string()),
            red: ("red".to_string(), "fff".to_string()),
            magenta: ("magenta".to_string(), "fff".to_string()),
            violet: ("violet".to_string(), "fff".to_string()),
            blue: ("blue".to_string(), "fff".to_string()),
            cyan: ("cyan".to_string(), "fff".to_string()),
            green: ("green".to_string(), "fff".to_string()),
        }
    }
}

impl ColorScheme {
    pub fn new() -> ColorScheme {
        ColorScheme {
            ..Default::default()
        }
        
    }

    pub fn set_color(&mut self, desc: &str) {

        let mut path = PathBuf::from("/home/hunter/Code/rust/color_coordinator/src/schemes");
        path.push(desc);
        path.set_extension("txt");

        let f = match File::open(path.as_path()) {
            Ok(f) => f,
            Err(..) => panic!("error opening file"),
        };

        let mut reader = BufReader::new(&f);
        let buffer_string = &mut String::new();


       reader.read_to_string(buffer_string)
           .ok()
           .expect("Error reading color scheme text file!");
       
       for (i, line) in buffer_string.lines().enumerate() {
           if line.len() == 7 {
               set_color_at_index(self, i as u32, &line);

           }
       }


    }
}

pub fn get_color_at_index(c: &ColorScheme, idx: u32) -> (String, String) {
    let col: (&str, &str) = match idx {
        0 => (&c.base03.0, &c.base03.1),
        1 => (&c.base02.0, &c.base02.1),
        2 => (&c.base01.0, &c.base01.1),
        3 => (&c.base00.0, &c.base00.1),
        4 => (&c.base0.0, &c.base0.1),
        5 => (&c.base1.0, &c.base1.1),
        6 => (&c.base2.0, &c.base2.1),
        7 => (&c.base3.0, &c.base3.1),
        8 => (&c.yellow.0, &c.yellow.1),
        9 => (&c.orange.0, &c.orange.1),
        10 => (&c.red.0, &c.red.1),
        11 => (&c.magenta.0, &c.magenta.1),
        12 => (&c.violet.0, &c.violet.1),
        13 => (&c.blue.0, &c.blue.1),
        14 => (&c.cyan.0, &c.cyan.1),
        15 => (&c.green.0, &c.green.1),
        _ => unreachable!()
    };

    (col.0.to_string(), col.1.to_string())
}

pub fn set_color_at_index(c: &mut ColorScheme, idx: u32, value: &str) {
    match idx {
        0 => c.base03.1 = value.to_string(),
        1 => c.base02.1 = value.to_string(),
        2 => c.base01.1 = value.to_string(),
        3 => c.base00.1 = value.to_string(),
        4 => c.base0.1 = value.to_string(),
        5 => c.base1.1 = value.to_string(),
        6 => c.base2.1 = value.to_string(),
        7 => c.base3.1 = value.to_string(),
        8 => c.yellow.1 = value.to_string(),
        9 => c.orange.1 = value.to_string(),
        10 => c.red.1 = value.to_string(),
        11 => c.magenta.1 = value.to_string(),
        12 => c.violet.1 = value.to_string(),
        13 => c.blue.1 = value.to_string(),
        14 => c.cyan.1 = value.to_string(),
        15 => c.green.1 = value.to_string(),
        _ => unreachable!()
    }
}

