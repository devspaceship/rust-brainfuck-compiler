use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub run: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let run = match args.next() {
            Some(arg) => arg == "--run" || arg == "-r",
            None => false,
        };

        Ok(Self { filename, run })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let c_code = transpile_to_c(&contents)?;
    fs::write("./tmp.c", c_code)?;

    Command::new("gcc").arg("./tmp.c").spawn()?.wait()?;

    if config.run {
        let output = Command::new("./a.out").output()?;
        print!("{}", String::from_utf8(output.stdout)?);
    }

    fs::remove_file("./tmp.c")?;

    Ok(())
}

fn transpile_to_c(contents: &str) -> Result<String, Box<dyn Error>> {
    let transpilation_table: HashMap<char, &str> = [
        ('>', "++ptr;"),
        ('<', "--ptr;"),
        ('+', "++*ptr;"),
        ('-', "--*ptr;"),
        ('.', "putchar(*ptr);"),
        (',', "*ptr = getchar();"),
        ('[', "while (*ptr) {"),
        (']', "}"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut c_code = String::new();
    for c in contents.chars() {
        if let Some(c_snippet) = transpilation_table.get(&c) {
            c_code.push_str(c_snippet);
            c_code.push('\n');
        }
    }

    insert_in_c_template(c_code)
}

fn insert_in_c_template(c_code: String) -> Result<String, Box<dyn Error>> {
    let template = fs::read_to_string(Path::new("./template.c"))?;
    Ok(template.replace("// おはようございます。", &c_code))
}
