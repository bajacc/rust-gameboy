use crate::gb::GameBoy;
use clap::{Args, Parser, Subcommand};
use rustyline::Editor;

pub struct Debugger {
    gb: GameBoy,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Cycle {
        #[arg(default_value_t = 1)]
        number: u64,
    },
    Step {
        #[arg(default_value_t = 1)]
        number: u64,
    },
    Exit,
}

impl Debugger {
    pub fn new(gb: GameBoy) -> Self {
        Debugger { gb: gb }
    }

    pub fn run(&mut self) {
        let mut rl = rustyline::Editor::<()>::new().unwrap();
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    if !self.parse_exec(line) {
                        break;
                    }
                }
                Err(e) if e.to_string() == "EOF" => break,
                Err(_) => (),
            }
        }
    }

    fn parse_exec(&mut self, mut line: String) -> bool {
        line.insert_str(0, ">> ");
        let cli = match Cli::try_parse_from(line.split_whitespace()) {
            Ok(cli) => cli,
            Err(e) => {
                println!("{}", e);
                return true;
            }
        };
        match cli.command {
            Commands::Exit => return false,
            Commands::Cycle { number } => {
                for _ in 0..number {
                    self.gb.cycle();
                }
            }
            Commands::Step { number } => {
                for _ in 0..number {
                    let pc = self.gb.cpu.pc;
                    while pc == self.gb.cpu.pc {
                        self.gb.cycle();
                    }
                }
            }
        }
        self.gb.disassemble(10);
        return true;
    }
}
