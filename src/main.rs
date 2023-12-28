use std::io::Read;

use clap::{Parser, ValueEnum};
use monistode_emulator::cisc_processor::CiscProcessor;
use monistode_emulator::common::{Processor, ProcessorContinue};
use monistode_emulator::executable::Executable;
use monistode_emulator::stack_processor::StackProcessor;

#[derive(ValueEnum, Clone, Copy)]
#[clap(rename_all = "kebab-case")]
enum ProcessorType {
    Stack,
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
    let bytes = std::fs::read(opts.executable).unwrap();
    let executable = Executable::new(&bytes.into_boxed_slice());
    match opts.processor {
        ProcessorType::Stack => run_stack_processor(executable),
        ProcessorType::Cisc => run_cisc_processor(executable),
    }
}

fn run_cisc_processor(executable: Executable) {
    let mut cisc_processor = CiscProcessor::new();
    cisc_processor.load_executable(&executable);

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
            ProcessorContinue::Halt => break,
        }
    }
}

fn run_stack_processor(executable: Executable) {
    let mut stack_processor = StackProcessor::new();
    stack_processor.load_executable(&executable);

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
            ProcessorContinue::Halt => break,
        }
    }
}
