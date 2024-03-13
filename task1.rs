use rand::Rng;
use std::time::Instant;

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

fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    // dbg!(&game_stamps[0..offset as usize]);
    let (_, st) = game_stamps.into_iter().enumerate().find(|(i, stamp)| {
        stamp.offset == offset || (stamp.offset < offset && offset < game_stamps[i + 1].offset)
    }).unwrap();
    (st.score.home, st.score.away)
}

#[test]
fn task1() {
    let start = Instant::now();
    dbg!(get_score(&generate_game()[0..], 10));
    dbg!(Instant::now() - start);
}
