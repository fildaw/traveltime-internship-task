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
    let locations_file = File::open(&cli.locations)
        .expect(&format!("Location file not found! (looked in {})", cli.locations.display()));
    let locations: Vec<Location> = serde_json::from_reader(BufReader::new(locations_file))
        .expect(&format!("An error occurred while reading the locations file! ({})", cli.locations.display()));

    let regions_file = File::open(&cli.regions)
        .expect(&format!("Regions file not found! (looked in {})", cli.regions.display()));
    let mut regions: Vec<Region> = serde_json::from_reader(BufReader::new(regions_file))
        .expect(&format!("An error occurred while reading the regions file! ({})", cli.regions.display()));

    match_locations_to_regions(&locations, &mut regions);

    let file = File::create(&cli.output)
        .expect("Cannot create output file!");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &regions)
        .expect("Cannot serialize output!");
    writer.flush().unwrap();
}
