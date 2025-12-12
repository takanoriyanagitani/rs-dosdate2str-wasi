use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DosDateComponents {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub base_year_for_calculation: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Output {
    pub dos_date_components: DosDateComponents,
    pub formatted_date: String,
}

pub fn parse_dos_date(dos_date_val: u16) -> Result<DosDateComponents, String> {
    let year_offset = (dos_date_val >> 9) & 0b1111111;
    let year = 1980 + year_offset;
    let month = ((dos_date_val >> 5) & 0b1111) as u8;
    let day = (dos_date_val & 0b11111) as u8;

    // Rationale for date validation:
    // This function performs a balanced check for date validity. It rejects
    // clearly incorrect dates based on standard calendar rules (e.g., month 0,
    // day 0, day exceeding month's capacity) but intentionally avoids complex
    // calendar logic like leap year calculations to keep the implementation
    // simple and dependency-free.

    if month == 0 || month > 12 {
        return Err(format!(
            "Invalid month: {}. Month must be between 1 and 12.",
            month
        ));
    }
    if day == 0 {
        return Err(format!(
            "Invalid day: {}. Day must be between 1 and 31.",
            day
        ));
    }

    // February max 29 days; no leap year handling.
    let days_in_month = match month {
        2 => 29,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };

    if day > days_in_month {
        return Err(format!(
            "Day {} is out of range for month {}. Allowed day range is 1-{}.",
            day, month, days_in_month
        ));
    }

    Ok(DosDateComponents {
        year,
        month,
        day,
        base_year_for_calculation: 1980,
    })
}

pub fn format_output(dos_date_components: DosDateComponents) -> Output {
    let formatted_date = format!(
        "{}-{:02}-{:02}",
        dos_date_components.year, dos_date_components.month, dos_date_components.day
    );

    Output {
        dos_date_components,
        formatted_date,
    }
}

#[cfg(test)]
#[allow(clippy::identity_op)] // Allow identity operations for test clarity
mod tests {
    use super::*;

    #[test]
    fn test_parse_dos_date_valid() {
        // 2024-05-20
        let date_word_2024_05_20 = (44 << 9) | (5 << 5) | 20; // 0x58B4
        let result = parse_dos_date(date_word_2024_05_20).unwrap();
        assert_eq!(result.year, 2024);
        assert_eq!(result.month, 5);
        assert_eq!(result.day, 20);
    }

    #[test]
    fn test_parse_dos_date_invalid_month_0() {
        let date_word = (0 << 9) | (0 << 5) | 1; // 1980-00-01
        let err = parse_dos_date(date_word).unwrap_err();
        assert!(err.contains("Invalid month: 0"));
    }

    #[test]
    fn test_parse_dos_date_invalid_month_13() {
        let date_word = (0 << 9) | (13 << 5) | 1; // 1980-13-01
        let err = parse_dos_date(date_word).unwrap_err();
        assert!(err.contains("Invalid month: 13"));
    }

    #[test]
    fn test_parse_dos_date_invalid_day_0() {
        let date_word = (0 << 9) | (1 << 5) | 0; // 1980-01-00
        let err = parse_dos_date(date_word).unwrap_err();
        assert!(err.contains("Invalid day: 0"));
    }

    #[test]
    fn test_parse_dos_date_invalid_feb_30() {
        let date_word = (0 << 9) | (2 << 5) | 30; // 1980-02-30
        let err = parse_dos_date(date_word).unwrap_err();
        assert!(err.contains("Day 30 is out of range for month 2. Allowed day range is 1-29."));
    }

    #[test]
    fn test_parse_dos_date_invalid_apr_31() {
        let date_word = (0 << 9) | (4 << 5) | 31; // 1980-04-31
        let err = parse_dos_date(date_word).unwrap_err();
        assert!(err.contains("Day 31 is out of range for month 4. Allowed day range is 1-30."));
    }
}
