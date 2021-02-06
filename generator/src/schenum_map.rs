use enum_map::{Enum, EnumMap};
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, ObjectValidation, Schema, SchemaObject},
    JsonSchema,
};
use serde::Serialize;

use std::fmt::Debug;

/// An EnumMap that can be schemars-ed
#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchemingEnumMap<K: enum_map::Enum<V>, V>(pub EnumMap<K, V>);

impl<K: enum_map::Enum<V>, V> From<EnumMap<K, V>> for SchemingEnumMap<K, V> {
    fn from(it: EnumMap<K, V>) -> Self {
        Self(it)
    }
}

impl<K, V> JsonSchema for SchemingEnumMap<K, V>
where
    K: Enum<V> + Enum<Schema> + Enum<()> + JsonSchema + Debug,
    V: JsonSchema,
{
    fn schema_name() -> String {
        format!("EnumMap_of_{}_to_{}", K::schema_name(), V::schema_name())
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let subschema = gen.subschema_for::<V>();
        SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                properties: EnumMap::from(|_: K| subschema.clone())
                    .into_iter()
                    .map(|(k, schema)| (format!("{:?}", k), schema))
                    .collect(),
                required: EnumMap::from(|_: K| ())
                    .into_iter()
                    .map(|(k, _)| format!("{:?}", k))
                    .collect(),
                additional_properties: Some(Box::new(Schema::Bool(false))),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}
