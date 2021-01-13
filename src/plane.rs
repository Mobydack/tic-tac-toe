pub struct Plane<T> {
    data: [T; 9],
    index: usize,
}

#[derive(Debug, PartialEq)]
pub struct Cell<T> {
    row: u8,
    column: u8,
    value: T,
}

impl<T: Copy> Iterator for Plane<T> {
    type Item = Cell<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let result = Some(Cell {
                row: (self.index / 3) as u8 + 1,
                column: (self.index % 2) as u8 + 1,
                value: self.data[self.index].clone(),
            });

            self.index += 1;

            result
        } else {
            None
        }
    }
}

impl<T: Copy> Plane<T> {
    fn new(value: T) -> Plane<T> {
        Plane {
            data: [value; 9],
            index: 0,
        }
    }
}

impl<T: Copy> Plane<T> {
    fn get_index(&mut self, line: &u8, column: &u8) -> u8 {
        if self.is_allowed_line_or_column(line) {
            panic!("Invalid line value {}", *line);
        } else if self.is_allowed_line_or_column(column) {
            panic!("Invalid line value {}", *column);
        }

        return 3 * (*line - 1) + column - 1;
    }

    fn is_allowed_line_or_column(&self, value: &u8) -> bool {
        *value == 0 || *value > 3
    }

    pub fn set_cell(&mut self, line: &u8, column: &u8, value: T) {
        let index = self.get_index(line, column);

        self.data[index as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_set_cell() {
        let mut my_plane = Plane::new(' ');

        my_plane.set_cell(&1, &1, 'z');

        assert_eq!(
            Some(Cell {
                row: 1,
                column: 1,
                value: 'z'
            }),
            my_plane.next()
        );
    }
}
