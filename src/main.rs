use csv::{ReaderBuilder, WriterBuilder};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};

fn process_cell(cell: &str) -> String {
    let cell = cell.trim();

    if cell.to_lowercase() == "\\n" || cell.to_lowercase() == "null" {
        return String::new();
    }

    if cell.ends_with(".0") && cell.replace(".", "").chars().all(|c| c.is_digit(10)) {
        return cell.trim_end_matches('0').trim_end_matches('.').to_string();
    }

    // Double the quotes for COPY command
    return cell.replace("\"", "\"\"");
}

fn csv_cut(input_path: &str, num_partitions: usize) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(input_path)?;
    let output_path = format!("{}.tmp", input_path);
    let output_file = File::create(&output_path)?;

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(BufReader::new(input_file));

    let mut writer = WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Always)
        .from_writer(BufWriter::new(output_file));

    for result in reader.records() {
        let record = result?;

        let new_row: Vec<String> = if num_partitions > 0 {
            let value = record
                .iter()
                .take(record.len() - num_partitions)
                .map(|cell| process_cell(cell))
                .collect();

            value
        } else {
            record.iter().map(|cell| process_cell(cell)).collect()
        };
        writer.write_record(&new_row)?;
    }

    writer.flush()?;

    // Replace the original file with the processed file
    std::fs::rename(&output_path, input_path)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_csv> <num_partitions>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let num_partitions: usize = args[2].parse()?;

    csv_cut(input_path, num_partitions)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_process_cell() {
        assert_eq!(process_cell("\\n"), "");
        assert_eq!(process_cell("null"), "");
        assert_eq!(process_cell("123.0"), "123");
        assert_eq!(process_cell("123.45"), "123.45");
        assert_eq!(process_cell("\"Quote\""), "\"\"Quote\"\"");
        assert_eq!(process_cell(" Trimmed "), "Trimmed");
    }

    #[test]
    fn test_csv_processing() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::new()?;
        let input = "Name,Age,City\nJohn,30.0,New York\nJane,25,\"San Francisco, CA\"\n";
        write!(temp_file, "{}", input)?;

        csv_cut(temp_file.path().to_str().unwrap(), 0)?;

        let processed_csv = std::fs::read_to_string(temp_file.path())?;
        let expected = "\"Name\",\"Age\",\"City\"\n\"John\",\"30\",\"New York\"\n\"Jane\",\"25\",\"San Francisco, CA\"\n";

        assert_eq!(processed_csv, expected);

        Ok(())
    }

    #[test]
    fn test_csv_processing_with_partitions() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::new()?;
        let input =
            "Name,Age,City,Partition\nJohn,30.0,New York,1\nJane,25,\"San Francisco, CA\",2\n";
        write!(temp_file, "{}", input)?;

        csv_cut(temp_file.path().to_str().unwrap(), 1)?;

        let processed_csv = std::fs::read_to_string(temp_file.path())?;
        let expected = "\"Name\",\"Age\",\"City\"\n\"John\",\"30\",\"New York\"\n\"Jane\",\"25\",\"San Francisco, CA\"\n";

        assert_eq!(processed_csv, expected);

        Ok(())
    }
}
