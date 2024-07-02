use data_structures::{Location, Region};
use geo::{coord, point, Contains, LineString, Polygon};

pub mod data_structures;

pub fn match_locations_to_regions(locations: &Vec<Location>, regions: &mut Vec<Region>) {
    for region in regions {
        for polygon in &region.polygons {
            let line_string: LineString<f64> = LineString::new(polygon.vertices.iter()
                .map(|vert| coord! { x: vert.longitude.val(), y: vert.latitude.val() })
                .collect()
            );
            let geo_polygon = Polygon::new(line_string, vec![]);
            region.matched_locations.extend(locations.iter()
                .filter(|location| geo_polygon.contains(&point!(x: location.coordinates.longitude.val(), y: location.coordinates.latitude.val())))
                .cloned()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use data_structures::Coordinates;
    use data_structures::DecimalLat;
    use data_structures::DecimalLon;
    use data_structures::Polygon;

    use super::*;
    #[test]
    fn test_region_on_south_pole() {
        let mut regions = vec![
            Region {
                name: "south_pole".into(),
                polygons: vec![
                    Polygon {
                        vertices: vec![
                            Coordinates {
                                latitude: DecimalLat::new(-83.2364265).unwrap(),
                                longitude: DecimalLon::new(-17.578125),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(-83.7155443).unwrap(),
                                longitude: DecimalLon::new(129.0234375),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(-72.8160737).unwrap(),
                                longitude: DecimalLon::new(66.09375),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(-83.2364265).unwrap(),
                                longitude: DecimalLon::new(-17.578125),
                            },
                        ],
                    },
                ],
                matched_locations: vec![],
            },
        ];
        let locations = vec![
            Location {
                name: "Location 1".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(-81.3215926).unwrap(),
                    longitude: DecimalLon::new(55.1074219),
                },
            },
            Location {
                name: "Location 2".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(-80.2979271).unwrap(),
                    longitude: DecimalLon::new(126.2109375),
                },
            },
            Location {
                name: "Location 3".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(-82.6313329).unwrap(),
                    longitude: DecimalLon::new(74.1796875),
                },
            },
            Location {
                name: "Location 4".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(-82.0700282).unwrap(),
                    longitude: DecimalLon::new(-125.5078125),
                },
            },
            Location {
                name: "Location 5".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(-74.4964131).unwrap(),
                    longitude: DecimalLon::new(66.09375),
                },
            },
        ];
        match_locations_to_regions(&locations, &mut regions);

        assert_eq!(regions[0].matched_locations, vec![locations[0].clone(), locations[2].clone(), locations[4].clone()]);
    }

    #[test]
    fn test_region_crossing_dateline() {
        let mut regions = vec![
            Region {
                name: "dateline_crossing".into(),
                polygons: vec![
                    Polygon {
                        vertices: vec![
                            Coordinates {
                                latitude: DecimalLat::new(1.9729085791280596).unwrap(),
                                longitude: DecimalLon::new(179.45182047167452),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(10.166318036315872).unwrap(),
                                longitude: DecimalLon::new(221.5167493379924),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(33.674251105897056).unwrap(),
                                longitude: DecimalLon::new(203.57476142603832),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(25.923204491260876).unwrap(),
                                longitude: DecimalLon::new(185.73842360857066),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(12.993861779456438).unwrap(),
                                longitude: DecimalLon::new(193.75697160392338),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(1.9729085791280596).unwrap(),
                                longitude: DecimalLon::new(179.45182047167452),
                            },
                        ],
                    },
                ],
                matched_locations: vec![],
            },
        ];

        let locations = vec![
            Location {
                name: "Location 1".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(15.9375).unwrap(),
                    longitude: DecimalLon::new(192.3397),
                },
            },
            Location {
                name: "Location 2".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(14.3281).unwrap(),
                    longitude: DecimalLon::new(209.0477),
                },
            },
            Location {
                name: "Location 3".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(30.0071).unwrap(),
                    longitude: DecimalLon::new(202.5000),
                },
            },
            Location {
                name: "Location 4".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(15.4924).unwrap(),
                    longitude: DecimalLon::new(189.1316),
                },
            },
            Location {
                name: "Location 5".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(14.9922).unwrap(),
                    longitude: DecimalLon::new(-179.4427),
                },
            },
            Location {
                name: "Location 6".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(3.6359).unwrap(),
                    longitude: DecimalLon::new(182.4744),
                },
            },
        ];
        match_locations_to_regions(&locations, &mut regions);

        assert_eq!(regions[0].matched_locations, vec![locations[0].clone(), locations[1].clone(), locations[2].clone(), locations[5].clone()]);
    }

    #[test]
    fn test_region_with_multiple_polygons() {
        let mut regions = vec![
            Region {
                name: "bory_tucholskie".into(),
                polygons: vec![
                    Polygon {
                        vertices: vec![
                            Coordinates {
                                latitude: DecimalLat::new(54.113263890982125).unwrap(),
                                longitude: DecimalLon::new(17.83255356035076),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(54.02579830288022).unwrap(),
                                longitude: DecimalLon::new(17.795452693995202),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(53.93873091663022).unwrap(),
                                longitude: DecimalLon::new(17.74990772923971),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(53.84985082823309).unwrap(),
                                longitude: DecimalLon::new(18.0726316744757),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(53.78381152974012).unwrap(),
                                longitude: DecimalLon::new(18.30621001515354),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(53.92340158940269).unwrap(),
                                longitude: DecimalLon::new(18.26005444680112),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(54.028749419977316).unwrap(),
                                longitude: DecimalLon::new(18.04030450061913),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(54.094894457634155).unwrap(),
                                longitude: DecimalLon::new(17.998325231424843),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(54.113263890982125).unwrap(),
                                longitude: DecimalLon::new(17.83255356035076),
                            },
                        ],
                    },
                    Polygon {
                        vertices: vec![
                            Coordinates {
                                latitude: DecimalLat::new(53.885876286371314).unwrap(),
                                longitude: DecimalLon::new(17.446081519786787),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(53.73833867219909).unwrap(),
                                longitude: DecimalLon::new(17.466045794468016),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(53.75581313064666).unwrap(),
                                longitude: DecimalLon::new(17.694912495793687),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(53.82123216564551).unwrap(),
                                longitude: DecimalLon::new(17.73700434073797),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(53.89511892541117).unwrap(),
                                longitude: DecimalLon::new(17.56648787290615),
                            },
                            Coordinates {
                                latitude: DecimalLat::new(53.885876286371314).unwrap(),
                                longitude: DecimalLon::new(17.446081519786787),
                            },
                        ],
                    },
                ],
                matched_locations: vec![],
            },
        ];
        let locations = vec![
            Location { //ok
                name: "Location 1".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(53.931943242940264).unwrap(),
                    longitude: DecimalLon::new(18.04777597110123),
                },
            },
            Location { //ok
                name: "Location 2".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(53.7702198592585).unwrap(),
                    longitude: DecimalLon::new(17.55344865541619),
                },
            },
            Location { //ok
                name: "Location 3".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(54.08518088704679).unwrap(),
                    longitude: DecimalLon::new(17.84555741202533),
                },
            },
            Location {
                name: "Location 4".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(53.903982576424795).unwrap(),
                    longitude: DecimalLon::new(17.590412921065422),
                },
            }
        ];
        match_locations_to_regions(&locations, &mut regions);
        
        let expected = vec![locations[0].clone(), locations[1].clone(), locations[2].clone()];
        assert!(regions[0].matched_locations.iter().all(|loc| expected.contains(loc)));
        assert!(regions[0].matched_locations.len() == expected.len());
    }

    #[test]
    fn test_point_on_overlapping_regions() {
        let mut regions = vec![
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
                matched_locations: vec![],
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
                matched_locations: vec![],
            }
        ];
        let locations = vec![
            Location { //overlapping
                name: "Location 1".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(49.24340413142335).unwrap(),
                    longitude: DecimalLon::new(19.726640710592307),
                },
            },
            Location { //region1
                name: "Location 2".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(49.232581877359536).unwrap(),
                    longitude: DecimalLon::new(19.36788978252892),
                },
            },
            Location { //region2
                name: "Location 3".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(49.24476375835607).unwrap(),
                    longitude: DecimalLon::new(20.219267732042425),
                },
            },
            Location { //out
                name: "Location 4".into(),
                coordinates: Coordinates {
                    latitude: DecimalLat::new(49.399912837692284).unwrap(),
                    longitude: DecimalLon::new(19.561924809724104),
                },
            }
        ];
        match_locations_to_regions(&locations, &mut regions);

        let expected_region1 = vec![locations[0].clone(), locations[1].clone()];
        let expected_region2 = vec![locations[0].clone(), locations[2].clone()];

        assert!(regions[0].matched_locations.iter().all(|loc| expected_region1.contains(loc)));
        assert!(regions[0].matched_locations.len() == expected_region1.len());
        assert!(regions[1].matched_locations.iter().all(|loc| expected_region2.contains(loc)));
        assert!(regions[1].matched_locations.len() == expected_region2.len());
    }
    
    
}