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
    ItemStack {
        item,
        count: count.into(),
    }
}
fn isf(item: Item, count: f64) -> ItemStack {
    ItemStack { item, count }
}

impl RecipeEntry {
    fn generate_all() -> EnumMap<Recipe, RecipeEntry> {
        EnumMap::from(|recipe| {
            // Sorry for this incredibly long table
            let partial = match recipe {
                Recipe::IronSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::IronOre, 1)],
                    time: 1.00,
                    results: vec![is(Item::IronIngot, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::MagnetSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::IronOre, 1)],
                    time: 1.50,
                    results: vec![is(Item::Magnet, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CopperSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::CopperOre, 1)],
                    time: 1.00,
                    results: vec![is(Item::CopperIngot, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::StoneSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::StoneOre, 1)],
                    time: 1.00,
                    results: vec![is(Item::StoneBrick, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Gear => PartialRecipeEntry {
                    ingredients: vec![is(Item::IronIngot, 1)],
                    time: 1.00,
                    results: vec![is(Item::Gear, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Electromagnet => PartialRecipeEntry {
                    ingredients: vec![is(Item::Magnet, 2), is(Item::CopperIngot, 1)],
                    time: 1.00,
                    results: vec![is(Item::Electromagnet, 2)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::WindTurbine => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 6),
                        is(Item::Gear, 1),
                        is(Item::Electromagnet, 3),
                    ],
                    time: 4.00,
                    results: vec![is(Item::WindTurbine, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::TeslaTower => PartialRecipeEntry {
                    ingredients: vec![is(Item::IronIngot, 2), is(Item::Electromagnet, 1)],
                    time: 1.00,
                    results: vec![is(Item::TeslaTower, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ElectromagneticMatrix => PartialRecipeEntry {
                    ingredients: vec![is(Item::Electromagnet, 1), is(Item::CircuitBoard, 1)],
                    time: 3.00,
                    results: vec![is(Item::ElectromagneticMatrix, 1)],
                    made_in: Producer::MatrixLab,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::MatrixLab => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 8),
                        is(Item::Glass, 4),
                        is(Item::CircuitBoard, 4),
                        is(Item::Electromagnet, 4),
                    ],
                    time: 3.00,
                    results: vec![is(Item::MatrixLab, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Prism => PartialRecipeEntry {
                    ingredients: vec![is(Item::Glass, 3)],
                    time: 2.00,
                    results: vec![is(Item::Prism, 2)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::PlasmaExciter => PartialRecipeEntry {
                    ingredients: vec![is(Item::Electromagnet, 4), is(Item::Prism, 2)],
                    time: 2.00,
                    results: vec![is(Item::PlasmaExciter, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::WirelessPowerTower => PartialRecipeEntry {
                    ingredients: vec![is(Item::TeslaTower, 1), is(Item::PlasmaExciter, 3)],
                    time: 3.00,
                    results: vec![is(Item::WirelessPowerTower, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::OilExtractor => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::Steel, 12),
                        is(Item::StoneBrick, 12),
                        is(Item::CircuitBoard, 6),
                        is(Item::PlasmaExciter, 4),
                    ],
                    time: 8.00,
                    results: vec![is(Item::OilExtractor, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::OilRefinery => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::Steel, 10),
                        is(Item::StoneBrick, 10),
                        is(Item::CircuitBoard, 6),
                        is(Item::PlasmaExciter, 6),
                    ],
                    time: 6.00,
                    results: vec![is(Item::OilRefinery, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::PlasmaRefining => PartialRecipeEntry {
                    ingredients: vec![is(Item::CrudeOil, 2)],
                    time: 4.00,
                    results: vec![is(Item::Hydrogen, 1), is(Item::RefinedOil, 2)],
                    made_in: Producer::OilRefinery,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::GraphiteSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::CoalOre, 2)],
                    time: 2.00,
                    results: vec![is(Item::EnergeticGraphite, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::EnergyMatrix => PartialRecipeEntry {
                    ingredients: vec![is(Item::EnergeticGraphite, 2), is(Item::Hydrogen, 2)],
                    time: 6.00,
                    results: vec![is(Item::EnergyMatrix, 1)],
                    made_in: Producer::MatrixLab,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::HydrogenFuelRod => PartialRecipeEntry {
                    ingredients: vec![is(Item::TitaniumIngot, 1), is(Item::Hydrogen, 5)],
                    time: 3.00,
                    results: vec![is(Item::HydrogenFuelRod, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Thruster => PartialRecipeEntry {
                    ingredients: vec![is(Item::Steel, 2), is(Item::CopperIngot, 3)],
                    time: 4.00,
                    results: vec![is(Item::Thruster, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ReinforcedThruster => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::TitaniumAlloy, 5),
                        is(Item::ElectromagneticTurbine, 5),
                    ],
                    time: 6.00,
                    results: vec![is(Item::ReinforcedThruster, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ChemicalPlant => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::Steel, 8),
                        is(Item::StoneBrick, 8),
                        is(Item::Glass, 8),
                        is(Item::CircuitBoard, 2),
                    ],
                    time: 5.00,
                    results: vec![is(Item::ChemicalPlant, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Plastic => PartialRecipeEntry {
                    ingredients: vec![is(Item::RefinedOil, 2), is(Item::EnergeticGraphite, 1)],
                    time: 3.00,
                    results: vec![is(Item::Plastic, 1)],
                    made_in: Producer::ChemicalPlant,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SulfuricAcidFromStone => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::RefinedOil, 6),
                        is(Item::StoneOre, 8),
                        is(Item::Water, 4),
                    ],
                    time: 6.00,
                    results: vec![is(Item::SulfuricAcid, 4)],
                    made_in: Producer::ChemicalPlant,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::OrganicCrystalFromPlastic => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::Plastic, 2),
                        is(Item::RefinedOil, 1),
                        is(Item::Water, 1),
                    ],
                    time: 6.00,
                    results: vec![is(Item::OrganicCrystal, 1)],
                    made_in: Producer::ChemicalPlant,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::TitaniumCrystal => PartialRecipeEntry {
                    ingredients: vec![is(Item::OrganicCrystal, 1), is(Item::TitaniumIngot, 3)],
                    time: 4.00,
                    results: vec![is(Item::TitaniumCrystal, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::StructureMatrix => PartialRecipeEntry {
                    ingredients: vec![is(Item::Diamond, 1), is(Item::TitaniumCrystal, 1)],
                    time: 8.00,
                    results: vec![is(Item::StructureMatrix, 1)],
                    made_in: Producer::MatrixLab,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CasimirCrystalFromTitanium => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::TitaniumCrystal, 1),
                        is(Item::Graphene, 2),
                        is(Item::Hydrogen, 12),
                    ],
                    time: 4.00,
                    results: vec![is(Item::CasimirCrystal, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CasimirCrystalFromOpticalGratingCrystal => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::OpticalGratingCrystal, 6),
                        is(Item::Graphene, 2),
                        is(Item::Hydrogen, 12),
                    ],
                    time: 4.00,
                    results: vec![is(Item::CasimirCrystal, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::TitaniumGlass => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::Glass, 2),
                        is(Item::TitaniumIngot, 2),
                        is(Item::Water, 2),
                    ],
                    time: 5.00,
                    results: vec![is(Item::TitaniumGlass, 2)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::GrapheneFromGraphiteAndSulfuric => PartialRecipeEntry {
                    ingredients: vec![is(Item::EnergeticGraphite, 3), is(Item::SulfuricAcid, 1)],
                    time: 3.00,
                    results: vec![is(Item::Graphene, 2)],
                    made_in: Producer::ChemicalPlant,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::GrapheneFromFireIce => PartialRecipeEntry {
                    ingredients: vec![is(Item::FireIce, 2)],
                    time: 2.00,
                    results: vec![is(Item::Graphene, 2), is(Item::Hydrogen, 1)],
                    made_in: Producer::ChemicalPlant,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CarbonNanotubeFromGraphene => PartialRecipeEntry {
                    ingredients: vec![is(Item::Graphene, 3), is(Item::TitaniumIngot, 1)],
                    time: 4.00,
                    results: vec![is(Item::CarbonNanotube, 2)],
                    made_in: Producer::ChemicalPlant,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SiliconOreFromStone => PartialRecipeEntry {
                    ingredients: vec![is(Item::StoneOre, 10)],
                    time: 10.00,
                    results: vec![is(Item::SiliconOre, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CarbonNanotubeFromSpiniform => PartialRecipeEntry {
                    ingredients: vec![is(Item::SpiniformStalagmiteCrystal, 2)],
                    time: 4.00,
                    results: vec![is(Item::CarbonNanotube, 2)],
                    made_in: Producer::ChemicalPlant,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ParticleBroadband => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::CarbonNanotube, 2),
                        is(Item::CrystalSilicon, 2),
                        is(Item::Plastic, 1),
                    ],
                    time: 8.00,
                    results: vec![is(Item::ParticleBroadband, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CrystalSiliconFromIngot => PartialRecipeEntry {
                    ingredients: vec![is(Item::HighPuritySilicon, 1)],
                    time: 2.00,
                    results: vec![is(Item::CrystalSilicon, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::PlaneFilter => PartialRecipeEntry {
                    ingredients: vec![is(Item::CasimirCrystal, 1), is(Item::TitaniumGlass, 2)],
                    time: 12.00,
                    results: vec![is(Item::PlaneFilter, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::MiniatureParticleCollider => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::TitaniumAlloy, 20),
                        is(Item::FrameMaterial, 20),
                        is(Item::SuperMagneticRing, 50),
                        is(Item::Graphene, 10),
                        is(Item::Processor, 8),
                    ],
                    time: 15.00,
                    results: vec![is(Item::MiniatureParticleCollider, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::DeuteriumInParticleCollider => PartialRecipeEntry {
                    ingredients: vec![is(Item::Hydrogen, 10)],
                    time: 5.00,
                    results: vec![is(Item::Deuterium, 5)],
                    made_in: Producer::MiniatureParticleCollider,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::DeuteronFuelRod => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::TitaniumAlloy, 1),
                        is(Item::Deuterium, 10),
                        is(Item::SuperMagneticRing, 1),
                    ],
                    time: 6.00,
                    results: vec![is(Item::DeuteronFuelRod, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::AnnihilationConstraintSphere => PartialRecipeEntry {
                    ingredients: vec![is(Item::ParticleContainer, 1), is(Item::Processor, 1)],
                    time: 20.00,
                    results: vec![is(Item::AnnihilationConstraintSphere, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ArtificialStar => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::TitaniumAlloy, 20),
                        is(Item::FrameMaterial, 20),
                        is(Item::AnnihilationConstraintSphere, 10),
                        is(Item::QuantumChip, 10),
                    ],
                    time: 30.00,
                    results: vec![is(Item::ArtificialStar, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::AntimatterFuelRod => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::Antimatter, 10),
                        is(Item::Hydrogen, 10),
                        is(Item::AnnihilationConstraintSphere, 1),
                        is(Item::TitaniumAlloy, 1),
                    ],
                    time: 12.00,
                    results: vec![is(Item::AntimatterFuelRod, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::AssemblingMachineMK1 => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 4),
                        is(Item::Gear, 8),
                        is(Item::CircuitBoard, 4),
                    ],
                    time: 2.00,
                    results: vec![is(Item::AssemblingMachineMK1, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::AssemblingMachineMK2 => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::AssemblingMachineMK1, 1),
                        is(Item::Graphene, 8),
                        is(Item::Processor, 4),
                    ],
                    time: 3.00,
                    results: vec![is(Item::AssemblingMachineMK2, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::AssemblingMachineMK3 => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::AssemblingMachineMK2, 1),
                        is(Item::ParticleBroadband, 8),
                        is(Item::QuantumChip, 2),
                    ],
                    time: 4.00,
                    results: vec![is(Item::AssemblingMachineMK3, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::MiningMachine => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 4),
                        is(Item::CircuitBoard, 2),
                        is(Item::Electromagnet, 2),
                        is(Item::Gear, 2),
                    ],
                    time: 3.00,
                    results: vec![is(Item::MiningMachine, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::WaterPump => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 8),
                        is(Item::StoneBrick, 4),
                        is(Item::Motor, 4),
                        is(Item::CircuitBoard, 2),
                    ],
                    time: 4.00,
                    results: vec![is(Item::WaterPump, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CircuitBoard => PartialRecipeEntry {
                    ingredients: vec![is(Item::IronIngot, 2), is(Item::CopperIngot, 1)],
                    time: 1.00,
                    results: vec![is(Item::CircuitBoard, 2)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Processor => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::CircuitBoard, 2),
                        is(Item::MicrocrystallineComponent, 2),
                    ],
                    time: 3.00,
                    results: vec![is(Item::Processor, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::QuantumChip => PartialRecipeEntry {
                    ingredients: vec![is(Item::Processor, 2), is(Item::PlaneFilter, 2)],
                    time: 6.00,
                    results: vec![is(Item::QuantumChip, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::MicrocrystallineComponent => PartialRecipeEntry {
                    ingredients: vec![is(Item::HighPuritySilicon, 2), is(Item::CopperIngot, 1)],
                    time: 2.00,
                    results: vec![is(Item::MicrocrystallineComponent, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::OrganicCrystalFromWood => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::Log, 20),
                        is(Item::PlantFuel, 30),
                        is(Item::Water, 10),
                    ],
                    time: 6.00,
                    results: vec![is(Item::OrganicCrystal, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::InformationMatrix => PartialRecipeEntry {
                    ingredients: vec![is(Item::Processor, 2), is(Item::ParticleBroadband, 1)],
                    time: 10.00,
                    results: vec![is(Item::InformationMatrix, 1)],
                    made_in: Producer::MatrixLab,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Smelter => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 4),
                        is(Item::StoneBrick, 2),
                        is(Item::CircuitBoard, 4),
                        is(Item::Electromagnet, 2),
                    ],
                    time: 3.00,
                    results: vec![is(Item::Smelter, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::GlassSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::StoneOre, 2)],
                    time: 2.00,
                    results: vec![is(Item::Glass, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::XRayCracking => PartialRecipeEntry {
                    ingredients: vec![is(Item::RefinedOil, 1), is(Item::Hydrogen, 2)],
                    time: 4.00,
                    results: vec![is(Item::Hydrogen, 3), is(Item::EnergeticGraphite, 1)],
                    made_in: Producer::OilRefinery,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SiliconSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::SiliconOre, 2)],
                    time: 2.00,
                    results: vec![is(Item::HighPuritySilicon, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::DiamondFromGraphite => PartialRecipeEntry {
                    ingredients: vec![is(Item::EnergeticGraphite, 1)],
                    time: 2.00,
                    results: vec![is(Item::Diamond, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::DiamondFromKimberlite => PartialRecipeEntry {
                    ingredients: vec![is(Item::Kimberlite, 1)],
                    time: 2.00,
                    results: vec![is(Item::Diamond, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CrystalSiliconFromFractal => PartialRecipeEntry {
                    ingredients: vec![is(Item::FractalSilicon, 1)],
                    time: 4.00,
                    results: vec![is(Item::CrystalSilicon, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SteelSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::IronIngot, 3)],
                    time: 3.00,
                    results: vec![is(Item::Steel, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ThermalPowerStation => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 10),
                        is(Item::StoneBrick, 4),
                        is(Item::Gear, 4),
                        is(Item::Electromagnet, 4),
                    ],
                    time: 5.00,
                    results: vec![is(Item::ThermalPowerStation, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::TitaniumSmelting => PartialRecipeEntry {
                    ingredients: vec![is(Item::TitaniumOre, 2)],
                    time: 2.00,
                    results: vec![is(Item::TitaniumIngot, 1)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::TitaniumAlloy => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::TitaniumIngot, 4),
                        is(Item::Steel, 4),
                        is(Item::SulfuricAcid, 8),
                    ],
                    time: 12.00,
                    results: vec![is(Item::TitaniumAlloy, 4)],
                    made_in: Producer::Smelter,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SolarPanel => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::CopperIngot, 6),
                        is(Item::HighPuritySilicon, 6),
                        is(Item::CircuitBoard, 4),
                    ],
                    time: 5.00,
                    results: vec![is(Item::SolarPanel, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::PhotonCombinerFromPrism => PartialRecipeEntry {
                    ingredients: vec![is(Item::Prism, 2), is(Item::CircuitBoard, 1)],
                    time: 3.00,
                    results: vec![is(Item::PhotonCombiner, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::PhotonCombinerFromCrystal => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::OpticalGratingCrystal, 1),
                        is(Item::CircuitBoard, 1),
                    ],
                    time: 3.00,
                    results: vec![is(Item::PhotonCombiner, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::EMRailEjector => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::Steel, 20),
                        is(Item::Gear, 20),
                        is(Item::Processor, 5),
                        is(Item::SuperMagneticRing, 10),
                    ],
                    time: 6.00,
                    results: vec![is(Item::EMRailEjector, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::RayReceiver => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::Steel, 20),
                        is(Item::HighPuritySilicon, 20),
                        is(Item::PhotonCombiner, 10),
                        is(Item::Processor, 5),
                        is(Item::SuperMagneticRing, 20),
                    ],
                    time: 8.00,
                    results: vec![is(Item::RayReceiver, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SatelliteSubstation => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::WirelessPowerTower, 1),
                        is(Item::SuperMagneticRing, 10),
                        is(Item::FrameMaterial, 2),
                    ],
                    time: 5.00,
                    results: vec![is(Item::SatelliteSubstation, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::DiracInversion => PartialRecipeEntry {
                    ingredients: vec![is(Item::CriticalPhoton, 2)],
                    time: 2.00,
                    results: vec![is(Item::Antimatter, 2), is(Item::Hydrogen, 2)],
                    made_in: Producer::MiniatureParticleCollider,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::UniverseMatrix => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::ElectromagneticMatrix, 1),
                        is(Item::EnergyMatrix, 1),
                        is(Item::StructureMatrix, 1),
                        is(Item::InformationMatrix, 1),
                        is(Item::GravityMatrix, 1),
                        is(Item::Antimatter, 1),
                    ],
                    time: 15.00,
                    results: vec![is(Item::UniverseMatrix, 1)],
                    made_in: Producer::MatrixLab,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Accumulator => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 6),
                        is(Item::SuperMagneticRing, 6),
                        is(Item::CrystalSilicon, 4),
                    ],
                    time: 5.00,
                    results: vec![is(Item::Accumulator, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::EnergyExchanger => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::TitaniumAlloy, 40),
                        is(Item::Steel, 40),
                        is(Item::Processor, 40),
                        is(Item::ParticleContainer, 8),
                    ],
                    time: 15.00,
                    results: vec![is(Item::EnergyExchanger, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SpaceWarperFromLens => PartialRecipeEntry {
                    ingredients: vec![is(Item::GravitonLens, 1)],
                    time: 10.00,
                    results: vec![is(Item::SpaceWarper, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SpaceWarperFromMatrix => PartialRecipeEntry {
                    ingredients: vec![is(Item::GravityMatrix, 1)],
                    time: 10.00,
                    results: vec![is(Item::SpaceWarper, 8)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::FrameMaterial => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::CarbonNanotube, 4),
                        is(Item::TitaniumAlloy, 1),
                        is(Item::HighPuritySilicon, 1),
                    ],
                    time: 6.00,
                    results: vec![is(Item::FrameMaterial, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::DysonSphereComponent => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::FrameMaterial, 3),
                        is(Item::SolarPanel, 3),
                        is(Item::Processor, 3),
                    ],
                    time: 8.00,
                    results: vec![is(Item::DysonSphereComponent, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::VerticalLaunchingSilo => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::TitaniumAlloy, 80),
                        is(Item::FrameMaterial, 30),
                        is(Item::GravitonLens, 20),
                        is(Item::QuantumChip, 10),
                    ],
                    time: 30.00,
                    results: vec![is(Item::VerticalLaunchingSilo, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SmallCarrierRocket => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::DysonSphereComponent, 2),
                        is(Item::DeuteronFuelRod, 2),
                        is(Item::QuantumChip, 2),
                    ],
                    time: 6.00,
                    results: vec![is(Item::SmallCarrierRocket, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ConveyorMK1 => PartialRecipeEntry {
                    ingredients: vec![is(Item::IronIngot, 2), is(Item::Gear, 1)],
                    time: 1.00,
                    results: vec![is(Item::ConveyorMK1, 3)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SorterMK1 => PartialRecipeEntry {
                    ingredients: vec![is(Item::IronIngot, 1), is(Item::CircuitBoard, 1)],
                    time: 1.00,
                    results: vec![is(Item::SorterMK1, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::StorageMK1 => PartialRecipeEntry {
                    ingredients: vec![is(Item::IronIngot, 4), is(Item::StoneBrick, 4)],
                    time: 2.00,
                    results: vec![is(Item::StorageMK1, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Splitter => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 3),
                        is(Item::Gear, 2),
                        is(Item::CircuitBoard, 1),
                    ],
                    time: 2.00,
                    results: vec![is(Item::Splitter, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SorterMK2 => PartialRecipeEntry {
                    ingredients: vec![is(Item::SorterMK1, 2), is(Item::Motor, 1)],
                    time: 1.00,
                    results: vec![is(Item::SorterMK2, 2)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ConveyorMK2 => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::ConveyorMK1, 3),
                        is(Item::ElectromagneticTurbine, 1),
                    ],
                    time: 1.00,
                    results: vec![is(Item::ConveyorMK2, 3)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SorterMK3 => PartialRecipeEntry {
                    ingredients: vec![is(Item::SorterMK2, 2), is(Item::ElectromagneticTurbine, 1)],
                    time: 1.00,
                    results: vec![is(Item::SorterMK3, 2)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::StorageMK2 => PartialRecipeEntry {
                    ingredients: vec![is(Item::Steel, 8), is(Item::StoneBrick, 8)],
                    time: 4.00,
                    results: vec![is(Item::StorageMK2, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ConveyorMK3 => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::ConveyorMK2, 3),
                        is(Item::SuperMagneticRing, 1),
                        is(Item::Graphene, 1),
                    ],
                    time: 1.00,
                    results: vec![is(Item::ConveyorMK3, 3)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::PlanetaryLogisticsStation => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::Steel, 40),
                        is(Item::TitaniumIngot, 40),
                        is(Item::Processor, 40),
                        is(Item::ParticleContainer, 20),
                    ],
                    time: 20.00,
                    results: vec![is(Item::PlanetaryLogisticsStation, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::LogisticsDrone => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 5),
                        is(Item::Processor, 2),
                        is(Item::Thruster, 2),
                    ],
                    time: 4.00,
                    results: vec![is(Item::LogisticsDrone, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::InterstellarLogisticsStation => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::PlanetaryLogisticsStation, 1),
                        is(Item::TitaniumAlloy, 40),
                        is(Item::ParticleContainer, 20),
                    ],
                    time: 30.00,
                    results: vec![is(Item::InterstellarLogisticsStation, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::LogisticsVessel => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::TitaniumAlloy, 10),
                        is(Item::Processor, 10),
                        is(Item::ReinforcedThruster, 2),
                    ],
                    time: 6.00,
                    results: vec![is(Item::LogisticsVessel, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Motor => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 2),
                        is(Item::Gear, 1),
                        is(Item::Electromagnet, 1),
                    ],
                    time: 2.00,
                    results: vec![is(Item::Motor, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ElectromagneticTurbine => PartialRecipeEntry {
                    ingredients: vec![is(Item::Motor, 2), is(Item::Electromagnet, 2)],
                    time: 2.00,
                    results: vec![is(Item::ElectromagneticTurbine, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ParticleContainerFromEMTurbine => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::ElectromagneticTurbine, 2),
                        is(Item::CopperIngot, 2),
                        is(Item::Graphene, 2),
                    ],
                    time: 4.00,
                    results: vec![is(Item::ParticleContainer, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::ParticleContainerFromUnipolar => PartialRecipeEntry {
                    ingredients: vec![is(Item::UnipolarMagnet, 10), is(Item::CopperIngot, 2)],
                    time: 4.00,
                    results: vec![is(Item::ParticleContainer, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::GravitonLens => PartialRecipeEntry {
                    ingredients: vec![is(Item::Diamond, 4), is(Item::StrangeMatter, 1)],
                    time: 6.00,
                    results: vec![is(Item::GravitonLens, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::GravityMatrix => PartialRecipeEntry {
                    ingredients: vec![is(Item::GravitonLens, 1), is(Item::QuantumChip, 1)],
                    time: 24.00,
                    results: vec![is(Item::GravityMatrix, 2)],
                    made_in: Producer::MatrixLab,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SuperMagneticRing => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::ElectromagneticTurbine, 2),
                        is(Item::Magnet, 3),
                        is(Item::EnergeticGraphite, 1),
                    ],
                    time: 3.00,
                    results: vec![is(Item::SuperMagneticRing, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::StrangeMatter => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::ParticleContainer, 2),
                        is(Item::IronIngot, 2),
                        is(Item::Deuterium, 10),
                    ],
                    time: 8.00,
                    results: vec![is(Item::StrangeMatter, 1)],
                    made_in: Producer::MiniatureParticleCollider,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                // accelerators and spray painters are nyi
                Recipe::Fractionator => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::Steel, 8),
                        is(Item::StoneBrick, 4),
                        is(Item::Glass, 4),
                        is(Item::Processor, 1),
                    ],
                    time: 3.00,
                    results: vec![is(Item::Fractionator, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::OrbitCollector => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::InterstellarLogisticsStation, 1),
                        is(Item::SuperMagneticRing, 50),
                        is(Item::ReinforcedThruster, 20),
                        is(Item::FullAccumulator, 20),
                    ],
                    time: 30.00,
                    results: vec![is(Item::OrbitCollector, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::Foundation => PartialRecipeEntry {
                    ingredients: vec![is(Item::StoneBrick, 3), is(Item::Steel, 1)],
                    time: 1.00,
                    results: vec![is(Item::Foundation, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::StorageTank => PartialRecipeEntry {
                    ingredients: vec![
                        is(Item::IronIngot, 8),
                        is(Item::StoneBrick, 4),
                        is(Item::Glass, 4),
                    ],
                    time: 2.00,
                    results: vec![is(Item::StorageTank, 1)],
                    made_in: Producer::AssemblingMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::DeuteriumFractionation => PartialRecipeEntry {
                    ingredients: vec![is(Item::Hydrogen, 1)],
                    time: 1.00,
                    results: vec![isf(Item::Deuterium, 0.01), isf(Item::Hydrogen, 0.99)],
                    made_in: Producer::Fractionator,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::IronMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::IronOre, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CopperMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::CopperOre, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SiliconMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::SiliconOre, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::TitaniumMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::TitaniumOre, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::StoneMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::StoneOre, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CoalMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::CoalOre, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::FireIceMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::FireIce, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::DiamondMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::Kimberlite, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::FractalSiliconMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::FractalSilicon, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::OrganicCrystalMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::OrganicCrystal, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::OpticalGratingCrystalMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::OpticalGratingCrystal, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SpiniformStalagmiteCrystalMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::SpiniformStalagmiteCrystal, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::UnipolarMagnetMining => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 2.00,
                    results: vec![is(Item::UnipolarMagnet, 1)],
                    made_in: Producer::MiningMachine,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::OilExtraction => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 1.00,
                    results: vec![is(Item::CrudeOil, 1)],
                    made_in: Producer::OilExtractor,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::CriticalPhotonReceiving => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 1.00,
                    results: vec![is(Item::CriticalPhoton, 1)],
                    made_in: Producer::RayReceiver,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::WaterPumping => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 1.20,
                    results: vec![is(Item::Water, 1)],
                    made_in: Producer::WaterPump,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::SulfuricAcidPumping => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 1.20,
                    results: vec![is(Item::SulfuricAcid, 1)],
                    made_in: Producer::WaterPump,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::GasGiantCollection => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 8.00,
                    results: vec![is(Item::Hydrogen, 4), is(Item::Deuterium, 0)],
                    made_in: Producer::OrbitCollector,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
                Recipe::IceGiantCollection => PartialRecipeEntry {
                    ingredients: vec![],
                    time: 8.00,
                    results: vec![is(Item::FireIce, 2), is(Item::Hydrogen, 1)],
                    made_in: Producer::OrbitCollector,
                    handcraftable: true,
                    unlocked_by: Technology::DysonSphereProgram,
                },
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
