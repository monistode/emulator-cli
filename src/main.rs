use std::io::Read;

use clap::{Parser, ValueEnum};
use monistode_emulator::acc_processor::AccProcessor;
use monistode_emulator::cisc_processor::CiscProcessor;
use monistode_emulator::common::{Processor, ProcessorContinue};
use monistode_emulator::executable::Executable;
use monistode_emulator::risc_processor::RiscProcessor;
use monistode_emulator::stack_processor::StackProcessor;

#[derive(ValueEnum, Clone, Copy)]
#[clap(rename_all = "kebab-case")]
enum ProcessorType {
    Stack,
    Accumulator,
    Risc,
    Cisc,
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
struct Opts {
    executable: std::path::PathBuf,
    #[clap(long, short)]
    processor: ProcessorType,
}

fn main() {
    let opts = Opts::parse();
    let bytes = std::fs::read(opts.executable).unwrap_or_else(|e| {
        eprintln!("Failed to read file: {}", e);
        std::process::exit(1);
    });
    let executable = Executable::new(&bytes.into_boxed_slice());
    let result = match opts.processor {
        ProcessorType::Stack => run_stack_processor(executable),
        ProcessorType::Accumulator => run_accumulator_processor(executable),
        ProcessorType::Risc => run_risc_processor(executable),
        ProcessorType::Cisc => run_cisc_processor(executable),
    };
    match result {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_cisc_processor(executable: Executable) -> Result<(), String> {
    let mut cisc_processor = CiscProcessor::new();
    cisc_processor.load_executable(&executable)?;

    loop {
        match cisc_processor.run_command(
            |_, value| {
                print!("{}", value as u8 as char);
            },
            |_| {
                let mut buffer = [0u8; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                buffer[0] as u16
            },
        ) {
            ProcessorContinue::KeepRunning => {}
            ProcessorContinue::Error => {
                return Err("Failed to run command".to_string());
            }
            ProcessorContinue::Halt => return Ok(()),
        }
    }
}

fn run_risc_processor(executable: Executable) -> Result<(), String> {
    let mut risc_processor = RiscProcessor::new();
    risc_processor.load_executable(&executable)?;

    loop {
        match risc_processor.run_command(
            |_, value| {
                print!("{}", value as u8 as char);
            },
            |_| {
                let mut buffer = [0u8; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                buffer[0] as u16
            },
        ) {
            ProcessorContinue::KeepRunning => {}
            ProcessorContinue::Error => {
                return Err("Failed to run command".to_string());
            }
            ProcessorContinue::Halt => return Ok(()),
        }
    }
}

fn run_accumulator_processor(executable: Executable) -> Result<(), String> {
    let mut accumulator_processor = AccProcessor::new();
    accumulator_processor.load_executable(&executable)?;

    loop {
        match accumulator_processor.run_command(
            |_, value| {
                print!("{}", value as u8 as char);
            },
            |_| {
                let mut buffer = [0u8; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                buffer[0] as u16
            },
        ) {
            ProcessorContinue::KeepRunning => {}
            ProcessorContinue::Error => {
                return Err("Failed to run command".to_string());
            }
            ProcessorContinue::Halt => return Ok(()),
        }
    }
}

fn run_stack_processor(executable: Executable) -> Result<(), String> {
    let mut stack_processor = StackProcessor::new();
    stack_processor.load_executable(&executable)?;

    loop {
        match stack_processor.run_command(
            |_, value| {
                print!("{}", value as u8 as char);
            },
            |_| {
                let mut buffer = [0u8; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                buffer[0] as u16
            },
        ) {
            ProcessorContinue::KeepRunning => {}
            ProcessorContinue::Error => {
                return Err("Failed to run command".to_string());
            }
            ProcessorContinue::Halt => return Ok(()),
        }
    }
}
