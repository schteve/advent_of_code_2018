/*
    --- Day 4: Repose Record ---
    You've sneaked into another supply closet - this time, it's across from the prototype suit manufacturing lab. You need to sneak inside and fix the issues with the suit, but there's a guard stationed outside the lab, so this is as close as you can safely get.

    As you search the closet for anything that might help, you discover that you're not the first person to want to sneak in. Covering the walls, someone has spent an hour starting every midnight for the past few months secretly observing this guard post! They've been writing down the ID of the one guard on duty that night - the Elves seem to have decided that one guard was enough for the overnight shift - as well as when they fall asleep or wake up while at their post (your puzzle input).

    For example, consider the following records, which have already been organized into chronological order:

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
    [1518-11-05 00:55] wakes up
    Timestamps are written using year-month-day hour:minute format. The guard falling asleep or waking up is always the one whose shift most recently started. Because all asleep/awake times are during the midnight hour (00:00 - 00:59), only the minute portion (00 - 59) is relevant for those events.

    Visually, these records show that the guards are asleep at these times:

    Date   ID   Minute
                000000000011111111112222222222333333333344444444445555555555
                012345678901234567890123456789012345678901234567890123456789
    11-01  #10  .....####################.....#########################.....
    11-02  #99  ........................................##########..........
    11-03  #10  ........................#####...............................
    11-04  #99  ....................................##########..............
    11-05  #99  .............................................##########.....
    The columns are Date, which shows the month-day portion of the relevant day; ID, which shows the guard on duty that day; and Minute, which shows the minutes during which the guard was asleep within the midnight hour. (The Minute column's header shows the minute's ten's digit in the first row and the one's digit in the second row.) Awake is shown as ., and asleep is shown as #.

    Note that guards count as asleep on the minute they fall asleep, and they count as awake on the minute they wake up. For example, because Guard #10 wakes up at 00:25 on 1518-11-01, minute 25 is marked as awake.

    If you can figure out the guard most likely to be asleep at a specific time, you might be able to trick that guard into working tonight so you can have the best chance of sneaking in. You have two strategies for choosing the best guard/minute combination.

    Strategy 1: Find the guard that has the most minutes asleep. What minute does that guard spend asleep the most?

    In the example above, Guard #10 spent the most minutes asleep, a total of 50 minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes (10+10+10). Guard #10 was asleep most during minute 24 (on two days, whereas any other minute the guard was asleep was only seen on one day).

    While this example listed the entries in chronological order, your entries are in the order you found them. You'll need to organize them before they can be analyzed.

    What is the ID of the guard you chose multiplied by the minute you chose? (In the above example, the answer would be 10 * 24 = 240.)
*/

use lazy_static::lazy_static;
use regex::Regex;
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
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[(\d+)\-(\d+)\-(\d+) (\d+):(\d+)\]").unwrap();
        }
        let caps = RE.captures(input).unwrap();

        Self {
            year:   caps[1].parse::<u32>().unwrap(),
            month:  caps[2].parse::<u32>().unwrap(),
            day:    caps[3].parse::<u32>().unwrap(),
            hour:   caps[4].parse::<u32>().unwrap(),
            minute: caps[5].parse::<u32>().unwrap(),
        }
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
        // Begin shift
        lazy_static! {
            static ref RE_BEGIN_SHIFT: Regex = Regex::new(r"(\[\d+\-\d+\-\d+ \d+:\d+\]) Guard #(\d+) begins shift").unwrap();
        }
        if let Some(caps) = RE_BEGIN_SHIFT.captures(input) {
            return Self {
                timestamp: Timestamp::from_string(&caps[1]),
                action: GuardAction::BeginShift(caps[2].parse::<u32>().unwrap()),
            };
        }

        // Fall asleep
        lazy_static! {
            static ref RE_FALL_ASLEEP: Regex = Regex::new(r"(\[\d+\-\d+\-\d+ \d+:\d+\]) falls asleep").unwrap();
        }
        if let Some(caps) = RE_FALL_ASLEEP.captures(input) {
            return Self {
                timestamp: Timestamp::from_string(&caps[1]),
                action: GuardAction::FallAsleep,
            };
        }

        // Wake up
        lazy_static! {
            static ref RE_WAKE_UP: Regex = Regex::new(r"(\[\d+\-\d+\-\d+ \d+:\d+\]) wakes up").unwrap();
        }
        if let Some(caps) = RE_WAKE_UP.captures(input) {
            return Self {
                timestamp: Timestamp::from_string(&caps[1]),
                action: GuardAction::WakeUp,
            };
        }

        panic!("Unknown record");
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
                    let timeline = timelines.entry(guard_id.unwrap()).or_insert_with(|| vec![0; 60]);
                    for m in asleep_time.unwrap().minute .. r.timestamp.minute {
                        timeline[m as usize] += 1;
                    }
                },
            }
        }

        Self {
            timelines,
        }
    }

    fn strategy_1(&self) -> u32 {
        // Find ID of sleepiest guard
        let (&guard_id, _timeline) = self.timelines.iter()
                                                .max_by_key(|(_g_id, timeline)| timeline.iter().sum::<u32>())
                                                .unwrap();

        // Get the minute at which this guard is most asleep
        let (i, _minute) = self.timelines.get(&guard_id).expect("Invalid guard ID")
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

#[aoc(day4, part1)]
pub fn solve(input: &str) -> u32 {
    let mut records: Vec<Record> = input.lines().map(|line| Record::from_string(line)).collect();
    records.sort_by_key(|r| r.timestamp.get_u32());
    //records.iter().for_each(|r| println!("{:?}", r));

    let schedule = Schedule::from_records(&records);
    let answer = schedule.strategy_1();

    println!("Strategy 1: {}", answer);
    assert_eq!(answer, 102688);
    answer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strategy_1() {
        let input =
"[1518-11-01 00:00] Guard #10 begins shift
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
        let records: Vec<Record> = input.lines().map(|line| Record::from_string(line)).collect();

        let schedule = Schedule::from_records(&records);
        let answer = schedule.strategy_1();
        assert_eq!(answer, 240);
    }
}
