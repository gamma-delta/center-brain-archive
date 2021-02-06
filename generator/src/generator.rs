use crate::{
    items::{Item, ItemStack},
    producers::Producer,
    recipes::Recipe,
    schenum_map::SchemingEnumMap,
    technologies::Technology,
};

use enum_map::EnumMap;
use schemars::JsonSchema;
use serde::Serialize;
use strum::IntoEnumIterator;

use std::fmt::Debug;

#[derive(Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AllDSPInfo {
    pub tech_tree: SchemingEnumMap<Technology, TechnologyEntry>,
    pub recipes: SchemingEnumMap<Recipe, RecipeEntry>,
    pub production_methods: SchemingEnumMap<Item, Vec<Recipe>>,
    pub consumption_methods: SchemingEnumMap<Item, Vec<Recipe>>,
}

impl AllDSPInfo {
    /// Generate all the information!
    pub fn generate() -> Self {
        let recipes = RecipeEntry::generate_all();
        let (production_methods, consumption_methods) = generate_usages(&recipes);
        Self {
            tech_tree: TechnologyEntry::generate_all().into(),
            recipes: recipes.into(),
            production_methods: production_methods.into(),
            consumption_methods: consumption_methods.into(),
        }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TechnologyEntry {
    tech: Technology,
    /// All technology that this immediately depends on
    prereqs: Vec<Technology>,
    /// All technology immediately depending on this
    postreqs: Vec<Technology>,
}

impl TechnologyEntry {
    fn generate_all() -> EnumMap<Technology, TechnologyEntry> {
        let mut postreqs = EnumMap::from(|_| Vec::new());
        for tech in Technology::iter() {
            for other in Technology::iter() {
                if tech.prerequisites().contains(&other) {
                    postreqs[other].push(tech);
                }
            }
        }

        // Sadly, I can't prove to EnumMap that I can map over the postreqs
        // so we have to clone instead
        EnumMap::from(|tech| TechnologyEntry {
            tech,
            prereqs: tech.prerequisites().to_vec(),
            postreqs: postreqs[tech].clone(),
        })
    }
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RecipeEntry {
    recipe: Recipe,
    ingredients: Vec<ItemStack>,
    /// Time in seconds to make it
    time: f32,
    results: Vec<ItemStack>,
    made_in: Producer,
    handcraftable: bool,
    unlocked_by: Technology,
}

#[derive(Default)]
struct PartialRecipeEntry {
    ingredients: Vec<ItemStack>,
    /// Time in seconds to make it
    time: f32,
    results: Vec<ItemStack>,
    made_in: Producer,
    handcraftable: bool,
    unlocked_by: Technology,
}

fn is(item: Item, count: u32) -> ItemStack {
    ItemStack { item, count }
}

impl RecipeEntry {
    fn generate_all() -> EnumMap<Recipe, RecipeEntry> {
        EnumMap::from(|recipe| {
            // Sorry for this incredibly long table
            let partial = match recipe {
                Recipe::IronSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::IronOre, 1)],
                    time: 1.0,
                    results: vec![is(Item::IronIngot, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CopperSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::CopperOre, 1)],
                    time: 1.0,
                    results: vec![is(Item::CopperIngot, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::StoneSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::StoneOre, 1)],
                    time: 1.0,
                    results: vec![is(Item::StoneBrick, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SiliconSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::SiliconOre, 2)],
                    time: 2.0,
                    results: vec![is(Item::HighPuritySilicon, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::SmeltingPurification,
                },
                Recipe::GraphiteSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::CoalOre, 2)],
                    time: 2.0,
                    results: vec![is(Item::EnergeticGraphite, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::SmeltingPurification,
                },
                Recipe::PlasmaRefining => PartialRecipeEntry {
                    ingredients: vec![is(Item::CrudeOil, 2)],
                    time: 4.0,
                    results: vec![is(Item::Hydrogen, 1), is(Item::RefinedOil, 2)],
                    made_in: Producer::OilRefinery,
                    handcraftable: false,
                    unlocked_by: Technology::PlasmaExtractRefining,
                },
                Recipe::Plastic => PartialRecipeEntry {
                    ingredients: vec![is(Item::EnergeticGraphite, 1), is(Item::RefinedOil, 2)],
                    time: 3.0,
                    results: vec![is(Item::Plastic, 1)],
                    made_in: Producer::ChemicalPlant,
                    handcraftable: false,
                    unlocked_by: Technology::BasicChemicalEngineering,
                },
                Recipe::GrapheneFromFireIce => PartialRecipeEntry {
                    ingredients: vec![is(Item::FireIce, 2)],
                    time: 2.0,
                    results: vec![is(Item::Graphene, 2), is(Item::Hydrogen, 1)],
                    made_in: Producer::ChemicalPlant,
                    handcraftable: false,
                    unlocked_by: Technology::AppliedSuperconductor,
                },
                Recipe::GrapheneFromGraphiteAndSulfuric => PartialRecipeEntry {
                    ingredients: vec![is(Item::EnergeticGraphite, 3), is(Item::RefinedOil, 1)],
                    time: 3.0,
                    results: vec![is(Item::Graphene, 2)],
                    made_in: Producer::ChemicalPlant,
                    handcraftable: false,
                    unlocked_by: Technology::AppliedSuperconductor,
                },
                Recipe::MagnetSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::IronOre, 1)],
                    time: 1.5,
                    results: vec![is(Item::Magnet, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Electromagnet => PartialRecipeEntry {
                    ingredients: vec![is(Item::Magnet, 2), is(Item::CopperIngot, 1)],
                    time: 1.0,
                    results: vec![is(Item::Electromagnet, 2)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                _ => PartialRecipeEntry::default(),
                /*
                TODO
                Recipe::CrystalSiliconFromIngot => {}
                Recipe::CrystalSiliconFromFractal => {}
                Recipe::GlassSmelting => {}
                Recipe::DiamondFromGraphite => {}
                Recipe::DiamondFromKimberlite => {}
                Recipe::XRayCracking => {}
                Recipe::OrganicCrystalFromWood => {}
                Recipe::OrganicCrystalFromPlastic => {}
                Recipe::HydrogenFuelRod => {}
                Recipe::SteelSmelting => {}
                Recipe::SiliconOreFromStone => {}
                Recipe::CircuitBoard => {}
                Recipe::SulfuricAcidFromStone => {}
                Recipe::PlasmaExciter => {}
                Recipe::Processor => {}
                Recipe::PhotonCombinerFromPrism => {}
                Recipe::PhotonCombinerFromCrystal => {}
                Recipe::MicrocrystallineComponent => {}
                Recipe::ElectromagneticMatrix => {}
                Recipe::EnergyMatrix => {}
                Recipe::Foundation => {}
                Recipe::TeslaTower => {}
                Recipe::WirelessPowerTower => {}
                Recipe::WindTurbine => {}
                Recipe::ThermalPowerStation => {}
                Recipe::SolarPanel => {}
                Recipe::ConveyorMK1 => {}
                Recipe::Splitter => {}
                Recipe::StorageMK1 => {}
                Recipe::FluidStorage => {}
                Recipe::SorterMK1 => {}
                Recipe::SorterMK2 => {}
                Recipe::MiningMachine => {}
                Recipe::WaterPump => {}
                Recipe::OilExtractor => {}
                Recipe::OilRefinery => {}
                Recipe::AssemblingMachineMK1 => {}
                Recipe::Smelter => {}
                Recipe::Fractionator => {}
                Recipe::ChemicalPlant => {}
                Recipe::MatrixLab => {}
                */
            };
            // Transform the entry
            RecipeEntry {
                recipe,
                ingredients: partial.ingredients,
                time: partial.time,
                results: partial.results,
                made_in: partial.made_in,
                handcraftable: partial.handcraftable,
                unlocked_by: partial.unlocked_by,
            }
        })
    }
}

/// Generate all the ways to produce and consume items.
///
/// Returns `(producers, consumers)`.
fn generate_usages(
    recipes: &EnumMap<Recipe, RecipeEntry>,
) -> (EnumMap<Item, Vec<Recipe>>, EnumMap<Item, Vec<Recipe>>) {
    let mut producers = EnumMap::from(|_| Vec::new());
    let mut consumers = EnumMap::from(|_| Vec::new());
    for (recipe, entry) in recipes.iter() {
        for produced in entry.results.iter() {
            producers[produced.item].push(recipe);
        }
        for consumed in entry.ingredients.iter() {
            consumers[consumed.item].push(recipe);
        }
    }

    (producers, consumers)
}
