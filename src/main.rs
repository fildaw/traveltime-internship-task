use std::{fs::File, io::{BufReader, BufWriter, Write}, path::PathBuf};

use clap::Parser;
use traveltime_internship_task::{data_structures::{Location, Region}, match_locations_to_regions};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// locations.json path
    #[arg(short, long, value_name="FILE")]
    locations: PathBuf,

    /// regions.json path
    #[arg(short, long, value_name="FILE")]
    regions: PathBuf,

    /// output file path
    #[arg(short, long, value_name="FILE")]
    output: PathBuf
}

fn main() {
    let cli = Cli::parse();
    let locations_result: Result<Vec<Location>, std::string::String> = File::open(&cli.locations)
        .map_err(|e| format!("Location file error! (looked in {}), os: {}", cli.locations.display(), e))
        .and_then(|locations_file| 
            serde_json::from_reader(BufReader::new(locations_file))
                .map_err(|e| format!("An error occurred while reading the locations file! {}", e))
        )
        .inspect_err(|e| println!("{}", e));
    let regions_result: Result<Vec<Region>, std::string::String> = File::open(&cli.regions)
        .map_err(|e| format!("Region file error! (looked in {}), os: {}", cli.regions.display(), e))
        .and_then(|regions_file| 
            serde_json::from_reader(BufReader::new(regions_file))
                .map_err(|e| format!("An error occurred while reading the regions file! {}", e))
        )
        .inspect_err(|e| println!("{}", e));

    let (locations, regions) = match (locations_result, regions_result) {
        (Ok(locs), Ok(regs)) => (locs, regs),
        _ => {
            return;
        }
    };

    let matched_results = match_locations_to_regions(&locations, &regions);

    let file = File::create(&cli.output)
        .expect("Cannot create output file!");
    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, &matched_results).unwrap();
    writer.flush().unwrap();
}
