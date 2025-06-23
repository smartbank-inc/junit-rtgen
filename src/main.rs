use quick_xml::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug, Deserialize)]
struct TestSuite {
    #[serde(rename = "$value")]
    elements: Vec<TestSuiteElement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TestSuiteElement {
    Properties(Properties),
    TestCase(TestCase),
}

#[derive(Debug, Deserialize)]
struct Properties {
    // We don't need the contents for this use case
}

#[derive(Debug, Deserialize)]
struct TestCase {
    #[serde(rename = "@file")]
    file: String,
    #[serde(rename = "@time")]
    time: String,
}

fn main() -> io::Result<()> {
    // Read all input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Group times by file
    let mut file_times: HashMap<String, f64> = HashMap::new();

    // Process all XML documents in the input
    let documents = split_xml_documents(&input);
    for xml_doc in documents {
        match from_str::<TestSuite>(&xml_doc) {
            Ok(testsuite) => {
                for element in testsuite.elements {
                    if let TestSuiteElement::TestCase(testcase) = element {
                        if let Ok(time) = testcase.time.parse::<f64>() {
                            *file_times.entry(testcase.file).or_insert(0.0) += time;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error parsing XML: {}", e);
            }
        }
    }

    // Output in ParallelTests::RSpec::RuntimeLogger format
    for (file, time) in file_times {
        println!("{}:{}", file, time);
    }

    Ok(())
}

fn split_xml_documents(input: &str) -> Vec<String> {
    let mut documents = Vec::new();
    let mut current_doc = String::new();
    let mut in_document = false;

    for line in input.lines() {
        if line.trim().starts_with("<?xml") {
            if !current_doc.trim().is_empty() {
                documents.push(current_doc.clone());
            }
            current_doc.clear();
            in_document = true;
        }

        if in_document {
            current_doc.push_str(line);
            current_doc.push('\n');

            if line.trim().ends_with("</testsuite>") {
                documents.push(current_doc.clone());
                current_doc.clear();
                in_document = false;
            }
        }
    }

    // Handle any remaining document
    if !current_doc.trim().is_empty() {
        documents.push(current_doc);
    }

    documents
}
