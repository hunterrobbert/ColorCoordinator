mod colorscheme;
mod constants;

use std::io::prelude::*;
use std::io;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::BufReader;

use colorscheme::ColorScheme;



fn generate_firefox_theme(c: &ColorScheme) -> io::Result<()> {
    println!("Generating: {}", "coordinated.css");

    let mut colors_f = try!(File::create("firefox/themed/color.scss"));

    write_on_color_loop(c, &mut colors_f, "$", ":   ", ";")
        .ok()
        .expect("Error writing color sass lines to new themed file");

    Command::new("sass")
            .arg("/home/hunter/Code/rust/color_coordinator/firefox/unthemed/base.scss")
            .arg("/home/hunter/Code/rust/color_coordinator/firefox/themed/coordinated.css")
            .output()
            .unwrap_or_else(|e| {panic!("failed to create css from firefox theme sass: {}", e) });

    return Ok(());
}

fn generate_vim_theme(c: &ColorScheme) -> io::Result<()> {
    println!("Generating: {}", constants::VIM);

    let mut themed_f = try!(File::create(get_themed_path(&constants::VIM)));

    let unthemed_f = match File::open(Path::new(&get_unthemed_path(&constants::VIM))) {
        Ok(unthemed_f) => unthemed_f,
        Err(..) => panic!("Error opening unthemed vim"),
    };

    let mut reader = BufReader::new(unthemed_f);
    let buffer_string = &mut String::new();

    reader.read_to_string(buffer_string)
        .ok()
        .expect("Error reading unthemed vim to buffer");

    for line in buffer_string.lines() {
        let mut linetowrite = line.to_string();
        let stripped = line.trim();
        //loop through all things to replace
        for i in 0..colorscheme::NUM_COLORS {
            let col: (String, String) = colorscheme::get_color_at_index(c, i as u32);
            let colorcheck = "let s:".to_string() + &col.0 + " = \"#";
            if stripped.starts_with(&colorcheck) {
                linetowrite = format!("     let s:{}    = \"{}\"", col.0, col.1);
            } 
        }
        try!(writeln!(themed_f, "{}", &linetowrite));
    }

    return Ok(());
    
}


fn generate_dmenu_script(c: &ColorScheme) -> io::Result<()> {
    println!("Generating: {}", constants::DMENU);

    let mut themed_f = try!(File::create(get_themed_path(&constants::DMENU)));

    try!(writeln!(themed_f, "#!/bin/bash"));

    let nf = colorscheme::get_color_at_index(c, 3);
    let nbsf = colorscheme::get_color_at_index(c, 1);
    let sb = colorscheme::get_color_at_index(c, 10);

    try!(writeln!(themed_f, "exec dmenu_run -nf '{}' -nb '{}' -sf '{}' -sb '{}'", &nf.1, &nbsf.1, &nbsf.1, &sb.1));

    return Ok(());
    
}

fn generate_themed_i3status(c: &ColorScheme) -> io::Result<()> {
    println!("Generating: {}", &constants::I3STATUS);

    let mut themed_f = try!(File::create(get_themed_path(&constants::I3STATUS)));
    
    let good = colorscheme::get_color_at_index(c, 15);
    let degraded = colorscheme::get_color_at_index(c, 4);
    let bad = colorscheme::get_color_at_index(c, 10);
    
    try!(writeln!(themed_f, "general {{"));
    try!(writeln!(themed_f, "   colors = true"));
    try!(writeln!(themed_f, "   color_good = '{}'", &good.1));
    try!(writeln!(themed_f, "   color_degraded = '{}'", &degraded.1));
    try!(writeln!(themed_f, "   color_bad = '{}'", &bad.1));
    try!(writeln!(themed_f, "   interval = 5"));
    try!(writeln!(themed_f, "}}"));
    
    let unthemed_f = match File::open(Path::new(&get_unthemed_path(&constants::I3STATUS))) {
        Ok(unthemed_f) => unthemed_f,
        Err(..) => panic!("Error opening unthemed i3status.conf"),
    };

    append_unthemed_to_themed(&unthemed_f, &mut themed_f)
        .ok()
        .expect("Error appending remaining i3status to new file");

   return Ok(()); 
}


fn generate_themed_file(c: &ColorScheme, name: &str, line: (&str, &str, &str)) -> io::Result<()> {
    println!("Generating: {}", name);

    let mut themed_f = try!(File::create(get_themed_path(name)));

    write_on_color_loop(c, &mut themed_f, line.0, line.1, line.2)
        .ok()
        .expect("Error writing colors to new themed file");


    let unthemed_f = match File::open(Path::new(&get_unthemed_path(name))) {
        Ok(unthemed_f) => unthemed_f,
        Err(..) => panic!("Error opening an unthemed file!"),
    };

    append_unthemed_to_themed(&unthemed_f, &mut themed_f)
        .ok()
        .expect("Error appending remaining code to new themed file!");

    return Ok(());
}



fn append_unthemed_to_themed(f_org: &File, f: &mut File) -> io::Result<()> {
    let mut reader = BufReader::new(f_org);
    let buffer_string = &mut String::new();

    reader.read_to_string(buffer_string)
        .ok()
        .expect("Error reading unthemed file"); 

    for line in buffer_string.lines() {
        try!(writeln!(f, "{}", &line))
    }

    return Ok(());
}

