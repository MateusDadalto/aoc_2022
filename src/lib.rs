use std::collections::HashSet;

use point::Point;

mod helper;
mod point;

pub fn solve() {
    let lines = helper::get_file_lines_iter("inputs/input.txt");

    let mut input: Vec<(Point, Point)> = vec![];
    for line in lines {
        let line = line.unwrap();
        let parts = line.split_once(':').unwrap();
        let sensor_point = Point::parse(parts.0);
        let beacon_point = Point::parse(parts.1);

        input.push((sensor_point, beacon_point));
    }

    let mut distress_coord: HashSet<Point> = HashSet::new();

    for x in 0..4_000_001 {
        if x%100_000 == 0 {
            println!("{x}");
        }

        for y in 0..4_000_001 {
            
            let p: Point = (x,y).into();

            let is_in_radius = input.iter().any(|(sensor, beacon)| {
                let radius = sensor.distance(beacon.clone());

                p.is_in_radius(sensor.clone(), radius)
            });

            if !is_in_radius{
                println!("{p:?}");
                distress_coord.insert(p);
            }
        }
    }

    println!("Day 15 part 1: {distress_coord:?}");
}
