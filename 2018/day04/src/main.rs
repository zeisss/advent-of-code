use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
	WakesUp,
	FallsAsleep,
	BeginsShift(i32)
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Log {
	month: i32,
	day: i32,
	hour: i32,
	minute: i32,
	action: Action
}

use std::num::ParseIntError;

impl std::str::FromStr for Log {
	type Err = ParseIntError;
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let month = input[6..8].parse::<i32>()?;
		let day = input[9..11].parse::<i32>()?;
		let hour = input[12..14].parse::<i32>()?;
		let minute = input[15..17].parse::<i32>()?;

		let action = {
			let action_str : &str = input[19..].as_ref();
			match action_str {
				"wakes up" => Action::WakesUp,
				"falls asleep" => Action::FallsAsleep,
				_ => {
					let s : Vec<&str> = action_str.split_whitespace().collect();
					let guard_id = s[1][1..].parse::<i32>()?;
					Action::BeginsShift(guard_id)
				}
			}
		};

		Ok(Log{
			month: month,
			day: day,
			hour: hour,
			minute: minute,
			action: action,
		})
	}
}

#[test]
fn test_parser_time() {
	assert_eq!("[1518-08-29 01:59] wakes up".parse::<Log>(), Ok(Log{
		month: 8,
		day: 29,
		hour: 01,
		minute: 59,
		action: Action::WakesUp,
	}));
	assert_eq!("[1518-09-30 10:01] wakes up".parse::<Log>(), Ok(Log{
		month: 9,
		day: 30,
		hour: 10,
		minute: 01,
		action: Action::WakesUp,
	}));
}
#[test]
fn test_parser_action() {
	assert_eq!("[1518-08-29 01:59] wakes up".parse::<Log>(), Ok(Log{
		month: 8,
		day: 29,
		hour: 01,
		minute: 59,
		action: Action::WakesUp,
	}));
	assert_eq!("[1518-08-29 01:59] falls asleep".parse::<Log>(), Ok(Log{
		month: 8,
		day: 29,
		hour: 01,
		minute: 59,
		action: Action::FallsAsleep,
	}));
	assert_eq!("[1518-08-29 01:59] Guard #1733 begins shift".parse::<Log>(), Ok(Log{
		month: 8,
		day: 29,
		hour: 01,
		minute: 59,
		action: Action::BeginsShift(1733),
	}));
}

fn process(input: &str) -> (i32, i32) {
	let mut r : Vec<Log> = input.lines().flat_map(
		|l| l.parse::<Log>()
	).collect();
	r.sort();

	// map the list of logs into a <guard, sum of minutes sleept>
	// and <(guard, minute), count> HashMap.
	let mut guard_sum = HashMap::new();
	let mut guard_minutes = HashMap::new();

	let mut guard_on_duty : i32 = 0;
	let mut minute_fell_asleep : i32 = 0;
	for log in r.iter() {
		println!("> {:?}", log);
		match log.action {
		Action::BeginsShift(guard_id) => {
			guard_on_duty = guard_id;
		},
		Action::FallsAsleep => {
			if guard_on_duty == 0 {
				panic!("guard_id not set");
			}
			minute_fell_asleep = log.minute;
		},
		Action::WakesUp => {
			if guard_on_duty == 0 {
				panic!("guard_id not set");
			}
			let asleep = log.minute - minute_fell_asleep;
			println!("guard {} sleept for {} minutes", guard_on_duty, asleep);
			
			// increase the per-guard sum-asleep map			
			{
				let key = guard_on_duty;
				let new_value = guard_sum.get(&key).map(|x| x + asleep).unwrap_or(asleep);
				guard_sum.insert(key, new_value);
			}

			// increase the per-(guard, minute) map
			for i in minute_fell_asleep..log.minute {
				let key = (guard_on_duty, i);
				println!("#{} sleept at minute {}", guard_on_duty, i);
				let new_value = guard_minutes.get(&key).map(|x| x + 1).unwrap_or(1);
				guard_minutes.insert(key, new_value);
			}
		}
		}
	}

	let strategy1_result = {
		// find the guard with the highest amount of sleep
		let (guard_chosen, guard_sum_asleep) = {
			guard_sum.iter().fold((0,0), |(cg, cs), (g, s)| {
				if *s > cs {
					(*g, *s)
				} else {
					(cg, cs)
				}
			})
		};
		
		// find the minute he sleept most
		let guard_chosen_minute = {
			guard_minutes.iter().filter(|((guard, _minute), _count)| {
				*guard == guard_chosen
			})
			.fold((0,0), |(chosen_minute, chosen_count), ((_guard, minute), count)| {
				if chosen_count > *count {
					(chosen_minute, chosen_count)
				} else {
					(*minute, *count)
				}
			}).0
			
		};
		println!("#{:?} sleept longest with {} minutes total and \
				  mostly at minute {}", guard_chosen,guard_sum_asleep,
				 guard_chosen_minute);
		println!("answer: {}", guard_chosen * guard_chosen_minute);

		guard_chosen * guard_chosen_minute
	};
	// part 2
	// Strategy 2: Of all guards, which guard is most frequently 
	// asleep on the same minute?

	let strategy2_result = {
		let i = guard_minutes.iter().fold(
			// cg,cm,cc= chosen guard/minute/count
			(0,0,0), |(cg, cm, cc), ((guard, minute), count)| {
				if cc > *count {
					(cg, cm, cc)
				} else {
					(*guard, *minute, *count)
				}

			}
		);
		println!("i={:?}", i);
		i.0 * i.1
	};


	(strategy1_result, strategy2_result)
}

#[test]
fn test_example() {
	let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
	let (s1, s2) = process(input);
	assert_eq!(s1, 240);
	assert_eq!(s2, 4455);
}
fn main() {
	let input = include_str!("input.txt");
    println!("result = {:?}", process(input));
}
