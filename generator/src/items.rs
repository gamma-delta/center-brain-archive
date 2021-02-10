use std::fmt::{Debug, Formatter};

use enum_map::Enum;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Enum, Serialize, JsonSchema)]
pub enum Item {
    IronOre,
    CopperOre,
    StoneOre,
    CoalOre,
    SiliconOre,
    TitaniumOre,
    Water,
    CrudeOil,
    Hydrogen,
    Deuterium,
    Antimatter,
    Kimberlite,
    IronIngot,
    CopperIngot,
    Stone,
    EnergeticGraphite,
    HighPuritySilicon,
    TitaniumIngot,
    SulfuricAcid,
    RefinedOil,
    HydrogenFuelRod,
    DeuteronFuelRod,
    AntimatterFuelRod,
    FractalSilicon,
    Magnet,
    Electromagnet,
    Glass,
    Diamond,
    CrystalSilicon,
    TitaniumAlloy,
    FireIce,
    Plastic,
    OrganicCrystal,
    Graphene,
    Thruster,
    OpticalGratingCrystal,
    Steel,
    CircuitBoard,
    Prism,
    Motor,
    MicrocrystallineComponent,
    CasimirCrystal,
    StrangeMatter,
    TitaniumCrystal,
    CarbonNanotube,
    ReinforcedThruster,
    SpiniformStalagmiteCrystal,
    Gear,
    PlasmaExciter,
    PhotonCombiner,
    ElectromagneticTurbine,
    Processor,
    AnnihilationConstraintSphere,
    TitaniumGlass,
    ParticleBroadband,
    LogisticsDrone,
    UnipolarMagnet,
    Foundation,
    CriticalPhoton,
    ParticleContainer,
    SuperMagneticRing,
    GravitonLens,
    SpaceWarper,
    PlaneFilter,
    QuantumChip,
    LogisticsVessel,
    Log,
    ElectromagneticMatrix,
    EnergyMatrix,
    StructureMatrix,
    InformationMatrix,
    GravityMatrix,
    UniverseMatrix,
    SolarSail,
    FrameMaterial,
    DysonSphereComponent,
    SmallCarrierRocket,
    PlantFuel,

    TeslaTower,
    WirelessPowerTower,
    SatelliteSubstation,
    WindTurbine,
    ThermalPowerStation,
    SolarPanel,
    MiniFusionPowerStation,
    Accumulator,
    FullAccumulator,
    EnergyExchanger,
    RayReceiver,
    ArtificialStar,
    ConveyorMK1,
    ConveyorMK2,
    ConveyorMK3,
    Splitter,
    StorageMK1,
    StorageMK2,
    PlanetaryLogisticsStation,
    InterstellarLogisticsStation,
    OrbitCollector,
    EMRailEjector,
    SorterMK1,
    SorterMK2,
    SorterMK3,
    MiningMachine,
    OilExtractor,
    OilRefinery,
    MiniatureParticleCollider,
    MatrixLab,
    VerticalLaunchingSilo,
    AssemblingMachineMK1,
    AssemblingMachineMK2,
    AssemblingMachineMK3,
    Smelter,
    ChemicalPlant,
    Fractionator,
    WaterPump,
    StorageTank,
}

#[derive(Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ItemStack {
    pub item: Item,
    /// This is a floating-point value to reflect chance recipies
    /// (Specifically, Deuterium).
    ///
    /// I'm using f64 because I'm not 100% certain f32 can store 0.01...
    /// Plus, JS uses f64s so it's safest this way.
    pub count: f64,
}

impl Debug for ItemStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{:?}", self.count, self.item)
    }
}