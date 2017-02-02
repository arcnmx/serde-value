#![doc(html_root_url="https://arcnmx.github.io/serde-value")]

extern crate serde;
extern crate ordered_float;

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::cmp::Ordering;
use serde::{de, Deserialize, Serialize, Serializer};
use serde::de::DeserializeSeed;
use ordered_float::OrderedFloat;

#[macro_use]
mod forward;

#[derive(Debug)]
pub enum Unexpected {
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Char(char),
    Str(String),
    Bytes(Vec<u8>),
    Unit,
    Option,
    NewtypeStruct,
    Seq,
    Map,
    Enum,
    UnitVariant,
    NewtypeVariant,
    TupleVariant,
    StructVariant,
    Other(String),
}

impl<'a> From<de::Unexpected<'a>> for Unexpected {
    fn from(unexp: de::Unexpected) -> Unexpected {
        match unexp {
            de::Unexpected::Bool(v) => Unexpected::Bool(v),
            de::Unexpected::Unsigned(v) => Unexpected::Unsigned(v),
            de::Unexpected::Signed(v) => Unexpected::Signed(v),
            de::Unexpected::Float(v) => Unexpected::Float(v),
            de::Unexpected::Char(v) => Unexpected::Char(v),
            de::Unexpected::Str(v) => Unexpected::Str(v.to_owned()),
            de::Unexpected::Bytes(v) => Unexpected::Bytes(v.to_owned()),
            de::Unexpected::Unit => Unexpected::Unit,
            de::Unexpected::Option => Unexpected::Option,
            de::Unexpected::NewtypeStruct => Unexpected::NewtypeStruct,
            de::Unexpected::Seq => Unexpected::Seq,
            de::Unexpected::Map => Unexpected::Map,
            de::Unexpected::Enum => Unexpected::Enum,
            de::Unexpected::UnitVariant => Unexpected::UnitVariant,
            de::Unexpected::NewtypeVariant => Unexpected::NewtypeVariant,
            de::Unexpected::TupleVariant => Unexpected::TupleVariant,
            de::Unexpected::StructVariant => Unexpected::StructVariant,
            de::Unexpected::Other(v) => Unexpected::Other(v.to_owned()),
        }
    }
}

impl Unexpected {
    pub fn to_unexpected<'a>(&'a self) -> de::Unexpected<'a> {
        match *self {
            Unexpected::Bool(v) => de::Unexpected::Bool(v),
            Unexpected::Unsigned(v) => de::Unexpected::Unsigned(v),
            Unexpected::Signed(v) => de::Unexpected::Signed(v),
            Unexpected::Float(v) => de::Unexpected::Float(v),
            Unexpected::Char(v) => de::Unexpected::Char(v),
            Unexpected::Str(ref v) => de::Unexpected::Str(v),
            Unexpected::Bytes(ref v) => de::Unexpected::Bytes(v),
            Unexpected::Unit => de::Unexpected::Unit,
            Unexpected::Option => de::Unexpected::Option,
            Unexpected::NewtypeStruct => de::Unexpected::NewtypeStruct,
            Unexpected::Seq => de::Unexpected::Seq,
            Unexpected::Map => de::Unexpected::Map,
            Unexpected::Enum => de::Unexpected::Enum,
            Unexpected::UnitVariant => de::Unexpected::UnitVariant,
            Unexpected::NewtypeVariant => de::Unexpected::NewtypeVariant,
            Unexpected::TupleVariant => de::Unexpected::TupleVariant,
            Unexpected::StructVariant => de::Unexpected::StructVariant,
            Unexpected::Other(ref v) => de::Unexpected::Other(v),
        }
    }

}

