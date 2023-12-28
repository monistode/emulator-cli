use std::io::Read;

use clap::Parser;
use monistode_emulator::common::{Processor, ProcessorContinue};
use monistode_emulator::executable::Executable;
use monistode_emulator::stack_processor::StackProcessor;

#[derive(Parser)]
struct Opts {
    executable: std::path::PathBuf,
}

fn main() {
    let opts = Opts::parse();
    let bytes = std::fs::read(opts.executable).unwrap();
    let executable = Executable::new(&bytes.into_boxed_slice());
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
