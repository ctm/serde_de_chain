use serde::{Deserialize, Serialize};
use serde_de_chain::SerdeDeChain;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
struct Year(u16);

#[derive(Clone, Copy, Debug, Deserialize, Ord, PartialEq, PartialOrd, Serialize, Eq, Hash)]
struct PlayerId(i32);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
enum ReportSpan {
    All,
    YearToDate,
    MonthToDate,
    WeekToDate,
    OneYear,
    OneMonth,
    OneWeek,
}

#[derive(Debug, Eq, PartialEq, SerdeDeChain)]
#[serde_de_chain(ReportV1)]
enum Report {
    #[allow(dead_code)]
    LeaderboardWSOPS(Year, Option<PlayerId>, ReportSpan),
    #[allow(dead_code)]
    LeaderboardWYWAB(Year, Option<PlayerId>, ReportSpan),
    #[allow(dead_code)]
    LeaderboardWYWAE(Year, Option<PlayerId>, ReportSpan),
}

impl From<ReportV1> for Report {
    fn from(v: ReportV1) -> Self {
        use {Report as O, ReportV1::*};

        match v {
            LeaderboardWSOPS(year, pid, span) => O::LeaderboardWSOPS(year, pid, span),
            LeaderboardWYWAB2024(pid, span) => O::LeaderboardWYWAB(Year(2024), pid, span),
            LeaderboardWYWAE2025(pid, span) => O::LeaderboardWYWAB(Year(2025), pid, span),
            LeaderboardWYWAE2026(pid, span) => O::LeaderboardWYWAE(Year(2026), pid, span),
        }
    }
}

#[derive(Debug, Eq, PartialEq, SerdeDeChain)]
#[serde_de_chain(ReportV0)]
enum ReportV1 {
    #[allow(dead_code)]
    LeaderboardWSOPS(Year, Option<PlayerId>, ReportSpan),
    #[allow(dead_code)]
    LeaderboardWYWAB2024(Option<PlayerId>, ReportSpan),
    #[allow(dead_code)]
    LeaderboardWYWAE2025(Option<PlayerId>, ReportSpan),
    #[allow(dead_code)]
    LeaderboardWYWAE2026(Option<PlayerId>, ReportSpan),
}

impl From<ReportV0> for ReportV1 {
    fn from(v: ReportV0) -> Self {
        use {ReportV0::*, ReportV1 as O};

        match v {
            LeaderboardWSOPS2022(pid, span) => O::LeaderboardWSOPS(Year(2022), pid, span),
            LeaderboardWSOPS2023(pid, span) => O::LeaderboardWSOPS(Year(2023), pid, span),
            LeaderboardWSOPS2024(pid, span) => O::LeaderboardWSOPS(Year(2024), pid, span),
            LeaderboardWSOPS2025(pid, span) => O::LeaderboardWSOPS(Year(2025), pid, span),
            LeaderboardWSOPS2026(pid, span) => O::LeaderboardWSOPS(Year(2026), pid, span),
            LeaderboardWYWAB2024(pid, span) => O::LeaderboardWYWAB2024(pid, span),
            LeaderboardWYWAE2025(pid, span) => O::LeaderboardWYWAE2025(pid, span),
            LeaderboardWYWAE2026(pid, span) => O::LeaderboardWYWAE2026(pid, span),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
enum ReportV0 {
    LeaderboardWSOPS2022(Option<PlayerId>, ReportSpan),
    LeaderboardWSOPS2023(Option<PlayerId>, ReportSpan),
    LeaderboardWSOPS2024(Option<PlayerId>, ReportSpan),
    LeaderboardWSOPS2025(Option<PlayerId>, ReportSpan),
    LeaderboardWSOPS2026(Option<PlayerId>, ReportSpan),
    LeaderboardWYWAB2024(Option<PlayerId>, ReportSpan),
    LeaderboardWYWAE2025(Option<PlayerId>, ReportSpan),
    LeaderboardWYWAE2026(Option<PlayerId>, ReportSpan),
}

#[derive(Debug, Eq, PartialEq, SerdeDeChain)]
#[serde_de_chain(PointV0)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl From<PointV0> for Point {
    fn from(v: PointV0) -> Self {
        Point {
            x: v.x,
            y: v.y,
            z: 0,
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct PointV0 {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, SerdeDeChain)]
#[serde_de_chain(PairV0)]
struct Pair(i32, i32, i32);

impl From<PairV0> for Pair {
    fn from(v: PairV0) -> Self {
        Pair(v.0, v.1, 0)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct PairV0(i32, i32);

fn test<T: std::fmt::Debug + PartialEq + for<'de> serde::Deserialize<'de>>(
    v: serde_json::Value,
    expected: T,
) {
    match serde_json::from_value::<T>(v) {
        Err(e) => {
            panic!("{e:?}");
        }
        Ok(r) => {
            assert_eq!(r, expected);
        }
    }
}

#[test]
fn v0() {
    test::<ReportV0>(
        serde_json::json!({
            "LeaderboardWYWAE2025": [10, "All"]
        }),
        ReportV0::LeaderboardWYWAE2025(Some(PlayerId(10)), ReportSpan::All),
    );
}

#[test]
fn v1() {
    test::<ReportV1>(
        serde_json::json!({
            "LeaderboardWSOPS": [2026, 10, "All"]
        }),
        ReportV1::LeaderboardWSOPS(Year(2026), Some(PlayerId(10)), ReportSpan::All),
    );
}

#[test]
fn v0_as_v1() {
    test::<ReportV1>(
        serde_json::json!({
            "LeaderboardWSOPS2026": [10, "All"]
        }),
        ReportV1::LeaderboardWSOPS(Year(2026), Some(PlayerId(10)), ReportSpan::All),
    );
}

#[test]
fn v0_as_report() {
    test::<Report>(
        serde_json::json!({
            "LeaderboardWSOPS2026": [10, "All"]
        }),
        Report::LeaderboardWSOPS(Year(2026), Some(PlayerId(10)), ReportSpan::All),
    );
}

#[test]
fn point_new() {
    test::<Point>(
        serde_json::json!({ "x": 1, "y": 2, "z": 3 }),
        Point { x: 1, y: 2, z: 3 },
    );
}

#[test]
fn point_old_as_new() {
    test::<Point>(
        serde_json::json!({ "x": 1, "y": 2 }),
        Point { x: 1, y: 2, z: 0 },
    );
}

#[test]
fn pair_new() {
    test::<Pair>(serde_json::json!([1, 2, 3]), Pair(1, 2, 3));
}

#[test]
fn pair_old_as_new() {
    test::<Pair>(serde_json::json!([1, 2]), Pair(1, 2, 0));
}