#[derive(Debug)]
pub enum DeserializerError {
    Custom(String),
    InvalidType(Unexpected, String),
    InvalidValue(Unexpected, String),
    InvalidLength(usize, String),
    UnknownVariant(String, &'static [&'static str]),
    UnknownField(String, &'static [&'static str]),
    MissingField(&'static str),
    DuplicateField(&'static str),
}

impl de::Error for DeserializerError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        DeserializerError::Custom(msg.to_string())
    }

    fn invalid_type(unexp: de::Unexpected, exp: &de::Expected) -> Self {
        DeserializerError::InvalidType(unexp.into(), exp.to_string())
    }

    fn invalid_value(unexp: de::Unexpected, exp: &de::Expected) -> Self {
        DeserializerError::InvalidValue(unexp.into(), exp.to_string())
    }

    fn invalid_length(len: usize, exp: &de::Expected) -> Self {
        DeserializerError::InvalidLength(len, exp.to_string())
    }

    fn unknown_variant(field: &str, expected: &'static [&'static str]) -> Self {
        DeserializerError::UnknownVariant(field.into(), expected)
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        DeserializerError::UnknownField(field.into(), expected)
    }

    fn missing_field(field: &'static str) -> Self {
        DeserializerError::MissingField(field)
    }

    fn duplicate_field(field: &'static str) -> Self {
        DeserializerError::DuplicateField(field)
    }
}

impl DeserializerError {
    pub fn to_error<E: de::Error>(&self) -> E {
        match *self {
            DeserializerError::Custom(ref msg) => E::custom(msg.clone()),
            DeserializerError::InvalidType(ref unexp, ref exp) => {
                E::invalid_type(unexp.to_unexpected(), &&**exp)
            }
            DeserializerError::InvalidValue(ref unexp, ref exp) => {
                E::invalid_value(unexp.to_unexpected(), &&**exp)
            }
            DeserializerError::InvalidLength(len, ref exp) => E::invalid_length(len, &&**exp),
            DeserializerError::UnknownVariant(ref field, exp) => E::unknown_variant(field, exp),
            DeserializerError::UnknownField(ref field, exp) => E::unknown_field(field, exp),
            DeserializerError::MissingField(field) => E::missing_field(field),
            DeserializerError::DuplicateField(field) => E::missing_field(field),
        }
    }

    pub fn into_error<E: de::Error>(self) -> E {
        self.to_error()
    }
}

impl Error for DeserializerError {
    fn description(&self) -> &str {
        "Value deserializer error"
    }
}

impl fmt::Display for DeserializerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserializerError::Custom(ref msg) => write!(f, "{}", msg),
            DeserializerError::InvalidType(ref unexp, ref exp) => {
                write!(f, "Invalid type {}. Expected {}", unexp.to_unexpected(), exp)
            }
            DeserializerError::InvalidValue(ref unexp, ref exp) => {
                write!(f, "Invalid value {}. Expected {}", unexp.to_unexpected(), exp)
            }
            DeserializerError::InvalidLength(len, ref exp) => {
                write!(f, "Invalid length {}. Expected {}", len, exp)
            }
            DeserializerError::UnknownVariant(ref field, exp) => {
                write!(f, "Unknown variant {}. Expected one of {}", field, exp.join(", "))
            },
            DeserializerError::UnknownField(ref field, exp) => {
                write!(f, "Unknown field {}. Expected one of {}", field, exp.join(", "))
            }
            DeserializerError::MissingField(field) => write!(f, "Missing field {}", field),
            DeserializerError::DuplicateField(field) => write!(f, "Duplicate field {}", field),
        }
    }
}

