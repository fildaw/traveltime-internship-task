use std::error::Error;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Coordinates {
    pub longitude: DecimalLon,
    pub latitude: DecimalLat,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Location {
    pub name: String,
    pub coordinates: Coordinates,
}

// When we serialize location, we only want to serialize the name
impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}

#[derive(Debug)]
pub struct BadLatitudeError(String);

impl Error for BadLatitudeError {}

impl std::fmt::Display for BadLatitudeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Bad latitude: {} (it should be of range: -90 to 90)", self.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecimalLat(f64);
impl DecimalLat {
    pub fn new(lat: f64) -> Result<DecimalLat, BadLatitudeError> {
        if lat < -90.0 || lat > 90.0 {
            return Err(BadLatitudeError(lat.to_string()));
        }
        Ok(DecimalLat(lat))
    }
    pub fn val(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecimalLon(f64);
impl DecimalLon {
    pub fn new(lon: f64) -> DecimalLon {
        DecimalLon(lon)
    }
    pub fn val(&self) -> f64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for DecimalLat {
    fn deserialize<D>(deserializer: D) -> Result<DecimalLat, D::Error>
    where
        D: Deserializer<'de>,
    {
        DecimalLat::new(f64::deserialize(deserializer)?).map_err(|e| serde::de::Error::custom(e))
    }
}

impl<'de> Deserialize<'de> for DecimalLon {
    fn deserialize<D>(deserializer: D) -> Result<DecimalLon, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DecimalLon::new(f64::deserialize(deserializer)?))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Region {
    #[serde(rename(serialize = "region"))]
    pub name: String,
    #[serde(rename="coordinates", skip_serializing)]
    pub polygons: Vec<Polygon>,
    #[serde(skip_deserializing)]
    pub matched_locations: Vec<Location>,
}

#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<Coordinates>,
}

impl<'de> Deserialize<'de> for Polygon {
    fn deserialize<D>(deserializer: D) -> Result<Polygon, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vertices: Vec<Coordinates> = Vec::deserialize(deserializer)?;
        if vertices.len() < 4 {
            return Err(serde::de::Error::custom("Polygon must have at least 3 vertices"));
        }
        if vertices.first().unwrap() != vertices.last().unwrap() {
            return Err(serde::de::Error::custom("Polygon must be closed"));
        }
        Ok(Polygon { vertices })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_location() {
        let json = r#"{
            "name": "location1",
            "coordinates": [
                54.64057937965808,
                25.21051562929364
            ]
        }"#;
        let location: Location = serde_json::from_str(json).unwrap();
        assert_eq!(location.name, "location1");
        assert_eq!(location.coordinates.longitude.0, 54.64057937965808);
        assert_eq!(location.coordinates.latitude.0, 25.21051562929364);
    }

    #[test]
    fn test_deserialize_region() {
        let json = r#"{
            "name": "region3",
            "coordinates": [
                [
                    [
                    21.099044587495996,
                    55.697364539462455
                    ],
                    [
                    21.13167699979246,
                    55.63985211052827
                    ],
                    [
                    21.135756051329366,
                    55.80067402588713
                    ],
                    [
                    21.099044587495996,
                    55.697364539462455
                    ]
                ],
                [
                    [
                    21.100737600741354,
                    55.64456937538671
                    ],
                    [
                    21.08556244179519,
                    55.48839930587644
                    ],
                    [
                    20.97630129738701,
                    55.30743065067017
                    ],
                    [
                    21.049142060326403,
                    55.31952101226224
                    ],
                    [
                    21.115912759686722,
                    55.495276974748975
                    ],
                    [
                    21.131087918632034,
                    55.630865024154645
                    ],
                    [
                    21.100737600741354,
                    55.64456937538671
                    ]
                ]
            ]
          }"#;
        let region: Region = serde_json::from_str(json).unwrap();
        assert_eq!(region.name, "region3");
        assert_eq!(region.polygons.len(), 2);
        assert_eq!(region.polygons[0].vertices.len(), 4);
        assert_eq!(region.polygons[1].vertices.len(), 7);
        assert_eq!(region.polygons[1].vertices[1].longitude.val(), 21.08556244179519);
    }

