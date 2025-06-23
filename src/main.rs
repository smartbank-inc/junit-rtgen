use junit_rtgen::process_junit_xml;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Read all input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Process the XML and get file times
    let file_times = process_junit_xml(&input);

    // Output in ParallelTests::RSpec::RuntimeLogger format
    for (file, time) in file_times {
        println!("{}:{}", file, time);
    }

    Ok(())
}
