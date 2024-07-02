# Internship task

The task is to match locations from a JSON file to regions defined by another JSON file. The output should be saved in a new JSON file.

## Locations file structure
```json
[
  {
    "name": "location1",
    "coordinates": [
      25.21051562929364,
      54.64057937965808
    ]
  },
  ... other locations
]
```

## Regions file structure
```json
[
  {
    "name": "region1",
    "coordinates": [
      [
        [
          25.13573603154873,
          54.67922829209249
        ],
        [
          25.13573603154873,
          54.77109854334182
        ],
        [
          25.286660938416787,
          54.5942400514071
        ],
        [
          25.13573603154873,
          54.67922829209249
        ]
      ],
      ... other polygons in the region
    ]
  },
  ... other regions
]
```
The polygon is defined by a list of points. The first and the last point should be the same to close the polygon.
## Output file structure
```json
[
  {
    "region": "region1",
    "matched_locations": [
      "location1",
      "location2"
    ]
  },
  ... other regions
]
```

# Implementation
The implementation is written in Rust using the following libraries (crates): 
- [`serde_json`](https://crates.io/crates/serde_json) - for deserializing and serializing JSON, 
- [`geo`](https://crates.io/crates/geo) - for checking if point is inside a polygon
- [`clap`](https://crates.io/crates/clap) - for parsing command line arguments.

# Compiling
To compile the project Rust toolchain and Cargo (the Rust package manager) are required. The simplest way to install them is using [rustup](https://rustup.rs/). \
After installing Rust and Cargo, the project can be built using the `cargo build` command in the root directory of the project. The resulting binary will be in the `target/debug` directory. \
To run unit tests use `cargo test` command.

# Usage
```bash
./traveltime_internship_task --locations <FILE> --regions <FILE> --output <FILE>
```
All of the parameters are required. The program will read locations and regions from the specified files, match locations to regions and save the output to the specified file.