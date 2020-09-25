use cynic::{serde_json::Value, DecodeError, Scalar, SerializeError};

use crate::{Int64Scalar, UInt64Scalar};

impl Scalar for Int64Scalar {
    fn decode(value: &Value) -> Result<Self, DecodeError> {
        match value {
            Value::String(string) => {
                let integer: i64 = string
                    .parse()
                    .map_err(|_| DecodeError::Other(format!("Invalid value {}", value)))?;

                Ok(Self(integer))
            }
            _ => Err(DecodeError::IncorrectType(
                "String".to_owned(),
                format!("Invalid value {}", value),
            )),
        }
    }

    fn encode(&self) -> Result<Value, SerializeError> {
        Ok(Value::String(self.0.to_string()))
    }
}

impl Scalar for UInt64Scalar {
    fn decode(value: &Value) -> Result<Self, DecodeError> {
        match value {
            Value::String(string) => {
                let integer: u64 = string
                    .parse()
                    .map_err(|_| DecodeError::Other(format!("Invalid value {}", value)))?;

                Ok(Self(integer))
            }
            _ => Err(DecodeError::IncorrectType(
                "String".to_owned(),
                format!("Invalid value {}", value),
            )),
        }
    }

    fn encode(&self) -> Result<Value, SerializeError> {
        Ok(Value::String(self.0.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io;

    #[test]
    fn with_i64() -> io::Result<()> {
        assert_eq!(
            Int64Scalar(i64::MAX)
                .encode()
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?,
            Value::String("9223372036854775807".to_owned())
        );
        assert_eq!(
            Int64Scalar::decode(&Value::String("9223372036854775807".to_owned()))
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?,
            Int64Scalar(i64::MAX)
        );
        Ok(())
    }

    #[test]
    fn with_u64() -> io::Result<()> {
        assert_eq!(
            UInt64Scalar(u64::MAX)
                .encode()
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?,
            Value::String("18446744073709551615".to_owned())
        );
        assert_eq!(
            UInt64Scalar::decode(&Value::String("18446744073709551615".to_owned()))
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?,
            UInt64Scalar(u64::MAX)
        );
        Ok(())
    }
}
