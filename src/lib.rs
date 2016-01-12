#![doc(html_root_url="https://arcnmx.github.io/serde-value")]

extern crate serde;
extern crate ordered_float;

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::cmp::Ordering;
use serde::{de, Deserialize, Serialize, Serializer};
use ordered_float::OrderedFloat;

#[derive(Debug)]
pub enum DeserializerError {
    Syntax(String),
    EndOfStream,
    UnknownField(String),
    MissingField(&'static str),
}

impl de::Error for DeserializerError {
    fn syntax(msg: &str) -> Self {
        DeserializerError::Syntax(msg.into())
    }

    fn end_of_stream() -> Self {
        DeserializerError::EndOfStream
    }

    fn unknown_field(field: &str) -> Self {
        DeserializerError::UnknownField(field.into())
    }

    fn missing_field(field: &'static str) -> Self {
        DeserializerError::MissingField(field)
    }
}

impl DeserializerError {
    pub fn to_error<E: de::Error>(&self) -> E {
        match *self {
            DeserializerError::Syntax(ref msg) => E::syntax(msg),
            DeserializerError::EndOfStream=> E::end_of_stream(),
            DeserializerError::UnknownField(ref field) => E::unknown_field(field),
            DeserializerError::MissingField(field) => E::missing_field(field),
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
            DeserializerError::Syntax(ref msg) => write!(f, "Syntax error: {}", msg),
            DeserializerError::EndOfStream => write!(f, "End of stream"),
            DeserializerError::UnknownField(ref field) => write!(f, "Unknown field: {}", field),
            DeserializerError::MissingField(field) => write!(f, "Missing field: {}", field),
        }
    }
}

impl From<de::value::Error> for DeserializerError {
    fn from(e: de::value::Error) -> Self {
        match e {
            de::value::Error::SyntaxError => DeserializerError::Syntax(String::new()),
            de::value::Error::EndOfStreamError => DeserializerError::EndOfStream,
            de::value::Error::UnknownFieldError(field) => DeserializerError::UnknownField(field),
            de::value::Error::MissingFieldError(field) => DeserializerError::MissingField(field),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),

    Usize(usize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    Isize(isize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    Char(char),
    String(String),

    Unit,
    UnitStruct(&'static str),
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
            (&Value::Usize(v0), &Value::Usize(v1)) if v0 == v1 => true,
            (&Value::U8(v0), &Value::U8(v1)) if v0 == v1 => true,
            (&Value::U16(v0), &Value::U16(v1)) if v0 == v1 => true,
            (&Value::U32(v0), &Value::U32(v1)) if v0 == v1 => true,
            (&Value::U64(v0), &Value::U64(v1)) if v0 == v1 => true,
            (&Value::Isize(v0), &Value::Isize(v1)) if v0 == v1 => true,
            (&Value::I8(v0), &Value::I8(v1)) if v0 == v1 => true,
            (&Value::I16(v0), &Value::I16(v1)) if v0 == v1 => true,
            (&Value::I32(v0), &Value::I32(v1)) if v0 == v1 => true,
            (&Value::I64(v0), &Value::I64(v1)) if v0 == v1 => true,
            (&Value::F32(v0), &Value::F32(v1)) if OrderedFloat(v0) == OrderedFloat(v1) => true,
            (&Value::F64(v0), &Value::F64(v1)) if OrderedFloat(v0) == OrderedFloat(v1) => true,
            (&Value::Char(v0), &Value::Char(v1)) if v0 == v1 => true,
            (&Value::String(ref v0), &Value::String(ref v1)) if v0 == v1 => true,
            (&Value::Unit, &Value::Unit) => true,
            (&Value::UnitStruct(v0), &Value::UnitStruct(v1)) if v0 == v1 => true,
            (&Value::Option(ref v0), &Value::Option(ref v1)) if v0 == v1 => true,
            (&Value::Newtype(ref v0), &Value::Newtype(ref v1)) if v0 == v1 => true,
            (&Value::Seq(ref v0), &Value::Seq(ref v1)) if v0 == v1 => true,
            (&Value::Map(ref v0), &Value::Map(ref v1)) if v0 == v1 => true,
            (&Value::Bytes(ref v0), &Value::Bytes(ref v1)) if v0 == v1 => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match (self, rhs) {
            (&Value::Bool(v0), &Value::Bool(ref v1)) => v0.partial_cmp(v1),
            (&Value::Usize(v0), &Value::Usize(ref v1)) => v0.partial_cmp(v1),
            (&Value::U8(v0), &Value::U8(ref v1)) => v0.partial_cmp(v1),
            (&Value::U16(v0), &Value::U16(ref v1)) => v0.partial_cmp(v1),
            (&Value::U32(v0), &Value::U32(ref v1)) => v0.partial_cmp(v1),
            (&Value::U64(v0), &Value::U64(ref v1)) => v0.partial_cmp(v1),
            (&Value::Isize(v0), &Value::Isize(ref v1)) => v0.partial_cmp(v1),
            (&Value::I8(v0), &Value::I8(ref v1)) => v0.partial_cmp(v1),
            (&Value::I16(v0), &Value::I16(ref v1)) => v0.partial_cmp(v1),
            (&Value::I32(v0), &Value::I32(ref v1)) => v0.partial_cmp(v1),
            (&Value::I64(v0), &Value::I64(ref v1)) => v0.partial_cmp(v1),
            (&Value::F32(v0), &Value::F32(v1)) => Some(OrderedFloat(v0).cmp(&OrderedFloat(v1))),
            (&Value::F64(v0), &Value::F64(v1)) => Some(OrderedFloat(v0).cmp(&OrderedFloat(v1))),
            (&Value::Char(v0), &Value::Char(ref v1)) => v0.partial_cmp(v1),
            (&Value::String(ref v0), &Value::String(ref v1)) => v0.partial_cmp(v1),
            (&Value::Unit, &Value::Unit) => Some(Ordering::Equal),
            (&Value::UnitStruct(v0), &Value::UnitStruct(v1)) => v0.partial_cmp(v1),
            (&Value::Option(ref v0), &Value::Option(ref v1)) => v0.partial_cmp(v1),
            (&Value::Newtype(ref v0), &Value::Newtype(ref v1)) => v0.partial_cmp(v1),
            (&Value::Seq(ref v0), &Value::Seq(ref v1)) => v0.partial_cmp(v1),
            (&Value::Map(ref v0), &Value::Map(ref v1)) => v0.partial_cmp(v1),
            (&Value::Bytes(ref v0), &Value::Bytes(ref v1)) => v0.partial_cmp(v1),
            (ref v0, ref v1) => v0.discriminant().partial_cmp(&v1.discriminant()),
        }
    }
}

impl Value {
    fn discriminant(&self) -> usize {
        match *self {
            Value::Bool(..) => 0,
            Value::Usize(..) => 1,
            Value::U8(..) => 2,
            Value::U16(..) => 3,
            Value::U32(..) => 4,
            Value::U64(..) => 5,
            Value::Isize(..) => 6,
            Value::I8(..) => 7,
            Value::I16(..) => 8,
            Value::I32(..) => 9,
            Value::I64(..) => 10,
            Value::F32(..) => 11,
            Value::F64(..) => 12,
            Value::Char(..) => 13,
            Value::String(..) => 14,
            Value::Unit => 15,
            Value::UnitStruct(..) => 16,
            Value::Option(..) => 17,
            Value::Newtype(..) => 18,
            Value::Seq(..) => 19,
            Value::Map(..) => 20,
            Value::Bytes(..) => 21,
        }
    }

    pub fn deserializer(self) -> Deserializer {
        Deserializer::new(self)
    }

    pub fn deserialize_into<T: Deserialize>(self) -> Result<T, DeserializerError> {
        T::deserialize(&mut self.deserializer())
    }
}

impl Eq for Value { }
impl Ord for Value {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.partial_cmp(rhs).expect("total ordering")
    }
}

pub struct ValueVisitor;

impl de::Visitor for ValueVisitor {
    type Value = Value;

    fn visit_bool<E>(&mut self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_usize<E>(&mut self, value: usize) -> Result<Value, E> {
        Ok(Value::Usize(value))
    }

    fn visit_isize<E>(&mut self, value: isize) -> Result<Value, E> {
        Ok(Value::Isize(value))
    }

    fn visit_i8<E>(&mut self, value: i8) -> Result<Value, E> {
        Ok(Value::I8(value))
    }

    fn visit_i16<E>(&mut self, value: i16) -> Result<Value, E> {
        Ok(Value::I16(value))
    }

    fn visit_i32<E>(&mut self, value: i32) -> Result<Value, E> {
        Ok(Value::I32(value))
    }

    fn visit_i64<E>(&mut self, value: i64) -> Result<Value, E> {
        Ok(Value::I64(value))
    }

    fn visit_u8<E>(&mut self, value: u8) -> Result<Value, E> {
        Ok(Value::U8(value))
    }

    fn visit_u16<E>(&mut self, value: u16) -> Result<Value, E> {
        Ok(Value::U16(value))
    }

    fn visit_u32<E>(&mut self, value: u32) -> Result<Value, E> {
        Ok(Value::U32(value))
    }

    fn visit_u64<E>(&mut self, value: u64) -> Result<Value, E> {
        Ok(Value::U64(value))
    }

    fn visit_f32<E>(&mut self, value: f32) -> Result<Value, E> {
        Ok(Value::F32(value))
    }

    fn visit_f64<E>(&mut self, value: f64) -> Result<Value, E> {
        Ok(Value::F64(value))
    }

    fn visit_char<E>(&mut self, value: char) -> Result<Value, E> {
        Ok(Value::Char(value))
    }

    fn visit_str<E>(&mut self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.into()))
    }

    fn visit_string<E>(&mut self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }

    fn visit_unit<E>(&mut self) -> Result<Value, E> {
        Ok(Value::Unit)
    }

    fn visit_unit_struct<E>(&mut self, name: &'static str) -> Result<Value, E> {
        Ok(Value::UnitStruct(name))
    }

    fn visit_none<E>(&mut self) -> Result<Value, E> {
        Ok(Value::Option(None))
    }

    fn visit_some<D: de::Deserializer>(&mut self, d: &mut D) -> Result<Value, D::Error> {
        d.visit(ValueVisitor).map(|v| Value::Option(Some(Box::new(v))))
    }

    fn visit_newtype_struct<D: de::Deserializer>(&mut self, d: &mut D) -> Result<Value, D::Error> {
        d.visit(ValueVisitor).map(|v| Value::Newtype(Box::new(v)))
    }

    fn visit_seq<V: de::SeqVisitor>(&mut self, visitor: V) -> Result<Value, V::Error> {
        let values = try!(de::impls::VecVisitor::new().visit_seq(visitor));
        Ok(Value::Seq(values))
    }

    fn visit_map<V: de::MapVisitor>(&mut self, visitor: V) -> Result<Value, V::Error> {
        let mut v = de::impls::BTreeMapVisitor::new();
        let values = try!(v.visit_map(visitor));
        Ok(Value::Map(values))
    }

    fn visit_bytes<E>(&mut self, v: &[u8]) -> Result<Value, E> {
        Ok(Value::Bytes(v.into()))
    }

    fn visit_byte_buf<E>(&mut self, v: Vec<u8>) -> Result<Value, E> {
        Ok(Value::Bytes(v))
    }
}


impl Deserialize for Value {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<Self, D::Error> {
        d.visit(ValueVisitor)
    }
}

impl Serialize for Value {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        match self {
            &Value::Bool(v) => s.visit_bool(v),
            &Value::Usize(v) => s.visit_usize(v),
            &Value::U8(v) => s.visit_u8(v),
            &Value::U16(v) => s.visit_u16(v),
            &Value::U32(v) => s.visit_u32(v),
            &Value::U64(v) => s.visit_u64(v),
            &Value::Isize(v) => s.visit_isize(v),
            &Value::I8(v) => s.visit_i8(v),
            &Value::I16(v) => s.visit_i16(v),
            &Value::I32(v) => s.visit_i32(v),
            &Value::I64(v) => s.visit_i64(v),
            &Value::F32(v) => s.visit_f32(v),
            &Value::F64(v) => s.visit_f64(v),
            &Value::Char(v) => s.visit_char(v),
            &Value::String(ref v) => s.visit_str(v),
            &Value::Unit => s.visit_unit(),
            &Value::UnitStruct(name) => s.visit_unit_struct(name),
            &Value::Option(None) => s.visit_none(),
            &Value::Option(Some(ref v)) => s.visit_some(v),
            &Value::Newtype(ref v) => s.visit_newtype_struct("", v),
            &Value::Seq(ref v) => v.serialize(s),
            &Value::Map(ref v) => v.serialize(s),
            &Value::Bytes(ref v) => s.visit_bytes(v),
        }
    }
}

pub struct Deserializer {
    pub value: Option<Value>,
}

impl Deserializer {
    pub fn new(value: Value) -> Self {
        Deserializer {
            value: Some(value),
        }
    }
}

impl de::value::ValueDeserializer for Value {
    type Deserializer = ValueDeserializer;

