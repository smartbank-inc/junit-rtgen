use quick_xml::Reader;
use quick_xml::de::from_str;
use quick_xml::events::Event;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug, Deserialize)]
pub struct TestSuite {
    #[serde(rename = "$value")]
    pub elements: Vec<TestSuiteElement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestSuiteElement {
    Properties(Properties),
    TestCase(TestCase),
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    // We don't need the contents for this use case
}

#[derive(Debug, Deserialize)]
pub struct TestCase {
    #[serde(rename = "@file")]
    pub file: String,
    #[serde(rename = "@time", deserialize_with = "deserialize_time")]
    pub time: f64,
}

fn deserialize_time<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

pub fn process_junit_xml(input: &str) -> HashMap<String, f64> {
    let mut file_times: HashMap<String, f64> = HashMap::new();

    // Process all XML documents in the input
    let documents = split_xml_documents(input);
    for xml_doc in documents {
        match from_str::<TestSuite>(&xml_doc) {
            Ok(testsuite) => {
                for element in testsuite.elements {
                    if let TestSuiteElement::TestCase(testcase) = element {
                        *file_times.entry(testcase.file).or_insert(0.0) += testcase.time;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error parsing XML: {}", e);
            }
        }
    }

    file_times
}

pub fn split_xml_documents(input: &str) -> Vec<String> {
    let mut documents = Vec::new();
    let mut current_doc = String::with_capacity(1024); // Pre-allocate reasonable capacity
    let mut in_document = false;

    for line in input.lines() {
        if line.trim().starts_with("<?xml") {
            if !current_doc.trim().is_empty() {
                documents.push(std::mem::take(&mut current_doc));
                current_doc.reserve(1024); // Reserve capacity for next document
            }
            in_document = true;
        }

        if in_document {
            current_doc.push_str(line);
            current_doc.push('\n');

            if line.trim().ends_with("</testsuite>") {
                documents.push(std::mem::take(&mut current_doc));
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

/// Streaming XML parser that processes input without loading everything into memory
pub fn process_junit_xml_streaming<R: BufRead>(reader: R) -> HashMap<String, f64> {
    let mut file_times: HashMap<String, f64> = HashMap::new();
    let mut reader = Reader::from_reader(reader);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"testcase" => {
                let mut file = None;
                let mut time = None;

                for attr in e.attributes().flatten() {
                    match attr.key.as_ref() {
                        b"file" => file = Some(String::from_utf8_lossy(&attr.value).into_owned()),
                        b"time" => time = String::from_utf8_lossy(&attr.value).parse::<f64>().ok(),
                        _ => {}
                    }
                }

                if let (Some(file), Some(time)) = (file, time) {
                    *file_times.entry(file).or_insert(0.0) += time;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Error parsing XML: {}", e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    file_times
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_single_xml_document() {
        let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="rspec" tests="1">
<testcase classname="spec.foo_spec" name="Foo spec" file="./spec/foo.rb" time="0.11111"></testcase>
</testsuite>"#;

        let documents = split_xml_documents(input);
        assert_eq!(documents.len(), 1);
        assert!(documents[0].contains("<?xml"));
        assert!(documents[0].contains("</testsuite>"));
    }

    #[test]
    fn test_split_multiple_xml_documents() {
        let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="rspec" tests="1">
<testcase classname="spec.foo_spec" name="Foo spec" file="./spec/foo.rb" time="0.11111"></testcase>
</testsuite>
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="rspec" tests="1">
<testcase classname="spec.bar_spec" name="Bar spec" file="./spec/bar.rb" time="0.22222"></testcase>
</testsuite>"#;

        let documents = split_xml_documents(input);
        assert_eq!(documents.len(), 2);
        assert!(documents[0].contains("foo.rb"));
        assert!(documents[1].contains("bar.rb"));
    }

    #[test]
    fn test_split_xml_with_whitespace() {
        let input = r#"   <?xml version="1.0" encoding="UTF-8"?>
<testsuite name="rspec" tests="1">
<testcase classname="spec.foo_spec" name="Foo spec" file="./spec/foo.rb" time="0.11111"></testcase>
</testsuite>   "#;

        let documents = split_xml_documents(input);
        assert_eq!(documents.len(), 1);
    }

    #[test]
    fn test_split_empty_input() {
        let input = "";
        let documents = split_xml_documents(input);
        assert_eq!(documents.len(), 0);
    }

    #[test]
    fn test_split_no_xml_declaration() {
        let input = r#"<testsuite name="rspec" tests="1">
<testcase classname="spec.foo_spec" name="Foo spec" file="./spec/foo.rb" time="0.11111"></testcase>
</testsuite>"#;

        let documents = split_xml_documents(input);
        assert_eq!(documents.len(), 0);
    }

    #[test]
    fn test_split_incomplete_xml() {
        let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="rspec" tests="1">
<testcase classname="spec.foo_spec" name="Foo spec" file="./spec/foo.rb" time="0.11111"></testcase>"#;

        let documents = split_xml_documents(input);
        assert_eq!(documents.len(), 1);
        assert!(documents[0].contains("<?xml"));
        assert!(!documents[0].contains("</testsuite>"));
    }

    #[test]
    fn test_process_junit_xml() {
        let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="rspec" tests="3">
<testcase classname="spec.foo_spec" name="Foo spec" file="./spec/foo.rb" time="0.11111"></testcase>
<testcase classname="spec.foo2_spec" name="Foo2 spec" file="./spec/foo.rb" time="0.12345"></testcase>
<testcase classname="spec.bar_spec" name="Bar spec" file="./spec/bar.rb" time="0.10101"></testcase>
</testsuite>"#;

        let result = process_junit_xml(input);
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("./spec/foo.rb"), Some(&0.23456));
        assert_eq!(result.get("./spec/bar.rb"), Some(&0.10101));
    }

    #[test]
    fn test_process_junit_xml_streaming() {
        let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="rspec" tests="3">
<testcase classname="spec.foo_spec" name="Foo spec" file="./spec/foo.rb" time="0.11111"></testcase>
<testcase classname="spec.foo2_spec" name="Foo2 spec" file="./spec/foo.rb" time="0.12345"></testcase>
<testcase classname="spec.bar_spec" name="Bar spec" file="./spec/bar.rb" time="0.10101"></testcase>
</testsuite>"#;

        let result = process_junit_xml_streaming(input.as_bytes());
        assert_eq!(result.len(), 2);

        let foo_time = result.get("./spec/foo.rb").unwrap();
        assert!((foo_time - 0.23456).abs() < 0.00001);

        let bar_time = result.get("./spec/bar.rb").unwrap();
        assert!((bar_time - 0.10101).abs() < 0.00001);
    }
}
