use enum_map::Enum;
use schemars::JsonSchema;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, EnumIter, Serialize, JsonSchema)]
pub enum Technology {
    DysonSphereProgram,
    Electromagnetism,
    BasicLogisticsSystem,
    AutomaticMetallurgy,
    ElectromagneticMatrix,
    BasicAssemblingProcesses,
    FluidStorageEncapsulation,
    HighEfficiencyPlasmaControl,
    ElectromagneticDrive,
    ImprovedLogisticsSystem,
    SteelSmelting,
    SmeltingPurification,
    ThermalPower,
    PlasmaExtractRefining,
    AccelerantMK1,
    EnvironmentModification,
    CrystalSmelting,
    SolarCollection,
    SemiconductorMaterial,
    DeuteriumFractionation,
    BasicChemicalEngineering,
    EnergyMatrix,
    MagneticLevitationTechnology,
    HighEfficiencyLogisticsSystem,
    TitaniumSmelting,
    EnergyStorage,
    PhotonFrequencyConversion,
    Processor,
    AppliedSuperconductor,
    PolymerChemicalEngineering,
    XRayCracking,
    HydrogenFuelRod,
    SuperMagneticFieldGenerator,
    PlanetaryLogisticsSystem,
    SolarSailOrbitSystem,
    HighSpeedAssemblingProcesses,
    HighStrengthCrystal,
    Thruster,
    AccelerantMK2,
    MagneticParticleTrap,
    HighStrengthTitaniumAlloy,
    HighStrengthLightweightStructure,
    RayReceiver,
    MiniFusionPowerGeneration,
    HighStrengthMaterial,
    StructureMatrix,
    ReinforcedThruster,
    InterstellarLogisticsSystem,
    InterstellarPowerTransmission,
    ParticleControlTechnology,
    HighStrengthGlass,
    CasimirCrystal,
    MiniatureParticleCollider,
    AccelerantMK3,
    SatellitePowerDistributionSystem,
    GasGiantsExplotiation,
    InformationMatrix,
    WaveFunctionInterference,
    StrangeMatter,
    VerticalLaunchingSilo,
    QuantumChip,
    GravitationalWaveRefraction,
    DysonSphereStressSystem,
    PlanetaryIonosphereUtilization,
    QuantumPrintingTechnology,
    GravityMatrix,
    DiracInversionMechanism,
    ControlledAnnihilationReaction,
    ArtificialStar,
    UniverseMatrix,
    MissionCompleted,
}