    fn into_deserializer(self) -> Self::Deserializer {
        ValueDeserializer(Deserializer::new(self))
    }
}

struct MapVisitor<'a, I> {
    iter: I,
    de: &'a mut Deserializer,
    value: Option<Value>,
}

impl<'a, I: Iterator<Item=(Value, Value)>> de::MapVisitor for MapVisitor<'a, I> {
    type Error = DeserializerError;

    fn visit_key<K: Deserialize>(&mut self) -> Result<Option<K>, Self::Error> {
        while let Some((k, v)) = self.iter.next() {
            let mut de = Deserializer::new(k.clone());
            return match Deserialize::deserialize(&mut de) {
                Ok(k) => {
                    self.value = Some(v);
                    Ok(Some(k))
                },
                Err(DeserializerError::UnknownField(..)) => {
                    self.de.borrow_map().insert(k, v);
                    continue
                },
                Err(e) => Err(e),
            };
        }

        Ok(None)
    }

    fn visit_value<V: Deserialize>(&mut self) -> Result<V, Self::Error> {
        match self.value.take() {
            Some(v) => V::deserialize(&mut Deserializer::new(v)),
            None => Err(de::Error::end_of_stream())
        }
    }

    fn missing_field<V: de::Deserialize>(&mut self, _field: &'static str) -> Result<V, Self::Error> {
        //let mut de = de::value::ValueDeserializer::into_deserializer(());
        let mut de = Deserializer::new(Value::Unit);
        de::Deserialize::deserialize(&mut de)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        self.de.borrow_map().extend(&mut self.iter);
        Ok(())
    }
}

