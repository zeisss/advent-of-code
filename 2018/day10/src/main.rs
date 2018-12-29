fn is_splitter(c: char) -> bool {
	c == '<' || c == ',' || c == '>'
}

type P = i64;
type Pos = (P, P);
type Delta = i64;

#[derive(Debug)]
struct Star {
	pos: Pos,
	dx: Delta,
	dy: Delta,
}

impl Star {
	fn tick(&mut self) {
		self.pos.0 += self.dx;
		self.pos.1 += self.dy;
	}

	fn position(&self) -> Pos {
		self.pos
	}
}

#[test]
fn test_star() {
	let mut s = Star{pos: (3, 9), dx: 1, dy: -2};
	s.tick();
	s.tick();
	s.tick();
	assert_eq!((6, 3), s.position());
}

fn edges(v: &Vec<Star>) -> (Pos, Pos) {
	let minx = v.iter().map(|s| s.pos.0 ).min().unwrap();
	let miny = v.iter().map(|s| s.pos.1 ).min().unwrap();
	let maxx = v.iter().map(|s| s.pos.0 ).max().unwrap();
	let maxy = v.iter().map(|s| s.pos.1 ).max().unwrap();


	((minx, miny), (maxx, maxy))
}

fn calc(v: &mut Vec<Star>) {
	let mut i = 1;
	while true {
		v.iter_mut().for_each(|s| s.tick());

		let (first, second) = edges(v);

		let limit = 15;
		if (first.0 - second.0).abs() < limit || (first.1 - second.1).abs() < limit {
			println!("#{}", i);
			render(v);
			break
		}
		// else { print!("."); }

		i+=1;
	}
}

fn render(v: &Vec<Star>) {
	let (first, second) = edges(v);

	for y in 0..24 {
		for x in 0..80 {
			let p : Pos = (first.0 + x, first.1 + y);
			if let Some(_s) = v.iter().filter(|s| s.pos == p).next() {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!("");
	}
}

fn parse(s: &str) -> Vec<Star> {
	s.lines().map(|l| {
		let i : Vec<&str> = l.split(is_splitter).collect();
		// println!("{:?}", i);

		Star{
			pos: (i[1].trim().parse::<P>().unwrap(), i[2].trim().parse::<P>().unwrap()),
			dx: i[4].trim().parse::<P>().unwrap(),
			dy: i[5].trim().parse::<P>().unwrap(),
		}
	}).collect()
}

#[test]
fn test_parse_examples() {
	let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

	let parsed = parse(input);

	assert_eq!(31, parsed.len());
	assert_eq!(9, parsed[0].pos.0);
	assert_eq!(1, parsed[0].pos.1);
	assert_eq!(0, parsed[0].dx);
	assert_eq!(2, parsed[0].dy);

	assert_eq!(7, parsed[1].pos.0);
	assert_eq!(0, parsed[1].pos.1);
	assert_eq!(-1, parsed[1].dx);
	assert_eq!(0, parsed[1].dy);
}

fn main() {
	let input = include_str!("input.txt");
	let mut parsed = parse(input);
	calc(&mut parsed);
}
