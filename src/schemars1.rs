use std::borrow::Cow;

use crate::{Array, Slice, Vec};
use schemars1::{json_schema, JsonSchema, Schema, SchemaGenerator};

impl<T> JsonSchema for Vec<T>
where
    T: JsonSchema,
{
    fn schema_name() -> Cow<'static, str> {
        format!("NonEmpty_Array_of_{}", T::schema_name()).into()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "items": gen.subschema_for::<T>(),
            "minItems": 1,
        })
    }
}

impl<T> JsonSchema for Box<Slice<T>>
where
    T: JsonSchema,
{
    fn schema_name() -> Cow<'static, str> {
        Vec::<T>::schema_name()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        Vec::<T>::json_schema(gen)
    }
}

impl<T, const N: usize> JsonSchema for Array<T, N>
where
    T: JsonSchema,
{
    fn schema_name() -> Cow<'static, str> {
        format!("Array_of_{}_of_{}", N, T::schema_name()).into()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "items": gen.subschema_for::<T>(),
            "minItems": N,
            "maxItems": N,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use schemars1::schema_for;

    #[test]
    fn test_schema() {
        let schema = schema_for!(Vec<i32>);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        assert_eq!(
            serde_json::to_value(&schema).unwrap(),
            serde_json::json!({
              "$schema": "https://json-schema.org/draft/2020-12/schema",
              "title": "NonEmpty_Array_of_int32",
              "type": "array",
              "items": {
                "type": "integer",
                "format": "int32"
              },
              "minItems": 1
            })
        )
    }
}
