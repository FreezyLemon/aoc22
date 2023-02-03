use std::{str::FromStr, convert::Infallible, collections::HashSet};

use get_input::*;

fn main() {
    let content = get_input().unwrap();
    let y_of_concern = 2_000_000;

    let reports: Vec<SensorReport> = content
        .split('\n')
        .map(SensorReport::from_str)
        .map(|e| e.unwrap())
        .collect();

        let points_with_dist: Vec<(Point, isize)> = reports
        .iter()
        .map(|r| {
            let dist = r.dist();
            (r.sensor.clone(), dist)
        })
        .filter(|(s, d)| (y_of_concern - s.y).abs() <= *d)
        .collect();

    let mut blocked = HashSet::new();
    for (p, dist) in points_with_dist {
        let dy = (p.y - y_of_concern).abs();
        let dx = dist - dy;
        for x in (p.x - dx)..=(p.x + dx) {
            blocked.insert(x);
        }
    }

    for r in reports {
        if r.beacon.y == y_of_concern {
            blocked.remove(&r.beacon.x);
        }
    }
    let result = blocked.len();
    println!("result: {result}");
}

struct SensorReport {
    sensor: Point,
    beacon: Point,
}

impl SensorReport {
    fn dist(&self) -> isize {
        (self.beacon.x - self.sensor.x).abs() +
        (self.beacon.y - self.sensor.y).abs()
    }
}

impl FromStr for SensorReport {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(':').unwrap();

        assert!(left.starts_with("Sensor at "));
        let (_, coords) = left.split_at("Sensor at ".len());
        let (x, y) = coords.split_once(',').unwrap();
        let sensor = Point {
            x: x[2..].parse().unwrap(),
            y: y[3..].parse().unwrap(),
        };

        assert!(right.starts_with(" closest beacon is at "));
        let (_, coords) = right.split_at(" closest beacon is at ".len());
        let (x, y) = coords.split_once(',').unwrap();
        let beacon = Point {
            x: x[2..].parse().unwrap(),
            y: y[3..].parse().unwrap(),
        };

        Ok(SensorReport { sensor, beacon })
    }
}

#[derive(Clone)]
struct Point {
    x: isize,
    y: isize,
}
