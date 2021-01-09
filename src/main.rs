struct Plane {
    data: [u8; 9],
}

impl Plane {
    fn new() -> Plane {
        Plane {
            data: [0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    fn get_index(line: &u8, column: &u8) -> u8 {
        if Plane::is_allowed_line_or_column(line) {
            panic!("Invalid line value {}", *line);
        } else if Plane::is_allowed_line_or_column(column) {
            panic!("Invalid line value {}", *column);
        }

        return 3 * (*line - 1) + column - 1;
    }

    fn is_allowed_line_or_column(value: &u8) -> bool {
        *value == 0 || *value > 3
    }

    fn set_cell(&mut self, line: &u8, column: &u8, value: u8) {
        let index = Plane::get_index(line, column);

        self.data[index as usize] = value;
    }
}

fn main() {
    let mut plane = Plane::new();

    plane.set_cell(&10, &1, 3);

    println!("{:#?}", plane.data);
}
/*

[   0  1  2
--------------
    0, 0, 0, | 0
    0, 0, 0, | 1
    0, 0, 0, | 2
]
*/
