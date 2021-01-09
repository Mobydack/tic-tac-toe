struct Plane {
    data: [u8; 9],
}

impl Plane {
    fn new() -> Plane {
        Plane {
            data: [0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}

fn main() {
    let plane = Plane::new();

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
