/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use std::str::FromStr;
use std::time::Duration;

use hurl_core::types::DurationUnit;
use regex::Regex;

use super::CliOptionsError;

/// Parses a string with or without time unit into a `Duration`.
///
/// When there is no time unit in the user string, the duration is parsed with a default time unit.
///
/// Example: `32s`, `10m`, `20000`.
///
pub fn duration_from_str(
    value: &str,
    default_unit: DurationUnit,
) -> Result<Duration, CliOptionsError> {
    let re = Regex::new(r"^(\d+)([a-zA-Z]*)$").unwrap();
    let Some(caps) = re.captures(value) else {
        let error = CliOptionsError::Error("Invalid duration".to_string());
        return Err(error);
    };
    let source = caps.get(1).unwrap().as_str().to_string();
    let duration = source
        .parse::<u64>()
        .map_err(|_| CliOptionsError::Error("Duration value too large".to_string()))?;
    let unit = caps.get(2).unwrap().as_str();
    let unit = if unit.is_empty() {
        default_unit
    } else {
        DurationUnit::from_str(unit).map_err(CliOptionsError::Error)?
    };
    let millis = match unit {
        DurationUnit::MilliSecond => Some(duration),
        DurationUnit::Second => duration.checked_mul(1000),
        DurationUnit::Minute => duration.checked_mul(1000 * 60),
        DurationUnit::Hour => duration.checked_mul(1000 * 60 * 60),
    };
    match millis {
        Some(millis) => Ok(Duration::from_millis(millis)),
        None => {
            let error = CliOptionsError::Error("Duration value too large".to_string());
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_error() {
        assert_eq!(
            duration_from_str("", DurationUnit::MilliSecond).unwrap_err(),
            CliOptionsError::Error("Invalid duration".to_string())
        );
        assert_eq!(
            duration_from_str("s", DurationUnit::MilliSecond).unwrap_err(),
            CliOptionsError::Error("Invalid duration".to_string())
        );
        assert_eq!(
            duration_from_str("10s10", DurationUnit::MilliSecond).unwrap_err(),
            CliOptionsError::Error("Invalid duration".to_string())
        );
        assert_eq!(
            duration_from_str("10mm", DurationUnit::MilliSecond).unwrap_err(),
            CliOptionsError::Error("Invalid duration unit mm".to_string())
        );
    }

    #[test]
    pub fn test_parse() {
        assert_eq!(
            duration_from_str("10", DurationUnit::MilliSecond).unwrap(),
            Duration::from_millis(10)
        );
        assert_eq!(
            duration_from_str("10s", DurationUnit::MilliSecond).unwrap(),
            Duration::from_secs(10)
        );
        assert_eq!(
            duration_from_str("10000ms", DurationUnit::Second).unwrap(),
            Duration::from_millis(10000)
        );
        assert_eq!(
            duration_from_str("5m", DurationUnit::Second).unwrap(),
            Duration::from_secs(5 * 60)
        );
        assert_eq!(
            duration_from_str("3h", DurationUnit::MilliSecond).unwrap(),
            Duration::from_hours(3)
        );
    }
}
