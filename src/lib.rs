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

    let max_x_duo = input
        .iter()
        .max_by_key(|(sensor, beacon)| {
            let max = sensor.x.max(beacon.x);
            max
        })
        .unwrap();
    let min_x_duo = input
        .iter()
        .min_by_key(|(sensor, beacon)| {
            let min = sensor.x.min(beacon.x);
            min
        })
        .unwrap();

    let radius_max_x = max_x_duo.0.distance(max_x_duo.1) as isize;
    let radius_min_x = min_x_duo.0.distance(min_x_duo.1) as isize;
    let start_range = min_x_duo.0.x.min(min_x_duo.1.x) - radius_min_x;
    let end_range = max_x_duo.0.x.max(max_x_duo.1.x) + radius_max_x;

    let columns = Point::range_x(
        start_range,
        end_range,
        2_000_000,
    );
    let mut empty_points: HashSet<Point> = HashSet::with_capacity((end_range - start_range).unsigned_abs());

    for (sensor, beacon) in input {
        let radius = sensor.distance(beacon);
        empty_points.extend(columns.iter().filter(|p| p.is_in_radius(sensor, radius)));

        empty_points.remove(&beacon);
    }

    println!("Day 15 part 1: {}", empty_points.len());
}
