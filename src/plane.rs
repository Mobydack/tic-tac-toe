pub struct Plane<T> {
    pub data: [T; 9],
    line_length: u8,
}

pub fn create_plane<T: Copy>(fill_value: T) -> Plane<T> {
    Plane {
        data: [fill_value; 9],
        line_length: 3,
    }
}

impl<T> Plane<T> {
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

    pub fn set_cell(&mut self, line: &u8, column: &u8, value: T) {
        let index = self.get_index(line, column);

        self.data[index as usize] = value;
    }
}
