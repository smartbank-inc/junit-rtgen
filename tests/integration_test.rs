use junit_rtgen::process_junit_xml;

#[test]
fn test_end_to_end_processing() {
    let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="rspec" tests="3">
<properties>
<property name="seed" value="1234"/>
</properties>
<testcase classname="spec.foo_spec" name="Foo spec" file="./spec/foo.rb" time="0.11111"></testcase>
<testcase classname="spec.foo2_spec" name="Foo2 spec" file="./spec/foo.rb" time="0.12345"></testcase>
<testcase classname="spec.bar_spec" name="Bar spec" file="./spec/bar.rb" time="0.10101"></testcase>
</testsuite>
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="rspec" tests="2">
<testcase classname="spec.foo_spec" name="Another Foo spec" file="./spec/foo.rb" time="0.22222"></testcase>
<testcase classname="spec.baz_spec" name="Baz spec" file="./spec/baz.rb" time="0.33333"></testcase>
</testsuite>"#;

    let result = process_junit_xml(input);

    assert_eq!(result.len(), 3);

    // Use approximate comparison for floating point values
    let foo_time = result.get("./spec/foo.rb").unwrap();
    assert!((foo_time - 0.45678).abs() < 0.00001);

    let bar_time = result.get("./spec/bar.rb").unwrap();
    assert!((bar_time - 0.10101).abs() < 0.00001);

    let baz_time = result.get("./spec/baz.rb").unwrap();
    assert!((baz_time - 0.33333).abs() < 0.00001);
}
