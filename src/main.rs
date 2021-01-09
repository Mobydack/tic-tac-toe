mod plane;

fn main() {
    let mut plane = plane::create_plane::<char>(' ');

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
