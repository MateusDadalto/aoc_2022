use point::Point;

mod helper;
mod point;

pub fn solve() {
    let lines = helper::get_file_lines_iter("inputs/input.txt");

    let mut input: Vec<(Point, Point)> = vec![];
    let mut answer = Point { x: 0, y: 0 };
    for line in lines {
        let line = line.unwrap();
        let parts = line.split_once(':').unwrap();
        let sensor_point = Point::parse(parts.0);
        let beacon_point = Point::parse(parts.1);

        input.push((sensor_point, beacon_point));
    }

    // I know how inneficient this is
    'outer: for (sensor, beacon) in input.iter() {
        let outside_points = sensor.outside_points(sensor.distance(beacon.clone()));

        for point in outside_points {
            if input
                .iter()
                .any(|(s, b)| point.is_in_radius(s.clone(), s.distance(b.clone())))
            {
                continue;
            }

            println!("{point:?}");
            answer = point;
            break 'outer;
        }
    }

    println!("Day 15 part 2: {}", answer.x * 4_000_000 + answer.y);
}
