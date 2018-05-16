use timer::interval::Interval;
use timer::once::Once;

/// `Enum` wrapping all the `Timer`s together.
///
/// This derives interior `Timer`s.
/// Derive precedence is as follows:
///   * Interval
///   * Once
#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Timing {
    Interval(Interval),
    Once(Once),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use timer::Timer;

    #[test]
    fn once_with_delay() {
        let derived: Timing = serde_json::from_str(
            r#"{
                "delay": 0
            }"#,
        ).unwrap();

        let actual = Timing::Once(Once::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn once_without_delay() {
        let derived: Timing = serde_json::from_str(r#"{}"#).unwrap();

        let actual = Timing::Once(Once::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn interval_with_delay() {
        let derived: Timing = serde_json::from_str(
            r#"{
                "delay": 0,
                "interval": 1000
            }"#,
        ).unwrap();

        let actual = Timing::Interval(Interval::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn interval_without_delay() {
        let derived: Timing = serde_json::from_str(
            r#"{
                "interval": 1000
            }"#,
        ).unwrap();

        let actual = Timing::Interval(Interval::default());

        assert_eq!(derived, actual);
    }
}