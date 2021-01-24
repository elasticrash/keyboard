use kpt_lib::config;
use std::env;
use std::io::{self};
mod command;
use command::Command;
use kpt_lib::actions::dxf_io::{dxf_export, dxf_import};
use kpt_lib::geometry::exported_geometry::get_geometry;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: &str = args[1].as_ref();
    let keyboard = config::reader::read(format!("{}.json", file).as_str()).unwrap();
    let geometry = get_geometry(&keyboard);
    let drawing = dxf_import(&geometry);
    let name: String = file.to_owned();
    loop {
        println!("1. Export Dxf");
        println!("2. Export Ply");
        println!("3. Exit");

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Err(why) => panic!("couldn't read {:?}", why.raw_os_error()),
            _ => (),
        };

        let choice = match buffer.trim().parse::<Command>() {
            Ok(value) => {
                println!("..Executing option {:?}:", value);
                value
            }
            Err(why) => panic!("Invalid input ...exiting due to {:?}", why),
        };

        match choice {
            Command::ExportDxf => dxf_export(&drawing, &name),
            Command::ExportPly => println!("b"),
            Command::Exit => process::exit(0),
            _ => eprintln!("unknown number"),
        }
    }
}
