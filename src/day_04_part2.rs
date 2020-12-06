/*
    --- Part Two ---
    Strategy 2: Of all guards, which guard is most frequently asleep on the same minute?

    In the example above, Guard #99 spent minute 45 asleep more than any other guard or minute - three times in total. (In all other cases, any guard spent any minute asleep at most twice.)

    What is the ID of the guard you chose multiplied by the minute you chose? (In the above example, the answer would be 99 * 45 = 4455.)
*/

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Timestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl Timestamp {
    fn from_string(input: &str) -> Self {
        Self::parser(input).unwrap().1
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (year, month, day, hour, minute)) = tuple((
            preceded(char('['), map_res(digit1, |y: &str| y.parse::<u32>())),
            preceded(char('-'), map_res(digit1, |m: &str| m.parse::<u32>())),
            preceded(char('-'), map_res(digit1, |d: &str| d.parse::<u32>())),
            preceded(char(' '), map_res(digit1, |h: &str| h.parse::<u32>())),
            delimited(
                char(':'),
                map_res(digit1, |m: &str| m.parse::<u32>()),
                char(']'),
            ),
        ))(input)?;

        Ok((
            input,
            Self {
                year,
                month,
                day,
                hour,
                minute,
            },
        ))
    }

    fn get_u32(&self) -> u32 {
        (((self.year * 12 + self.month) * 31 + self.day) * 24 + self.hour) * 60 + self.minute
    }
}

#[derive(Clone, Copy, Debug)]
enum GuardAction {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}

#[derive(Clone, Copy, Debug)]
struct Record {
    timestamp: Timestamp,
    action: GuardAction,
}

impl Record {
    fn from_string(input: &str) -> Self {
        Self::parser(input).unwrap().1
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (timestamp, action)) = pair(
            Timestamp::parser,
            alt((
                map(
                    delimited(
                        tag(" Guard #"),
                        map_res(digit1, |guard_id: &str| guard_id.parse::<u32>()),
                        tag(" begins shift"),
                    ),
                    |guard_id| GuardAction::BeginShift(guard_id),
                ),
                map(tag(" falls asleep"), |_| GuardAction::FallAsleep),
                map(tag(" wakes up"), |_| GuardAction::WakeUp),
            )),
        )(input)?;

        Ok((input, Self { timestamp, action }))
    }
}

struct Schedule {
    timelines: HashMap<u32, Vec<u32>>,
}

impl Schedule {
    fn from_records(records: &[Record]) -> Self {
        let mut timelines: HashMap<u32, Vec<u32>> = HashMap::new();

        let mut guard_id: Option<u32> = None;
        let mut asleep_time: Option<Timestamp> = None;
        for r in records {
            match r.action {
                GuardAction::BeginShift(g_id) => guard_id = Some(g_id),
                GuardAction::FallAsleep => asleep_time = Some(r.timestamp),
                GuardAction::WakeUp => {
                    let timeline = timelines
                        .entry(guard_id.unwrap())
                        .or_insert_with(|| vec![0; 60]);
                    for m in asleep_time.unwrap().minute..r.timestamp.minute {
                        timeline[m as usize] += 1;
                    }
                }
            }
        }

        Self { timelines }
    }

    fn strategy_2(&self) -> u32 {
        // Find ID of guard with the minute with the most times asleep
        let (&guard_id, _timeline) = self
            .timelines
            .iter()
            .max_by_key(|(_g_id, timeline)| timeline.iter().max())
            .unwrap();

        // Get the minute at which this guard is most asleep
        let (i, _minute) = self
            .timelines
            .get(&guard_id)
            .expect("Invalid guard ID")
            .iter()
            .enumerate()
            .max_by_key(|(_i, &minute)| minute)
            .unwrap();

        encode_answer(guard_id, i as u32)
    }
}

fn encode_answer(guard_id: u32, minute: u32) -> u32 {
    guard_id * minute
}

#[aoc(day4, part2)]
pub fn solve(input: &str) -> u32 {
    let mut records: Vec<Record> = input
        .lines()
        .map(|line| Record::from_string(line))
        .collect();
    records.sort_by_key(|r| r.timestamp.get_u32());
    //records.iter().for_each(|r| println!("{:?}", r));

    let schedule = Schedule::from_records(&records);
    let answer = schedule.strategy_2();

    println!("Strategy 2: {}", answer);
    assert_eq!(answer, 56901);
    answer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strategy_2() {
        let input = "\
[1518-11-01 00:00] Guard #10 begins shift
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
        let records: Vec<Record> = input
            .lines()
            .map(|line| Record::from_string(line))
            .collect();

        let schedule = Schedule::from_records(&records);
        let answer = schedule.strategy_2();
        assert_eq!(answer, 4455);
    }
}
