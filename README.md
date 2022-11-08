# serde-value

`serde-cw-value` provides a way to capture serialization value trees for later
processing. Crate is designed to work with CosmWasm smart contracts, in
particular - it errors early when any floating-point types are processed. This
is for of [serde-value](https://github.com/arcnmx/serde-value).

# Usage

Add the library to the project with `cargo add`:

```
cargo add serde-cw-value
```

Then you can deserialize any serde source to intermediate
`serde_cw_value::Value` representing any structure deserializable by serde. For
example you can deserialize arbitrary Json without knowing it's exact
structure:

```rust
const json: &str = r#"{ "field": "value" }"#;

fn main() {
    let value = serde_json_wasm::from_str(json).unwrap();

    let Value::Map(m) = value else { unreachable!() };
    let v = m.get(&Value::String("field".to_owned())).unwrap();
}
```

This might be usefull for example when smart contract is forwarding a Json
received in the message, but not knowing it full structure. This will verify
that underlying data is a proper JSON, and also it allow you to figure out its
structure.

It is also possible to deserialize the `Value` further to a proper structure:

```rust
const json: &str = r#"{ "field": "value" }"#;

#[derive(Deserialize)]
struct Data {
    field: String,
}

fn main() {
    let value = serde_json_wasm::from_str(json).unwrap();
    let data: Data = value.deserialize_into();
}
```

It is always better to deserialize directly to final structure if possible, but
there are some cases, when deserialization has to be delayed. Also the `Value`
type is implementing `Clone` so it is possible to deserialize data to `Value`,
and then deserialize it to final type keeping the intermediate reprezentation
for future.

Finally, as `Value` also implements `Serialize` it is possible to transcribe one
format to another using this, without any knowledge of the internal data, by
first deserializing original format to `Value`, and then serializing it back
to the new format.
