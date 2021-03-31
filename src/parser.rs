use std::time::Duration;

#[derive(Debug, Eq, PartialEq)]
enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,//:mls
    Microsecond,//mcs
    NanoSecond,//nas
}


fn parse_str() -> anyhow::Result<Duration> {
    unimplemented!()
}