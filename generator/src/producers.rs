use schemars::JsonSchema;
use serde::Serialize;

/// Anything that makes items.
#[derive(Debug, Clone, Copy, Serialize, JsonSchema)]
#[allow(dead_code)]
pub enum Producer {
    AssemblingMachine,
    Smelter,
    OilRefinery,
    ChemicalPlant,
    Fractionator,

    ToDo,
}

impl Default for Producer {
    fn default() -> Self {
        Producer::ToDo
    }
}
