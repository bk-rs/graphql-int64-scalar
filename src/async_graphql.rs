use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

use crate::{Int64Scalar, UInt64Scalar};

#[Scalar]
impl ScalarType for Int64Scalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        match &value {
            Value::String(string) => {
                let integer: i64 = string
                    .parse()
                    .map_err(|_| InputValueError::Custom(format!("Invalid value {}", value)))?;
                Ok(Self(integer))
            }
            _ => Err(InputValueError::ExpectedType(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

#[Scalar]
impl ScalarType for UInt64Scalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        match &value {
            Value::String(string) => {
                let integer: u64 = string
                    .parse()
                    .map_err(|_| InputValueError::Custom(format!("Invalid value {}", value)))?;
                Ok(Self(integer))
            }
            _ => Err(InputValueError::ExpectedType(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io;

    #[test]
    fn with_i64() -> io::Result<()> {
        assert_eq!(
            Int64Scalar(i64::MAX).to_value(),
            Value::String("9223372036854775807".to_owned())
        );
        assert_eq!(
            Int64Scalar::parse(Value::String("9223372036854775807".to_owned()))
                .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))?,
            Int64Scalar(i64::MAX)
        );
        Ok(())
    }

    #[test]
    fn with_u64() -> io::Result<()> {
        assert_eq!(
            UInt64Scalar(u64::MAX).to_value(),
            Value::String("18446744073709551615".to_owned())
        );
        assert_eq!(
            UInt64Scalar::parse(Value::String("18446744073709551615".to_owned()))
                .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))?,
            UInt64Scalar(u64::MAX)
        );
        Ok(())
    }
}
