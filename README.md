# junit-rtgen

A CLI tool that converts JUnit XML format files to ParallelTests::RSpec::RuntimeLogger format.

Inspiration from the following article:
- https://qiita.com/tomoasleep/items/0ee5cae02739c8695c59

## TODO

- [ ] Parse JUnit XML format files from stdin
  - [ ] Extract "file" and "time" attributes from the XML
- [ ] Group test results by file and sum the execution times
- [ ] Output results in ParallelTests::RSpec::RuntimeLogger format
  - [ ] Format: `file:sum_of_time` (one per line)
- [ ] Support piping multiple XML files at once
- [ ] Support piping output to file

## Usage (Planned)

```bash
# Single file
cat junit-report.xml | junit-rtgen > runtime.log

# Multiple files
cat *.xml | junit-rtgen > runtime.log

# Direct output
junit-rtgen < junit-report.xml
```