impl Deserializer {
    fn borrow_map(&mut self) -> &mut BTreeMap<Value, Value> {
        self.value = self.value.take().or(Some(Value::Map(BTreeMap::new())));
        match *self.value.as_mut().unwrap() {
            Value::Map(ref mut map) => map,
            _ => unreachable!(),
        }
    }
}

impl de::Deserializer for Deserializer {
    type Error = DeserializerError;

    fn visit<V: de::Visitor>(&mut self, mut visitor: V) -> Result<V::Value, Self::Error> {
        if let Some(value) = self.value.take() {
            match value {
                Value::Bool(v) => visitor.visit_bool(v),
                Value::Usize(v) => visitor.visit_usize(v),
                Value::U8(v) => visitor.visit_u8(v),
                Value::U16(v) => visitor.visit_u16(v),
                Value::U32(v) => visitor.visit_u32(v),
                Value::U64(v) => visitor.visit_u64(v),
                Value::Isize(v) => visitor.visit_isize(v),
                Value::I8(v) => visitor.visit_i8(v),
                Value::I16(v) => visitor.visit_i16(v),
                Value::I32(v) => visitor.visit_i32(v),
                Value::I64(v) => visitor.visit_i64(v),
                Value::F32(v) => visitor.visit_f32(v),
                Value::F64(v) => visitor.visit_f64(v),
                Value::Char(v) => visitor.visit_char(v),
                Value::String(v) => visitor.visit_string(v),
                Value::Unit => visitor.visit_unit(),
                Value::UnitStruct(name) => visitor.visit_unit_struct(name),
                Value::Option(None) => visitor.visit_none(),
                Value::Option(Some(v)) => visitor.visit_some(&mut Deserializer::new(*v)),
                Value::Newtype(v) => visitor.visit_newtype_struct(&mut Deserializer::new(*v)),
                Value::Seq(v) => {
                    let len = v.len();
                    visitor.visit_seq(de::value::SeqDeserializer::new(v.into_iter(), len)).map_err(From::from)
                },
                Value::Map(v) => {
                    visitor.visit_map(MapVisitor {
                        iter: v.into_iter(),
                        de: self,
                        value: None,
                    })
                },
                Value::Bytes(v) => visitor.visit_byte_buf(v),
            }
        } else {
            Err(de::Error::end_of_stream())
        }
    }
}

pub struct ValueDeserializer(Deserializer);

impl de::Deserializer for ValueDeserializer {
    type Error = de::value::Error;

    fn visit<V: de::Visitor>(&mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.visit(visitor).map_err(DeserializerError::into_error)
    }
}
