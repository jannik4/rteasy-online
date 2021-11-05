use super::Value;

#[derive(Debug, Clone)]
pub enum SignedValue {
    Negative(Value),
    Positive(Value),
}

impl SignedValue {
    pub fn into_twos_complement(self, size: usize) -> Result<Value, SignedValue> {
        match self {
            SignedValue::Negative(mut value) => {
                if value.size() > size {
                    Err(SignedValue::Negative(value))
                } else {
                    value.extend_zero(size);
                    Ok(-value)
                }
            }
            SignedValue::Positive(mut value) => {
                if value.size() > size {
                    Err(SignedValue::Positive(value))
                } else {
                    value.extend_zero(size);
                    Ok(value)
                }
            }
        }
    }

    pub fn parse_bin(bin: &str) -> Result<Self, ()> {
        if bin.starts_with('-') {
            Ok(SignedValue::Negative(Value::parse_bin(&bin[1..])?))
        } else {
            Ok(SignedValue::Positive(Value::parse_bin(bin)?))
        }
    }

    pub fn parse_dec(dec: &str) -> Result<Self, ()> {
        if dec.starts_with('-') {
            Ok(SignedValue::Negative(Value::parse_dec(&dec[1..])?))
        } else {
            Ok(SignedValue::Positive(Value::parse_dec(dec)?))
        }
    }

    pub fn parse_hex(hex: &str) -> Result<Self, ()> {
        if hex.starts_with('-') {
            Ok(SignedValue::Negative(Value::parse_hex(&hex[1..])?))
        } else {
            Ok(SignedValue::Positive(Value::parse_hex(hex)?))
        }
    }
}

impl From<Value> for SignedValue {
    fn from(value: Value) -> Self {
        SignedValue::Positive(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Bit;

    #[test]
    fn test_into_twos_complement_positive() {
        assert_eq!(
            SignedValue::Positive(Value { bits: vec![Bit::One, Bit::One] })
                .into_twos_complement(2)
                .unwrap(),
            Value { bits: vec![Bit::One, Bit::One] }
        );

        assert_eq!(
            SignedValue::Positive(Value { bits: vec![Bit::One, Bit::One] })
                .into_twos_complement(3)
                .unwrap()
                .size(),
            3
        );

        assert!(SignedValue::Positive(Value { bits: vec![Bit::One, Bit::One, Bit::Zero] })
            .into_twos_complement(2)
            .is_err());
    }

    #[test]
    fn test_into_twos_complement_negative() {
        assert_eq!(
            SignedValue::Negative(Value { bits: vec![Bit::One, Bit::Zero] })
                .into_twos_complement(2)
                .unwrap(),
            Value { bits: vec![Bit::One, Bit::One] }
        );

        assert_eq!(
            SignedValue::Negative(Value { bits: vec![Bit::One, Bit::One] })
                .into_twos_complement(3)
                .unwrap()
                .size(),
            3
        );

        assert!(SignedValue::Negative(Value { bits: vec![Bit::One, Bit::One, Bit::Zero] })
            .into_twos_complement(2)
            .is_err());
    }

    #[test]
    fn test_parse_bin() {
        let parsed = SignedValue::parse_bin("-01").unwrap();
        assert!(matches!(parsed, SignedValue::Negative(..)));
        assert_eq!(
            parsed.into_twos_complement(2).unwrap(),
            Value { bits: vec![Bit::One, Bit::One] }
        );

        assert!(matches!(SignedValue::parse_bin("01010").unwrap(), SignedValue::Positive(..)));

        assert!(SignedValue::parse_bin("--01010").is_err());
        assert!(SignedValue::parse_bin("-").is_err());
        assert!(SignedValue::parse_bin("- 1").is_err());
    }

    #[test]
    fn test_parse_dec() {
        let parsed = SignedValue::parse_dec("-3").unwrap();
        assert!(matches!(parsed, SignedValue::Negative(..)));
        assert_eq!(
            parsed.into_twos_complement(4).unwrap(),
            Value { bits: vec![Bit::One, Bit::Zero, Bit::One, Bit::One] }
        );

        assert!(matches!(SignedValue::parse_dec("379856").unwrap(), SignedValue::Positive(..)));

        assert!(SignedValue::parse_dec("--495783").is_err());
        assert!(SignedValue::parse_dec("-").is_err());
        assert!(SignedValue::parse_dec("- 1").is_err());
    }

    #[test]
    fn test_parse_hex() {
        let parsed = SignedValue::parse_hex("-FF").unwrap();
        assert!(matches!(parsed, SignedValue::Negative(..)));
        assert_eq!(
            parsed.into_twos_complement(12).unwrap(),
            Value {
                bits: vec![
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One
                ]
            }
        );

        assert!(matches!(SignedValue::parse_hex("0A7e").unwrap(), SignedValue::Positive(..)));

        assert!(SignedValue::parse_hex("--ff").is_err());
        assert!(SignedValue::parse_hex("-").is_err());
        assert!(SignedValue::parse_hex("- F").is_err());
    }

    #[test]
    fn test_from_value() {
        assert!(matches!(
            SignedValue::from(Value { bits: vec![Bit::One, Bit::Zero] }),
            SignedValue::Positive(..)
        ));
    }
}