impl Technology {
    /// Get all the technologies that must be researched immediately before this one.
    pub fn prerequisites(self) -> &'static [Technology] {
        match self {
            Technology::DysonSphereProgram => &[],
            Technology::Electromagnetism => &[Technology::DysonSphereProgram],
            Technology::BasicLogisticsSystem => &[Technology::Electromagnetism],
            Technology::AutomaticMetallurgy => &[Technology::Electromagnetism],
            Technology::ElectromagneticMatrix => &[Technology::Electromagnetism],
            Technology::BasicAssemblingProcesses => &[Technology::Electromagnetism],
            Technology::FluidStorageEncapsulation => &[Technology::Electromagnetism],
            Technology::HighEfficiencyPlasmaControl => &[Technology::Electromagnetism],
            Technology::ElectromagneticDrive => &[Technology::Electromagnetism],
            Technology::ImprovedLogisticsSystem => &[Technology::BasicLogisticsSystem],
            Technology::SteelSmelting => &[Technology::AutomaticMetallurgy],
            Technology::SmeltingPurification => &[Technology::AutomaticMetallurgy],
            Technology::ThermalPower => &[Technology::BasicAssemblingProcesses],
            Technology::PlasmaExtractRefining => &[
                Technology::FluidStorageEncapsulation,
                Technology::HighEfficiencyPlasmaControl,
            ],
            Technology::AccelerantMK1 => &[Technology::HighEfficiencyPlasmaControl],
            Technology::EnvironmentModification => &[Technology::SteelSmelting],
            Technology::CrystalSmelting => &[Technology::SmeltingPurification],
            Technology::SolarCollection => &[
                Technology::SmeltingPurification,
                Technology::ElectromagneticMatrix,
            ],
            Technology::SemiconductorMaterial => &[Technology::BasicAssemblingProcesses],
            Technology::DeuteriumFractionation => &[Technology::ThermalPower],
            Technology::BasicChemicalEngineering => &[
                Technology::FluidStorageEncapsulation,
                Technology::PlasmaExtractRefining,
            ],
            Technology::EnergyMatrix => &[Technology::PlasmaExtractRefining],
            Technology::MagneticLevitationTechnology => &[Technology::ElectromagneticDrive],
            Technology::HighEfficiencyLogisticsSystem => &[Technology::ImprovedLogisticsSystem],
            Technology::TitaniumSmelting => &[Technology::SteelSmelting],
            Technology::EnergyStorage => &[Technology::CrystalSmelting],
            Technology::PhotonFrequencyConversion => &[Technology::SolarCollection],
            Technology::Processor => &[Technology::SemiconductorMaterial],
            Technology::AppliedSuperconductor => &[Technology::BasicChemicalEngineering],
            Technology::PolymerChemicalEngineering => &[Technology::BasicChemicalEngineering],
            Technology::XRayCracking => &[
                Technology::BasicChemicalEngineering,
                Technology::PlasmaExtractRefining,
            ],
            Technology::HydrogenFuelRod => &[Technology::EnergyMatrix],
            Technology::SuperMagneticFieldGenerator => &[Technology::MagneticLevitationTechnology],
            Technology::PlanetaryLogisticsSystem => &[Technology::HighEfficiencyLogisticsSystem],
            Technology::SolarSailOrbitSystem => &[Technology::PhotonFrequencyConversion],
            Technology::HighSpeedAssemblingProcesses => {
                &[Technology::BasicAssemblingProcesses, Technology::Processor]
            }
            Technology::HighStrengthCrystal => &[Technology::PolymerChemicalEngineering],
            Technology::Thruster => &[Technology::HydrogenFuelRod],
            Technology::AccelerantMK2 => &[Technology::AccelerantMK1],
            Technology::MagneticParticleTrap => &[Technology::MagneticLevitationTechnology],
            Technology::HighStrengthTitaniumAlloy => &[Technology::TitaniumSmelting],
            Technology::HighStrengthLightweightStructure => &[Technology::SolarSailOrbitSystem],
            Technology::RayReceiver => &[Technology::SolarSailOrbitSystem],
            Technology::MiniFusionPowerGeneration => &[Technology::DeuteriumFractionation],
            Technology::HighStrengthMaterial => &[Technology::AppliedSuperconductor],
            Technology::StructureMatrix => &[Technology::HighStrengthCrystal],
            Technology::ReinforcedThruster => &[Technology::Thruster],
            Technology::InterstellarLogisticsSystem => &[
                Technology::PlanetaryLogisticsSystem,
                Technology::HighStrengthTitaniumAlloy,
            ],
            Technology::InterstellarPowerTransmission => &[
                Technology::EnergyStorage,
                Technology::HighStrengthTitaniumAlloy,
            ],
            Technology::ParticleControlTechnology => &[Technology::HighStrengthMaterial],
            Technology::HighStrengthGlass => &[Technology::HighStrengthMaterial],
            Technology::CasimirCrystal => &[Technology::StructureMatrix],
            Technology::MiniatureParticleCollider => &[Technology::MiniatureParticleCollider],
            Technology::AccelerantMK3 => &[Technology::AccelerantMK2],
            Technology::SatellitePowerDistributionSystem => {
                &[Technology::SuperMagneticFieldGenerator]
            }
            Technology::GasGiantsExplotiation => &[
                Technology::InterstellarLogisticsSystem,
                Technology::InterstellarPowerTransmission,
            ],
            Technology::InformationMatrix => {
                &[Technology::Processor, Technology::ParticleControlTechnology]
            }
            Technology::WaveFunctionInterference => {
                &[Technology::HighStrengthGlass, Technology::CasimirCrystal]
            }
            Technology::StrangeMatter => &[Technology::MiniatureParticleCollider],
            Technology::VerticalLaunchingSilo => &[Technology::HighStrengthLightweightStructure],
            Technology::QuantumChip => &[
                Technology::InformationMatrix,
                Technology::WaveFunctionInterference,
            ],
            Technology::GravitationalWaveRefraction => &[Technology::StrangeMatter],
            Technology::DysonSphereStressSystem => &[Technology::VerticalLaunchingSilo],
            Technology::PlanetaryIonosphereUtilization => &[Technology::RayReceiver],
            Technology::QuantumPrintingTechnology => &[
                Technology::QuantumChip,
                Technology::GravitationalWaveRefraction,
            ],
            Technology::GravityMatrix => &[
                Technology::QuantumChip,
                Technology::GravitationalWaveRefraction,
            ],
            Technology::DiracInversionMechanism => &[Technology::PlanetaryIonosphereUtilization],
            Technology::ControlledAnnihilationReaction => &[Technology::DiracInversionMechanism],
            Technology::ArtificialStar => &[Technology::ControlledAnnihilationReaction],
            Technology::UniverseMatrix => &[Technology::DiracInversionMechanism],
            Technology::MissionCompleted => &[Technology::UniverseMatrix],
        }
    }
}

impl Default for Technology {
    fn default() -> Self {
        Technology::DysonSphereProgram
    }
}
