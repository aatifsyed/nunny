use crate::{Array, Slice, Vec};
use schemars08::{
    gen::SchemaGenerator,
    schema::{ArrayValidation, InstanceType, Schema, SchemaObject, SingleOrVec},
    JsonSchema,
};

impl<T> JsonSchema for Vec<T>
where
    T: JsonSchema,
{
    fn schema_name() -> String {
        format!("NonEmpty_Array_of_{}", T::schema_name())
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Array))),
            array: Some(Box::new(ArrayValidation {
                items: Some(SingleOrVec::Single(Box::new(gen.subschema_for::<T>()))),
                min_items: Some(1),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

impl<T> JsonSchema for Box<Slice<T>>
where
    T: JsonSchema,
{
    fn schema_name() -> String {
        Vec::<T>::schema_name()
    }

    fn json_schema(gen: &mut schemars08::gen::SchemaGenerator) -> Schema {
        Vec::<T>::json_schema(gen)
    }
}

impl<T, const N: usize> JsonSchema for Array<T, N>
where
    T: JsonSchema,
{
    fn schema_name() -> String {
        Vec::<T>::schema_name()
    }

    fn json_schema(gen: &mut schemars08::gen::SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Array))),
            array: Some(Box::new(ArrayValidation {
                items: Some(SingleOrVec::Single(Box::new(gen.subschema_for::<T>()))),
                min_items: Some(u32::try_from(N).unwrap_or(u32::MAX)),
                max_items: Some(u32::try_from(N).unwrap_or(u32::MAX)),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
