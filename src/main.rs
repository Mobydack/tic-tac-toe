struct Plane<T> {
    data: [T; 9],
    line_length: u8,
}

impl<T: Copy> Plane<T> {
    fn new(fill_value: T) -> Plane<T> {
        Plane {
            data: [fill_value; 9],
            line_length: 3,
        }
    }

    fn get_index(&self, line: &u8, column: &u8) -> u8 {
        if self.is_allowed_line_or_column(line) {
            panic!("Invalid line value {}", *line);
        } else if self.is_allowed_line_or_column(column) {
            panic!("Invalid line value {}", *column);
        }

        return self.line_length * (*line - 1) + column - 1;
    }

    fn is_allowed_line_or_column(&self, value: &u8) -> bool {
        *value == 0 || *value > self.line_length
    }

    fn set_cell(&mut self, line: &u8, column: &u8, value: T) {
        let index = self.get_index(line, column);

        self.data[index as usize] = value;
    }
}

fn main() {
    let mut plane: Plane<char> = Plane::new(' ');

    plane.set_cell(&1, &1, 'x');

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