    #[test]
    fn test_serialize_region() {
        let regions = vec![
            Region {
                name: "tatry_slovakia".into(),
                polygons: vec![
                    Polygon {
                        vertices: vec![
                            Coordinates {
                                latitude: DecimalLat::new(49.37351801413155).unwrap(),
                                longitude: DecimalLon::new(19.67847490452553),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(49.23803996442288).unwrap(),
                                longitude: DecimalLon::new(19.304812334103275),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(49.097478621327554).unwrap(),
                                longitude: DecimalLon::new(19.328347447593416),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(49.16226149904949).unwrap(),
                                longitude: DecimalLon::new(19.547539671978996),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(49.125183242944075).unwrap(),
                                longitude: DecimalLon::new(19.795878887634984),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(49.37351801413155).unwrap(),
                                longitude: DecimalLon::new(19.67847490452553),
                            },
                        ],
                    },
                ],
                matched_locations: vec![
                    Location { 
                        name: "Location 1".into(),
                        coordinates: Coordinates {
                            latitude: DecimalLat::new(49.24340413142335).unwrap(),
                            longitude: DecimalLon::new(19.726640710592307),
                        },
                    },
                    Location {
                        name: "Location 2".into(),
                        coordinates: Coordinates {
                            latitude: DecimalLat::new(49.232581877359536).unwrap(),
                            longitude: DecimalLon::new(19.36788978252892),
                        },
                    },
                ],
            },
            Region {
                name: "tatry_poland_slovakia".into(),
                polygons: vec![
                    Polygon {
                        vertices: vec![
                            Coordinates {
                                latitude: DecimalLat::new(49.31803102546846).unwrap(),
                                longitude: DecimalLon::new(19.855860471519293),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(49.250310004550215).unwrap(),
                                longitude: DecimalLon::new(19.70351226362419),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(49.17191660346475).unwrap(),
                                longitude: DecimalLon::new(19.757887614103993),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(49.15403200004263).unwrap(),
                                longitude: DecimalLon::new(20.25482071219585),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(49.26147108985464).unwrap(),
                                longitude: DecimalLon::new(20.332680386606796),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(49.31803102546846).unwrap(),
                                longitude: DecimalLon::new(19.855860471519293),
                            },
                        ],
                    }
                ],
                matched_locations: vec![
                    Location {
                        name: "Location 1".into(),
                        coordinates: Coordinates {
                            latitude: DecimalLat::new(49.24340413142335).unwrap(),
                            longitude: DecimalLon::new(19.726640710592307),
                        },
                    },
                    Location {
                        name: "Location 3".into(),
                        coordinates: Coordinates {
                            latitude: DecimalLat::new(49.24476375835607).unwrap(),
                            longitude: DecimalLon::new(20.219267732042425),
                        },
                    },
                ],
            },
            Region {
                name: "empty".to_owned(),
                polygons: vec![],
                matched_locations: vec![]
            }
        ];
        let json_str = serde_json::to_string(&regions).unwrap();
        assert_eq!(r#"[{"region":"tatry_slovakia","matched_locations":["Location 1","Location 2"]},{"region":"tatry_poland_slovakia","matched_locations":["Location 1","Location 3"]},{"region":"empty","matched_locations":[]}]"#.to_owned(), json_str);
    }

    #[test]
    fn test_bad_latitude() {
        let json = r#"{
            "name": "location1",
            "coordinates": [
                25.21051562929364,
                94.64057937965808
            ]
        }"#;
        let result: Result<Location, _> = serde_json::from_str(json);
        let error = result.unwrap_err();
        assert_eq!(error.classify(), serde_json::error::Category::Data);
    }

    #[test]
    fn test_unclosed_polygon() {
        let json = r#"{
            "name": "region3",
            "coordinates": [
                [
                    [
                    21.099044587495996,
                    55.697364539462455
                    ],
                    [
                    21.13167699979246,
                    55.63985211052827
                    ],
                    [
                    21.135756051329366,
                    55.80067402588713
                    ]
                ]
            ]
            }"#;
        let result: Result<Region, _> = serde_json::from_str(json);
        let error = result.unwrap_err();
        assert_eq!(error.classify(), serde_json::error::Category::Data);
    }

    #[test]
    fn test_too_little_vertices_to_form_polygon() {
        let json = r#"{
            "name": "region3",
            "coordinates": [
                [
                    [
                    21.099044587495996,
                    55.697364539462455
                    ],
                    [
                    21.13167699979246,
                    55.63985211052827
                    ],
                    [
                    21.099044587495996,
                    55.697364539462455
                    ]
                ]
            ]
            }"#;
        let result: Result<Region, _> = serde_json::from_str(json);
        let error = result.unwrap_err();
        assert_eq!(error.classify(), serde_json::error::Category::Data);
    }

}