impl From<de::value::Error> for DeserializerError {
    fn from(e: de::value::Error) -> DeserializerError {
        DeserializerError::Custom(e.to_string())
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    Char(char),
    String(String),

    Unit,
    Option(Option<Box<Value>>),
    Newtype(Box<Value>),
    Seq(Vec<Value>),
    Map(BTreeMap<Value, Value>),
    Bytes(Vec<u8>),
}

impl PartialEq for Value {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&Value::Bool(v0), &Value::Bool(v1)) if v0 == v1 => true,
            (&Value::U8(v0), &Value::U8(v1)) if v0 == v1 => true,
            (&Value::U16(v0), &Value::U16(v1)) if v0 == v1 => true,
            (&Value::U32(v0), &Value::U32(v1)) if v0 == v1 => true,
            (&Value::U64(v0), &Value::U64(v1)) if v0 == v1 => true,
            (&Value::I8(v0), &Value::I8(v1)) if v0 == v1 => true,
            (&Value::I16(v0), &Value::I16(v1)) if v0 == v1 => true,
            (&Value::I32(v0), &Value::I32(v1)) if v0 == v1 => true,
            (&Value::I64(v0), &Value::I64(v1)) if v0 == v1 => true,
            (&Value::F32(v0), &Value::F32(v1)) if OrderedFloat(v0) == OrderedFloat(v1) => true,
            (&Value::F64(v0), &Value::F64(v1)) if OrderedFloat(v0) == OrderedFloat(v1) => true,
            (&Value::Char(v0), &Value::Char(v1)) if v0 == v1 => true,
            (&Value::String(ref v0), &Value::String(ref v1)) if v0 == v1 => true,
            (&Value::Unit, &Value::Unit) => true,
            (&Value::Option(ref v0), &Value::Option(ref v1)) if v0 == v1 => true,
            (&Value::Newtype(ref v0), &Value::Newtype(ref v1)) if v0 == v1 => true,
            (&Value::Seq(ref v0), &Value::Seq(ref v1)) if v0 == v1 => true,
            (&Value::Map(ref v0), &Value::Map(ref v1)) if v0 == v1 => true,
            (&Value::Bytes(ref v0), &Value::Bytes(ref v1)) if v0 == v1 => true,
            _ => false,
        }
    }
}

impl Ord for Value {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (&Value::Bool(v0), &Value::Bool(ref v1)) => v0.cmp(v1),
            (&Value::U8(v0), &Value::U8(ref v1)) => v0.cmp(v1),
            (&Value::U16(v0), &Value::U16(ref v1)) => v0.cmp(v1),
            (&Value::U32(v0), &Value::U32(ref v1)) => v0.cmp(v1),
            (&Value::U64(v0), &Value::U64(ref v1)) => v0.cmp(v1),
            (&Value::I8(v0), &Value::I8(ref v1)) => v0.cmp(v1),
            (&Value::I16(v0), &Value::I16(ref v1)) => v0.cmp(v1),
            (&Value::I32(v0), &Value::I32(ref v1)) => v0.cmp(v1),
            (&Value::I64(v0), &Value::I64(ref v1)) => v0.cmp(v1),
            (&Value::F32(v0), &Value::F32(v1)) => OrderedFloat(v0).cmp(&OrderedFloat(v1)),
            (&Value::F64(v0), &Value::F64(v1)) => OrderedFloat(v0).cmp(&OrderedFloat(v1)),
            (&Value::Char(v0), &Value::Char(ref v1)) => v0.cmp(v1),
            (&Value::String(ref v0), &Value::String(ref v1)) => v0.cmp(v1),
            (&Value::Unit, &Value::Unit) => Ordering::Equal,
            (&Value::Option(ref v0), &Value::Option(ref v1)) => v0.cmp(v1),
            (&Value::Newtype(ref v0), &Value::Newtype(ref v1)) => v0.cmp(v1),
            (&Value::Seq(ref v0), &Value::Seq(ref v1)) => v0.cmp(v1),
            (&Value::Map(ref v0), &Value::Map(ref v1)) => v0.cmp(v1),
            (&Value::Bytes(ref v0), &Value::Bytes(ref v1)) => v0.cmp(v1),
            (ref v0, ref v1) => v0.discriminant().cmp(&v1.discriminant()),
        }
    }
}

