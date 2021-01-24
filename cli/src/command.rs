use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    ExportDxf = 1,
    ExportPly,
    Exit,
    Unknown,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "1" {
            return Ok(Command::ExportDxf);
        }
        if s == "2" {
            return Ok(Command::ExportPly);
        }
        if s == "3" {
            return Ok(Command::Exit);
        }

        Ok(Command::Unknown)
    }
}
