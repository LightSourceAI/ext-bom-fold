use bom_fold::{
    transform, ChildIdentificationPolicy, FlatData, ItemSyncFormat, ItemSyncFormatRules,
    OutputRules, Rules, ValueType,
};
use clap::Parser;
use std::{fs::File, io::Read, path::Path};

/// Parses a level ordered BOM flat file and writes ItemSync compatible output.
///
/// Currently only CSV input is supported.
#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "Idan <idan@lightsource.ai>")]
struct Opts {
    /// Input file path
    #[clap(long)]
    input: String,

    /// Output directory where files will be written.
    #[clap(long)]
    output: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    let input_path = Path::new(&opts.input);
    let file_extension = input_path.extension();

    let mut file = File::open(input_path).expect("Couldn't open `input` file");
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents).expect("Failed to read input file");

    let fixed_rules = Rules {
        type_mapping: Some([("Quantity".to_string(), ValueType::Number)].into_iter().collect()),
        child_identification_policy: ChildIdentificationPolicy::OrderedLevelKey(
            "level".to_string(),
        ),
        output_rules: OutputRules::ItemSync(ItemSyncFormatRules {
            id_key: "Part Number".to_string(),
            name_key: Some("Part Name".to_string()),
            quantity_key: Some("Quantity".to_string()),
        }),
    };

    let flat_data = match file_extension.and_then(|e| e.to_str()) {
        Some("csv") => FlatData::from_csv(&file_contents, &fixed_rules).unwrap(),
        Some("xlsx") => unimplemented!("Excel is not yet supported"),
        _ => {
            panic!("Unrecognized file type. Please ensure your file has a .csv or .xlsx extension")
        }
    };
    let folded_data = transform(&flat_data, &fixed_rules).unwrap();
    let formatted_data = match &fixed_rules.output_rules {
        OutputRules::ItemSync(item_sync_rules) => {
            ItemSyncFormat::format_item_sync(&folded_data, item_sync_rules).unwrap()
        }
    };

    if let Some(output_dir) = opts.output {
        write_output(&formatted_data, &output_dir);
    } else {
        println!("{formatted_data:?}")
    }
}

/// Writes the formatted_data to CSV.
fn write_output(formatted_data: &ItemSyncFormat, output_dir: &str) {
    let output_dir = Path::new(output_dir);
    std::fs::create_dir_all(output_dir).unwrap();

    let mut boms_writer = csv::Writer::from_path(output_dir.join("boms.csv")).unwrap();
    for bom in formatted_data.boms.iter() {
        boms_writer.serialize(bom).unwrap()
    }
    boms_writer.flush().unwrap();

    let mut bom_entries_writer =
        csv::Writer::from_path(output_dir.join("bom_entries.csv")).unwrap();
    for entry in formatted_data.bom_entries.iter() {
        bom_entries_writer.serialize(entry).unwrap();
    }
    bom_entries_writer.flush().unwrap();
}
