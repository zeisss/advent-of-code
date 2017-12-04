
fn number_spiral_pos(n: u32) -> (i32, i32) {
	if n == 1 {
		return (0,0)
	}
	let mut x : i32 = 0;
	let mut y : i32 = 0;
	let mut dx : i32 = 0;
	let mut dy : i32 = 0;
	let mut range = 1;
	let mut copy_range = 0;

	//println!("## n={}", n);
	for i in 0..n {
		x += dx;
		y += dy;

		//println!("Number {} -> pos=({}, {}) delta=({}, {}) range={}", i + 1, x, y, dx, dy, range);
		range -= 1;
		if range == 0 {
			if dx > 0 {
				dx = 0;
				dy = -1;
			} else if dy < 0 {
				dx = -1;
				dy = 0;
				copy_range += 1;
			} else if dx < 0 {
				dx = 0; 
				dy = 1;
			} else if dy > 0 {
				dx = 1;
				dy = 0;
				copy_range += 1;
			} else {
				dx = 1;
				dy = 0;
				copy_range = 1;
			}
			range = copy_range;
		}
	}

	println!("=> ({}, {})\n\n", x, y);
	(x, y)
}

#[test]
fn test_number_spiral_pos() {
	assert_eq!(  (0, 0), number_spiral_pos(1));
	assert_eq!(  (1, 0), number_spiral_pos(2));
	assert_eq!( (1, -1), number_spiral_pos(3));
	assert_eq!( (0, -1), number_spiral_pos(4));
	assert_eq!((-1, -1), number_spiral_pos(5));

	assert_eq!((2, -1), number_spiral_pos(12));
}

fn count_steps(n: u32) -> i32 {
	let (x,y) = number_spiral_pos(n);

	x.abs() + y.abs()
}

#[test]
fn test_step_counter() {
	assert_eq!(0, count_steps(1));
	assert_eq!(3, count_steps(12));
	assert_eq!(2, count_steps(23));
	assert_eq!(31, count_steps(1024));
}
fn main() {
    println!("Steps: {}", count_steps(289326));
}