impl Value {
    fn discriminant(&self) -> usize {
        match *self {
            Value::Bool(..) => 0,
            Value::U8(..) => 1,
            Value::U16(..) => 2,
            Value::U32(..) => 3,
            Value::U64(..) => 4,
            Value::I8(..) => 5,
            Value::I16(..) => 6,
            Value::I32(..) => 7,
            Value::I64(..) => 8,
            Value::F32(..) => 9,
            Value::F64(..) => 10,
            Value::Char(..) => 11,
            Value::String(..) => 12,
            Value::Unit => 13,
            Value::Option(..) => 14,
            Value::Newtype(..) => 15,
            Value::Seq(..) => 16,
            Value::Map(..) => 17,
            Value::Bytes(..) => 18,
        }
    }

    pub fn deserialize_into<T: Deserialize>(self) -> Result<T, DeserializerError> {
        T::deserialize(self)
    }
}

impl Eq for Value { }
impl PartialOrd for Value {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

pub struct ValueVisitor;

impl de::Visitor for ValueVisitor {
    type Value = Value;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("any value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i8<E>(self, value: i8) -> Result<Value, E> {
        Ok(Value::I8(value))
    }

    fn visit_i16<E>(self, value: i16) -> Result<Value, E> {
        Ok(Value::I16(value))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Value, E> {
        Ok(Value::I32(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::I64(value))
    }

    fn visit_u8<E>(self, value: u8) -> Result<Value, E> {
        Ok(Value::U8(value))
    }

    fn visit_u16<E>(self, value: u16) -> Result<Value, E> {
        Ok(Value::U16(value))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Value, E> {
        Ok(Value::U32(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
        Ok(Value::U64(value))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Value, E> {
        Ok(Value::F32(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
        Ok(Value::F64(value))
    }

    fn visit_char<E>(self, value: char) -> Result<Value, E> {
        Ok(Value::Char(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.into()))
    }

    fn visit_string<E>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }

    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Unit)
    }

    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Option(None))
    }

    fn visit_some<D: de::Deserializer>(self, d: D) -> Result<Value, D::Error> {
        d.deserialize(ValueVisitor).map(|v| Value::Option(Some(Box::new(v))))
    }

    fn visit_newtype_struct<D: de::Deserializer>(self, d: D) -> Result<Value, D::Error> {
        d.deserialize(ValueVisitor).map(|v| Value::Newtype(Box::new(v)))
    }

    fn visit_seq<V: de::SeqVisitor>(self, visitor: V) -> Result<Value, V::Error> {
        let values = try!(de::impls::VecVisitor::new().visit_seq(visitor));
        Ok(Value::Seq(values))
    }

    fn visit_map<V: de::MapVisitor>(self, visitor: V) -> Result<Value, V::Error> {
        let v = de::impls::BTreeMapVisitor::new();
        let values = try!(v.visit_map(visitor));
        Ok(Value::Map(values))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Value, E> {
        Ok(Value::Bytes(v.into()))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Value, E> {
        Ok(Value::Bytes(v))
    }
}


impl Deserialize for Value {
    fn deserialize<D: de::Deserializer>(d: D) -> Result<Self, D::Error> {
        d.deserialize(ValueVisitor)
    }
}

impl Serialize for Value {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            &Value::Bool(v) => s.serialize_bool(v),
            &Value::U8(v) => s.serialize_u8(v),
            &Value::U16(v) => s.serialize_u16(v),
            &Value::U32(v) => s.serialize_u32(v),
            &Value::U64(v) => s.serialize_u64(v),
            &Value::I8(v) => s.serialize_i8(v),
            &Value::I16(v) => s.serialize_i16(v),
            &Value::I32(v) => s.serialize_i32(v),
            &Value::I64(v) => s.serialize_i64(v),
            &Value::F32(v) => s.serialize_f32(v),
            &Value::F64(v) => s.serialize_f64(v),
            &Value::Char(v) => s.serialize_char(v),
            &Value::String(ref v) => s.serialize_str(v),
            &Value::Unit => s.serialize_unit(),
            &Value::Option(None) => s.serialize_none(),
            &Value::Option(Some(ref v)) => s.serialize_some(v),
            &Value::Newtype(ref v) => s.serialize_newtype_struct("", v),
            &Value::Seq(ref v) => v.serialize(s),
            &Value::Map(ref v) => v.serialize(s),
            &Value::Bytes(ref v) => s.serialize_bytes(v),
        }
    }
}

impl de::value::ValueDeserializer<DeserializerError> for Value {
    type Deserializer = Value;

    fn into_deserializer(self) -> Value {
        self
    }
}

struct MapVisitor<I> {
    iter: I,
    value: Option<Value>,
}

impl<I: Iterator<Item=(Value, Value)>> de::MapVisitor for MapVisitor<I> {
    type Error = DeserializerError;

    fn visit_key_seed<K: DeserializeSeed>(&mut self, seed: K)
        -> Result<Option<K::Value>, Self::Error>
    {
        match self.iter.next() {
            Some((k, v)) => {
                self.value = Some(v);
                seed.deserialize(k).map(Some)
            }
            None => Ok(None),
        }
    }

    fn visit_value_seed<V: DeserializeSeed>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
        match self.value.take() {
            Some(v) => seed.deserialize(v),
            None => Err(de::Error::custom("value is missing")),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl de::Deserializer for Value {
    type Error = DeserializerError;

    fn deserialize<V: de::Visitor>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self {
            Value::Bool(v) => visitor.visit_bool(v),
            Value::U8(v) => visitor.visit_u8(v),
            Value::U16(v) => visitor.visit_u16(v),
            Value::U32(v) => visitor.visit_u32(v),
            Value::U64(v) => visitor.visit_u64(v),
            Value::I8(v) => visitor.visit_i8(v),
            Value::I16(v) => visitor.visit_i16(v),
            Value::I32(v) => visitor.visit_i32(v),
            Value::I64(v) => visitor.visit_i64(v),
            Value::F32(v) => visitor.visit_f32(v),
            Value::F64(v) => visitor.visit_f64(v),
            Value::Char(v) => visitor.visit_char(v),
            Value::String(v) => visitor.visit_string(v),
            Value::Unit => visitor.visit_unit(),
            Value::Option(None) => visitor.visit_none(),
            Value::Option(Some(v)) => visitor.visit_some(*v),
            Value::Newtype(v) => visitor.visit_newtype_struct(*v),
            Value::Seq(v) => {
                visitor.visit_seq(de::value::SeqDeserializer::new(v.into_iter())).map_err(From::from)
            },
            Value::Map(v) => {
                visitor.visit_map(MapVisitor {
                    iter: v.into_iter(),
                    value: None,
                })
            },
            Value::Bytes(v) => visitor.visit_byte_buf(v),
        }
    }

    fn deserialize_option<V: de::Visitor>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self {
            Value::Option(..) => self.deserialize(visitor),
            Value::Unit => visitor.visit_none(),
            _ => visitor.visit_some(self)
        }
    }

    forward_to_deserialize!{
                deserialize_bool();
                deserialize_u8();
                deserialize_u16();
                deserialize_u32();
                deserialize_u64();
                deserialize_i8();
                deserialize_i16();
                deserialize_i32();
                deserialize_i64();
                deserialize_f32();
                deserialize_f64();
                deserialize_char();
                deserialize_str();
                deserialize_string();
                deserialize_unit();
                deserialize_seq();
                deserialize_seq_fixed_size(len: usize);
                deserialize_bytes();
                deserialize_byte_buf();
                deserialize_map();
                deserialize_unit_struct(name: &'static str);
                deserialize_newtype_struct(name: &'static str);
                deserialize_tuple_struct(name: &'static str, len: usize);
                deserialize_struct(name: &'static str, fields: &'static [&'static str]);
                deserialize_struct_field();
                deserialize_tuple(len: usize);
                deserialize_enum(name: &'static str, variants: &'static [&'static str]);
                deserialize_ignored_any();
            }
}
