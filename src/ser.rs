use serde::ser::{Serialize, Serializer};

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
