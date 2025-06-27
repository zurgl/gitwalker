use clap::Parser;
use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
    },
    path::Path,
    result::Result,
    string::String,
    vec::Vec,
};

static ERROR_FAIL_TO_PARSE_CLI_ARGS: &str = "ERROR_FAIL_TO_PARSE_CLI_ARGS";
static ERROR_FOLDER_DO_NOT_EXIST: &str = "ERROR_PROCESS_ON_EXEC";

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = String::from("github"))]
    folder: String,
}

fn parse_config(config: BufReader<File>) -> Option<String> {
    let url = config
        .lines()
        .filter(|line| line.is_ok())
        .flatten()
        .filter(|line| line.starts_with("\turl ="))
        .filter_map(|line| line.split_once("=").map(|(_, url)| String::from(url.trim())))
        .collect::<Vec<String>>();

    url.first().map(String::from)
}

fn run(folder: &Path) -> Vec<String> {
    walkdir::WalkDir::new(folder)
        .min_depth(3)
        .max_depth(3)
        .into_iter()
        .flatten()
        .map(|entry| entry.path().to_owned())
        .filter(|path| path.ends_with(".git/config"))
        .flat_map(File::open)
        .flat_map(|file| parse_config(BufReader::new(file)))
        .collect::<Vec<String>>()
}

fn main() -> Result<(), &'static str> {
    env_logger::init();

    <Cli as Parser>::try_parse()
        .map_err(|_| ERROR_FAIL_TO_PARSE_CLI_ARGS)
        .map(|Cli { folder }| println!("{:#?}", run(Path::new(&folder))))
        .map_err(|_| ERROR_FOLDER_DO_NOT_EXIST)
}
