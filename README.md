# csvcut

csvcut is a command-line tool for cutting n columns from CSV files.

Tool for adapting CSV files produced by Apache Hive by using `INSERT OVERWRITE DIRECTORY` into a CSV file compatible with PostgreSQL's `COPY` command.

It's making the replace in place.

`main.rs` contains the test cases as an example of the usage as well.

## Example

```
csvcut sample.csv 1
```