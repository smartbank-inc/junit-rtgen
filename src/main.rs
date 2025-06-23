use junit_rtgen::process_junit_xml_streaming;
use std::io::{self, stdin};

fn main() -> io::Result<()> {
    // Process XML directly from stdin without loading everything into memory
    let file_times = process_junit_xml_streaming(stdin().lock());

    // Output in ParallelTests::RSpec::RuntimeLogger format
    for (file, time) in file_times {
        println!("{}:{}", file, time);
    }

    Ok(())
}
