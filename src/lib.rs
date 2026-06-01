/// Declares an enum that also has an older representation.
///
/// Serde deserialization will attempt to deserialize using the
/// representation declared, but if that fails, the older
/// representation will be tried.  For this to work, you must
/// implement `From<new> for old`.  The older representation can
/// also be declared via `serde_de_chain`, in which case the deserialization
/// chain will be extended.
///
/// # Parameters
/// * `attr` - Zero or more attribute blocks, e.g., `#[derive(Debug, Eq, PartialEq)]`
/// * `vis` - Visibility, which can be blank, `pub`, etc.
/// * `new` - The identifier for the enum you're defining
/// * `old` - The identifier of the previous representation
/// * `variants` - A comma separated list of variants for this enum
///
/// # Examples
/// This snippet from the tests illustrates the syntax and
/// chaining.
///
/// The current representation is called `Report`, with the
/// previous version being `ReportV1` and an original representation
/// being `ReportV0`.  Trying to deserialize a
/// `serde_json::Value` that is in `ReportV0` form into a `Report`
/// succeeds, due `Report` deserialization failing, which then falls
/// back to `ReportV1` deserialization failing which then fails back
/// to `ReportV0` deserialization.
/// ```
/// # use serde::{Deserialize, Serialize};
/// #
/// # #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
/// # struct Year(u16);
/// #
/// # #[derive(Clone, Copy, Debug, Deserialize, Ord, PartialEq, PartialOrd, Serialize, Eq, Hash)]
/// # struct PlayerId(i32);
/// #
/// # #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
/// # enum ReportSpan {
/// #     All,
/// #     YearToDate,
/// #     MonthToDate,
/// #     WeekToDate,
/// #     OneYear,
/// #     OneMonth,
/// #     OneWeek,
/// # }
/// #
/// use serde_de_chain::serde_de_chain;
///
/// serde_de_chain! {
///     #[derive(Debug, Eq, PartialEq)]
///     enum Report <- ReportV1 {
///         LeaderboardWSOPS(Year, Option<PlayerId>, ReportSpan),
///         LeaderboardWYWAB(Year, Option<PlayerId>, ReportSpan),
///         LeaderboardWYWAE(Year, Option<PlayerId>, ReportSpan),
///     }
/// }
///
/// impl From<ReportV1> for Report {
///     fn from(v: ReportV1) -> Self {
///         use {Report as O, ReportV1::*};
///
///         match v {
///             LeaderboardWSOPS(year, pid, span) => O::LeaderboardWSOPS(year, pid, span),
///             LeaderboardWYWAB2024(pid, span) => O::LeaderboardWYWAB(Year(2024), pid, span),
///             LeaderboardWYWAE2025(pid, span) => O::LeaderboardWYWAB(Year(2025), pid, span),
///             LeaderboardWYWAE2026(pid, span) => O::LeaderboardWYWAE(Year(2026), pid, span),
///         }
///     }
/// }
///
/// serde_de_chain! {
///     #[derive(Debug, Eq, PartialEq)]
///     enum ReportV1 <- ReportV0 {
///         LeaderboardWSOPS(Year, Option<PlayerId>, ReportSpan),
///         LeaderboardWYWAB2024(Option<PlayerId>, ReportSpan),
///         LeaderboardWYWAE2025(Option<PlayerId>, ReportSpan),
///         LeaderboardWYWAE2026(Option<PlayerId>, ReportSpan),
///     }
/// }
///
/// impl From<ReportV0> for ReportV1 {
///     fn from(v: ReportV0) -> Self {
///         use {ReportV0::*, ReportV1 as O};
///
///         match v {
///             LeaderboardWSOPS2022(pid, span) => O::LeaderboardWSOPS(Year(2022), pid, span),
///             LeaderboardWSOPS2023(pid, span) => O::LeaderboardWSOPS(Year(2023), pid, span),
///             LeaderboardWSOPS2024(pid, span) => O::LeaderboardWSOPS(Year(2024), pid, span),
///             LeaderboardWSOPS2025(pid, span) => O::LeaderboardWSOPS(Year(2025), pid, span),
///             LeaderboardWSOPS2026(pid, span) => O::LeaderboardWSOPS(Year(2026), pid, span),
///             LeaderboardWYWAB2024(pid, span) => O::LeaderboardWYWAB2024(pid, span),
///             LeaderboardWYWAE2025(pid, span) => O::LeaderboardWYWAE2025(pid, span),
///             LeaderboardWYWAE2026(pid, span) => O::LeaderboardWYWAE2026(pid, span),
///         }
///     }
/// }
///
/// #[derive(Debug, Deserialize, Eq, PartialEq)]
/// enum ReportV0 {
///     LeaderboardWSOPS2022(Option<PlayerId>, ReportSpan),
///     LeaderboardWSOPS2023(Option<PlayerId>, ReportSpan),
///     LeaderboardWSOPS2024(Option<PlayerId>, ReportSpan),
///     LeaderboardWSOPS2025(Option<PlayerId>, ReportSpan),
///     LeaderboardWSOPS2026(Option<PlayerId>, ReportSpan),
///     LeaderboardWYWAB2024(Option<PlayerId>, ReportSpan),
///     LeaderboardWYWAE2025(Option<PlayerId>, ReportSpan),
///     LeaderboardWYWAE2026(Option<PlayerId>, ReportSpan),
/// }
/// ```
#[macro_export]
macro_rules! serde_de_chain {
    (
        $(#[$attr:meta])*
        $vis:vis enum $new:ident <- $old:ident { $($variants:tt)* }
    ) => {
        $(#[$attr])*
        $vis enum $new {
            $($variants)*
        }

        impl<'de> serde::Deserialize<'de> for $new {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                #[derive(serde::Deserialize)]
                enum New {
                    $($variants)*
                }

                #[derive(serde::Deserialize)]
                #[serde(untagged)]
                enum NewOld {
                    New(New),
                    Old($old),
                }

                NewOld::deserialize(deserializer).map(|v| match v {
                    NewOld::New(new) => unsafe { std::mem::transmute(new) },
                    NewOld::Old(old) => old.into(),
                })
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

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

    serde_de_chain! {
        #[derive(Debug, Eq, PartialEq)]
        enum Report <- ReportV1 {
            #[allow(dead_code)]
            LeaderboardWSOPS(Year, Option<PlayerId>, ReportSpan),
            #[allow(dead_code)]
            LeaderboardWYWAB(Year, Option<PlayerId>, ReportSpan),
            #[allow(dead_code)]
            LeaderboardWYWAE(Year, Option<PlayerId>, ReportSpan),
        }
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

    serde_de_chain! {
        #[derive(Debug, Eq, PartialEq)]
        enum ReportV1 <- ReportV0 {
            #[allow(dead_code)]
            LeaderboardWSOPS(Year, Option<PlayerId>, ReportSpan),
            #[allow(dead_code)]
            LeaderboardWYWAB2024(Option<PlayerId>, ReportSpan),
            #[allow(dead_code)]
            LeaderboardWYWAE2025(Option<PlayerId>, ReportSpan),
            #[allow(dead_code)]
            LeaderboardWYWAE2026(Option<PlayerId>, ReportSpan),
        }
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
}
