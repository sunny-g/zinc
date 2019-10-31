//!
//! The semantic analyzer element.
//!

mod error;
mod place;
mod value;

pub use self::error::Error;
pub use self::place::Descriptor as PlaceDescriptor;
pub use self::place::Error as PlaceError;
pub use self::place::Place;
pub use self::value::Error as ValueError;
pub use self::value::Integer;
pub use self::value::IntegerError;
pub use self::value::Value;

use std::fmt;

use crate::syntax::TypeVariant;

#[derive(Clone, PartialEq)]
pub enum Element {
    Place(Place),
    Value(Value),
    Type(TypeVariant),
}

impl Element {
    pub fn or(self, other: Self) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("or", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("or", element)),
        };

        value_1.or(value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn xor(self, other: Self) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("xor", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("xor", element)),
        };

        value_1.xor(value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn and(self, other: Self) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("and", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("and", element)),
        };

        value_1.and(value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn equals(self, other: Self) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("equals", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("equals", element)),
        };

        value_1
            .equals(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn not_equals(self, other: Self) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("not_equals", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("not_equals", element)),
        };

        value_1
            .not_equals(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn greater_equals(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Value { .. } => {}
            element => return Err(Error::ExpectedValueExpression("greater_equals", element)),
        };

        match other {
            Self::Value { .. } => {}
            element => return Err(Error::ExpectedValueExpression("greater_equals", element)),
        };

        Ok(Self::Value(Value::Boolean))
    }

    pub fn lesser_equals(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Value { .. } => {}
            element => return Err(Error::ExpectedValueExpression("lesser_equals", element)),
        };

        match other {
            Self::Value { .. } => {}
            element => return Err(Error::ExpectedValueExpression("lesser_equals", element)),
        };

        Ok(Self::Value(Value::Boolean))
    }

    pub fn greater(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Value { .. } => {}
            element => return Err(Error::ExpectedValueExpression("greater", element)),
        };

        match other {
            Self::Value { .. } => {}
            element => return Err(Error::ExpectedValueExpression("greater", element)),
        };

        Ok(Self::Value(Value::Boolean))
    }

    pub fn lesser(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Value { .. } => {}
            element => return Err(Error::ExpectedValueExpression("lesser", element)),
        };

        match other {
            Self::Value { .. } => {}
            element => return Err(Error::ExpectedValueExpression("lesser", element)),
        };

        Ok(Self::Value(Value::Boolean))
    }

    pub fn add(self, other: Self) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("add", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("add", element)),
        };

        value_1.add(value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn subtract(self, other: Self) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("subtract", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("subtract", element)),
        };

        value_1
            .subtract(value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn multiply(self, other: Self) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("multiply", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("multiply", element)),
        };

        value_1
            .multiply(value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn divide(self, other: Self) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("divide", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("divide", element)),
        };

        value_1
            .divide(value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn modulo(self, other: Self) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("modulo", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("modulo", element)),
        };

        value_1
            .modulo(value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn negate(self) -> Result<Self, Error> {
        let value = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("negate", element)),
        };

        value.negate().map(Self::Value).map_err(Error::Value)
    }

    pub fn not(self) -> Result<Self, Error> {
        let value = match self {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("not", element)),
        };

        value.not().map(Self::Value).map_err(Error::Value)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Place(place) => write!(f, "{}", place),
            Self::Value(value) => write!(f, "{}", value),
            Self::Type(r#type) => write!(f, "{}", r#type),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
