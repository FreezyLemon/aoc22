mod input;

fn main() {
    let content = crate::input::get_input().unwrap();

    let cubes: Vec<Cube> = content
        .split('\n')
        .map(|line| {
            let coords: Vec<i8> = line
                .split(',')
                .map(|n| n.parse().expect("cannot parse to u8"))
                .collect();
            
            if coords.len() != 3 {
                panic!("invalid input");
            }

            Cube {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();

    let mut result = 0;
    for i in 0..cubes.len() {
        let curr_cube = &cubes[i];
        result += 6;

        for k in i + 1..cubes.len() {
            if curr_cube.adjacent_to(&cubes[k]) {
                result -= 2;
            }
        }
    }

    println!("result: {result}");
}

struct Cube {
    x: i8,
    y: i8,
    z: i8,
}

impl Cube {
    fn adjacent_to(&self, other: &Cube) -> bool {
        self.x == other.x && self.y == other.y && (self.z - other.z).abs() == 1 ||
        self.x == other.x && (self.y - other.y).abs() == 1 && self.z == other.z ||
        (self.x - other.x).abs() == 1 && self.y == other.y && self.z == other.z
    }
}
