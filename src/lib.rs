use quick_xml::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;

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
    #[serde(rename = "@time")]
    pub time: String,
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

    file_times
}

pub fn split_xml_documents(input: &str) -> Vec<String> {
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
}
