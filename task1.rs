use rand::Rng;
use std::{fmt::{Display, Formatter}, time::Instant};

const TIMESTAMPS_COUNT: usize = 50000;

const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;

const PROBABILITY_HOME_SCORE: f64 = 0.45;

const OFFSET_MAX_STEP: i32 = 3;

const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

#[derive(Debug, Clone, Copy)]
struct Score {
    home: i32,
    away: i32,
}

#[derive(Debug, Clone, Copy)]
struct Stamp {
    offset: i32,
    score: Score,
}

impl Stamp {
    fn into_score(&self) -> String {
        format!("| home: {} away: {} |", self.score.home, self.score.away)
    }

    fn generate_stamp(previous_value: Stamp) -> Stamp {
        let score_changed: bool = rand::thread_rng().gen_bool(1.0 - PROBABILITY_SCORE_CHANGED);
        let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
        let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);
        Stamp {
            offset: previous_value.offset + offset_change,
            score: Score {
                home: previous_value.score.home
                    + if score_changed && home_score_change {
                        1
                    } else {
                        0
                    },
                away: previous_value.score.away
                    + if score_changed && !home_score_change {
                        1
                    } else {
                        0
                    },
            },
        }
    }
}

fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = Stamp::generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }
    stamps
}

enum ScoreCase {
    Exact(Stamp),
    Nearest(Stamp, i32),
    TooMatch(Stamp)
}

impl Display for ScoreCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ScoreCase::Exact(stamp) => format!("Счет: {}", stamp.into_score()),
            ScoreCase::Nearest(stamp, offset) => format!(
                "Offset {} не был зафикисрован. Ближайший offset {} со счетом: {}",
                offset,
                stamp.offset,
                stamp.into_score()
            ),
            ScoreCase::TooMatch(stamp) => format!("Игра была окончена на момент offset {}. Последний счет {}", stamp.offset, stamp.into_score()),
        };
        write!(f, "{message}")
    }
}

fn get_score(game_stamps: &[Stamp], offset: i32) -> ScoreCase {
    // dbg!(&game_stamps[0..offset as usize]);
    let last = game_stamps.last().unwrap();
    if offset > last.offset {
        return ScoreCase::TooMatch(*last);
    }

    for (i, stamp) in game_stamps.into_iter().enumerate() {
        if stamp.offset == offset {
            return ScoreCase::Exact(*stamp);
        }
        if stamp.offset < offset && offset < game_stamps[i + 1].offset {
            return ScoreCase::Nearest(*stamp, offset);
        }
    }
    panic!()
}

#[test]
fn task1() {
    use ScoreCase::*;
    assert!(
        match get_score(&generate_game()[0..=100], 100 * OFFSET_MAX_STEP + 1) {
            TooMatch(..) => true,
            _ => false
        }
    );

    // let start = Instant::now();
    println!("{}", get_score(&generate_game()[0..], 100 * OFFSET_MAX_STEP + 1));
    // dbg!(Instant::now() - start);
}