fn write_on_color_loop(c: &ColorScheme, file: &mut File, pre_n: &str, pst_n_pre_v: &str, pst_v: &str) -> io::Result<()> {
    for i in 0..colorscheme::NUM_COLORS {
        let col: (String, String) = colorscheme::get_color_at_index(c, i as u32);

        let line = pre_n.to_string() + &col.0 + pst_n_pre_v + &col.1 + pst_v;
        
        try!(writeln!(file, "{}", line));
    }

    return Ok(());
}

fn get_themed_path(f: &str) -> String {
    constants::THEMED_DIR.to_string() + f
}

fn get_unthemed_path(f: &str) -> String {
    constants::UNTHEMED_DIR.to_string() + f
}

fn clear_themed_dir(dir: &str) -> io::Result<()> {
    println!("Removing old dotfiles ...");

    for entry in try!(fs::read_dir(dir)) {
        //TODO: make it such that cargo run will still delete files even 
        //if not run from the color_coordinator home dir
        let file = try!(entry);
        println!("{:?}", file.path());
        try!(fs::remove_file(file.path()));
    }
    return Ok(());
}

fn list_available_themes() -> io::Result<()> {

    for entry in try!(fs::read_dir(constants::THEME_DIR)) {
        let file = try!(entry);
        let path = file.path();
        let ext = path.extension().unwrap();
        if ext == "txt" {
            //its a theme!
            let mut a = PathBuf::from(&path);
            a.set_extension("");
            let name = a.file_name().unwrap();
            println!("{:?}", name);
        }
    }

    println!("\n");

    return Ok(());
}


fn place_new_themed_file(path: &str, path_fin: &str) -> io::Result<()> {

    let mut finalpath = PathBuf::from(path_fin);
    finalpath.push(path);

    for entry in try!(fs::read_dir(path_fin)) {
        let file = try!(entry);
        let f_path = file.path();
        let name = f_path.file_name().unwrap();
        if name == path {
            println!("Replacing {:?}", name);
            try!(fs::remove_file(&f_path));
        }
    }


    try!(fs::copy(get_themed_path(path), finalpath.as_path()));

    return Ok(());
}


fn main() {
    println!("AVAILABLE THEMES:\n");
    list_available_themes().ok().expect("Error reading theme dir");

    println!("Type a themes name and press enter::\n");

    //wait for input
    let mut theme_choice = String::new();
    io::stdin().read_line(&mut theme_choice)
        .ok()
        .expect("Unable to read input");

    let theme_choice = theme_choice.trim();

    //generate new theme
    let mut c = ColorScheme::new();
    c.set_color(&theme_choice);

    for (i, f) in constants::get_supported_files().iter().enumerate() {
        println!("[{}]  {}", i, f);
    }

    println!("\nWhich files would you like to generate?");
    println!("Separate numbers by commas.");

    let mut file_choices = String::new();
    io::stdin().read_line(&mut file_choices)
        .ok()
        .expect("Couldn't interpret your file(s) choice");

    let opts: Vec<_> = file_choices.split(",").collect();

    for i in opts {
        println!("{}", i.trim());
        //TODO: finish changes to make file options work
    }

    


    //clear all colored filed
    clear_themed_dir(&constants::THEMED_DIR)
        .ok()
        .expect("Couldn't clear previously generated themed files");

    clear_themed_dir("firefox/themed")
        .ok()
        .expect("Couldn't clear firefox themed dir");

    // generate all new themed files
    let xres_args = ("#define S_", "   ", "");
    generate_themed_file(&c, &constants::XRES, xres_args)
        .ok()
        .expect("Couldn't write themed Xresource file!");

    let conf_args = ("set $", " ", "");
    generate_themed_file(&c, &constants::I3CONFIG, conf_args)
        .ok()
        .expect("Couldn't generate i3 config file!");

    generate_vim_theme(&c)
        .ok()
        .expect("Couldn't generate vim color theme file!");

    generate_dmenu_script(&c)
        .ok()
        .expect("Couldn't generate dmenu script file!");

    generate_themed_i3status(&c)
        .ok()
        .expect("Could not generate new I3STATUS file.");

    generate_firefox_theme(&c)
        .ok()
        .expect("Couldn't generate new firefox theme css");

    place_new_themed_file(&constants::XRES, &constants::XRES_FINAL_DEST)
        .ok()
        .expect("Couldn't place new XRES file in proper fs location.");

    place_new_themed_file(&constants::I3CONFIG, &constants::I3CONFIG_FINAL_DEST)
        .ok()
        .expect("Couldn't place new I3CONFIG file in proper fs location");

    place_new_themed_file(&constants::I3STATUS, &constants::I3STATUS_FINAL_DEST)
        .ok()
        .expect("Couldn't place new I3STATUS file in proper fs location");

    place_new_themed_file(&constants::VIM, &constants::VIM_FINAL_DEST)
        .ok()
        .expect("Couldn't place new VIM file in proper fs location.");

    place_new_themed_file(&constants::DMENU, &constants::DMENU_FINAL_DEST)
        .ok()
        .expect("Couldn't place new DMENU file in proper fs location.");

    Command::new("chmod")
        .arg("755")
        .arg("/home/hunter/.i3/dmenu_run_themed")
        .output()
        .unwrap_or_else(|e| {panic!("failed to chmod dmenu script: {}", e) });

}

