use std::path::Path;
use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use miette::IntoDiagnostic;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::debug;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepDivesInfo {
    #[serde(flatten)]
    pub date_range: DateRange,
    pub deep_dive: DeepDive,
    pub elite_deep_dive: DeepDive,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DateRange {
    #[serde(
        serialize_with = "as_naive_date_without_time",
        deserialize_with = "from_naive_date_with_known_time"
    )]
    pub start: NaiveDateTime,
    #[serde(
        serialize_with = "as_naive_date_without_time",
        deserialize_with = "from_naive_date_with_known_time"
    )]
    pub end: NaiveDateTime,
}

fn as_naive_date_without_time<S>(v: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&v.date().to_string())
}

pub fn from_naive_date_with_known_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    String::deserialize(deserializer)
        .and_then(|s| {
            NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|e| Error::custom(e.to_string()))
        })
        .map(|date| NaiveDateTime::new(date, NaiveTime::from_hms_opt(11, 0, 0).unwrap()))
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepDive {
    pub codename: String,
    pub biome: Biome,
    pub seed: u64,
    pub stages: [DeepDiveStage; 3],
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepDiveStage {
    pub primary_objective: PrimaryObjective,
    pub secondary_objective: SecondaryObjective,
    pub anomaly: Option<Anomaly>,
    pub warning: Option<Warning>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PrimaryObjective {
    #[serde(rename = "200 Morkite")]
    MiningExpedition200,
    #[serde(rename = "225 Morkite")]
    MiningExpedition225,
    #[serde(rename = "250 Morkite")]
    MiningExpedition250,
    #[serde(rename = "4 Eggs")]
    Egg4,
    #[serde(rename = "6 Eggs")]
    Egg6,
    #[serde(rename = "On-Site Refining")]
    OnSiteRefining,
    #[serde(rename = "2 Mini-mules")]
    MiniMules2,
    #[serde(rename = "3 Mini-mules")]
    MiniMules3,
    #[serde(rename = "7 Aquarqs")]
    PointExtraction7,
    #[serde(rename = "10 Aquarqs")]
    PointExtraction10,
    #[serde(rename = "Escort Duty")]
    EscortDuty,
    #[serde(rename = "2 Dreadnoughts")]
    Dreadnought2([Dreadnought; 2]),
    #[serde(rename = "3 Dreadnoughts")]
    Dreadnought3([Dreadnought; 3]),
    #[serde(rename = "Industrial Sabotage")]
    IndustrialSabotage,
}

impl PrimaryObjective {
    pub fn emoji_repr(&self) -> String {
        match self {
            PrimaryObjective::MiningExpedition200 => ":morkite: 200 Morkite".to_string(),
            PrimaryObjective::MiningExpedition225 => ":morkite: 225 Morkite".to_string(),
            PrimaryObjective::MiningExpedition250 => ":morkite: 250 Morkite".to_string(),
            PrimaryObjective::Egg4 => ":gegg: 4 Eggs".to_string(),
            PrimaryObjective::Egg6 => ":gegg: 6 Eggs".to_string(),
            PrimaryObjective::OnSiteRefining => ":refinerywell: On-Site Refinery".to_string(),
            PrimaryObjective::MiniMules2 => ":molly: 2 Mini-mules".to_string(),
            PrimaryObjective::MiniMules3 => ":molly: 3 Mini-mules".to_string(),
            PrimaryObjective::PointExtraction7 => ":aquarq: 7 Aquarqs".to_string(),
            PrimaryObjective::PointExtraction10 => ":aquarq: 10 Aquarqs".to_string(),
            PrimaryObjective::EscortDuty => ":drill: Escort Duty".to_string(),
            PrimaryObjective::Dreadnought2([dread1, dread2]) => {
                let dread1 = dread1.as_str();
                let dread2 = dread2.as_str();
                format!(":dreadegg: 2 Dreadnoughts ({dread1} + {dread2})")
            }
            PrimaryObjective::Dreadnought3([dread1, dread2, dread3]) => {
                let dread1 = dread1.as_str();
                let dread2 = dread2.as_str();
                let dread3 = dread3.as_str();
                format!(":dreadegg: 3 Dreadnoughts ({dread1} + {dread2} + {dread3})")
            }
            PrimaryObjective::IndustrialSabotage => ":caretaker: Industrial Sabotage".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Dreadnought {
    Classic,
    Twins,
    Hiveguard,
}

impl Dreadnought {
    pub fn as_str(&self) -> &'static str {
        match self {
            Dreadnought::Classic => "Classic",
            Dreadnought::Twins => "Twins",
            Dreadnought::Hiveguard => "Hiveguard",
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SecondaryObjective {
    #[serde(rename = "150 Morkite")]
    Mining150,
    #[serde(rename = "2 Eggs")]
    Egg,
    #[serde(rename = "2 Mini-mules")]
    MiniMules,
    #[serde(rename = "Dreadnought")]
    Dreadnought(Dreadnought),
    #[serde(rename = "Black Box")]
    BlackBox,
}

impl SecondaryObjective {
    pub fn emoji_repr(&self) -> &'static str {
        match self {
            SecondaryObjective::Mining150 => ":morkite: 150 Morkite",
            SecondaryObjective::Egg => ":gegg: 2 Eggs",
            SecondaryObjective::MiniMules => ":molly: 2 Mini-mules",
            SecondaryObjective::Dreadnought(dread) => match dread {
                Dreadnought::Classic => ":dreadegg: Classic",
                Dreadnought::Twins => ":dreadegg: Twins",
                Dreadnought::Hiveguard => ":dreadegg: Hiveguard",
            },
            SecondaryObjective::BlackBox => ":uplink: Black Box",
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Biome {
    #[serde(rename = "Sandblasted Corridors")]
    SandblastedCorridors,
    #[serde(rename = "Crystalline Caverns")]
    CrystallineCaverns,
    #[serde(rename = "Salt Pits")]
    SaltPits,
    #[serde(rename = "Fungus Bogs")]
    FungusBogs,
    #[serde(rename = "Radioactive Exclusion Zone")]
    RadioactiveExclusionZone,
    #[serde(rename = "Dense Biozone")]
    DenseBiozone,
    #[serde(rename = "Glacial Strata")]
    GlacialStrata,
    #[serde(rename = "Hollow Bough")]
    HollowBough,
    #[serde(rename = "Azure Weald")]
    AzureWeald,
    #[serde(rename = "Magma Core")]
    MagmaCore,
}

impl Biome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Biome::SandblastedCorridors => "Sandblasted Corridors",
            Biome::CrystallineCaverns => "Crystalline Caverns",
            Biome::SaltPits => "Salt Pits",
            Biome::FungusBogs => "Fungus Bogs",
            Biome::RadioactiveExclusionZone => "Radioactive Exclusion Zone",
            Biome::DenseBiozone => "Dense Biozone",
            Biome::GlacialStrata => "Glacial Strata",
            Biome::HollowBough => "Hollow Bough",
            Biome::AzureWeald => "Azure Weald",
            Biome::MagmaCore => "Magma Core",
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Anomaly {
    #[serde(rename = "Critical Weakness")]
    CriticalWeakness,
    #[serde(rename = "Low Gravity")]
    LowGravity,
    #[serde(rename = "Rich Atmosphere")]
    RichAtmosphere,
    #[serde(rename = "Volatile Guts")]
    VolatileGuts,
}

impl Anomaly {
    pub fn emoji_repr(&self) -> String {
        format!(
            ":rocknstone: {}",
            match self {
                Anomaly::CriticalWeakness => "Critical Weakness",
                Anomaly::LowGravity => "Low Gravity",
                Anomaly::RichAtmosphere => "Rich Atmosphere",
                Anomaly::VolatileGuts => "Volatile Guts",
            }
        )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Warning {
    #[serde(rename = "Cave Leech Cluster")]
    CaveLeechCluster,
    #[serde(rename = "Elite Threat")]
    EliteThreat,
    #[serde(rename = "Exploder Infestation")]
    ExploderInfestation,
    #[serde(rename = "Haunted Cave")]
    HauntedCave,
    #[serde(rename = "Lethal Enemies")]
    LethalEnemies,
    #[serde(rename = "Low Oxygen")]
    LowOxygen,
    #[serde(rename = "Mactera Plague")]
    MacteraPlague,
    #[serde(rename = "Parasites")]
    Parasites,
    #[serde(rename = "Regenerative Bugs")]
    RegenerativeBugs,
    #[serde(rename = "Rival Presence")]
    RivalPresence,
    #[serde(rename = "Shield Disruption")]
    ShieldDisruption,
    #[serde(rename = "Swarmageddon")]
    Swarmageddon,
}

impl Warning {
    pub fn emoji_repr(&self) -> String {
        format!(
            ":tothebone: {}",
            match self {
                Warning::CaveLeechCluster => "Cave Leech Cluster",
                Warning::EliteThreat => "Elite Threat",
                Warning::ExploderInfestation => "Exploder Infestation",
                Warning::HauntedCave => "Haunted Cave",
                Warning::LethalEnemies => "Lethal Enemies",
                Warning::LowOxygen => "Low Oxygen",
                Warning::MacteraPlague => "Mactera Plague",
                Warning::Parasites => "Parasites",
                Warning::RegenerativeBugs => "Regenerative Bugs",
                Warning::RivalPresence => "Rival Presence",
                Warning::ShieldDisruption => "Shield Disruption",
                Warning::Swarmageddon => "Swarmageddon",
            }
        )
    }
}

fn main() -> miette::Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(false)
                .unicode(false)
                .context_lines(2)
                .tab_width(4)
                .build(),
        )
    }))
    .unwrap();
    setup_logging();

    let mut info_example_json = std::env::current_dir().into_diagnostic()?;
    info_example_json.push("input/info.example.json");

    if !info_example_json.exists() {
        generate_example_json(info_example_json.as_path())?;
    }

    let mut info_json = std::env::current_dir().into_diagnostic()?;
    info_json.push("input/info.json");
    let info_json = std::fs::read_to_string(info_json.as_path()).into_diagnostic()?;
    let dives_info: DeepDivesInfo = serde_json::from_str(&info_json).into_diagnostic()?;

    debug!(
        "info_json:\n{}",
        serde_json::to_string_pretty(&dives_info).into_diagnostic()?
    );

    let emojified_info = discord_emojify_info(&dives_info);

    debug!("dives_info:\n{}", emojified_info);

    let mut formatted_file = std::env::current_dir().into_diagnostic()?;
    formatted_file.push("output/formatted.md");

    std::fs::write(formatted_file, &emojified_info).into_diagnostic()?;

    Ok(())
}

fn discord_emojify_info(info: &DeepDivesInfo) -> String {
    let DeepDivesInfo {
        date_range: DateRange { start, end },
        deep_dive,
        elite_deep_dive,
    } = info;
    let dd_region_codename = region_codename_fmt(&deep_dive);
    let dd_stages = stages_fmt(&deep_dive.stages);
    let dd_info = format!(
        "\
        :Deep_Dive: **DEEP DIVE** :Deep_Dive:\n\
        {dd_region_codename}\n\
        {dd_stages}\
        "
    );

    let edd_region_codename = region_codename_fmt(&elite_deep_dive);
    let edd_stages = stages_fmt(&elite_deep_dive.stages);
    let edd_info = format!(
        "\
        :Deep_Dive: **ELITE DEEP DIVE** :Deep_Dive:\n\
        {edd_region_codename}\n\
        {edd_stages}\
        "
    );

    let unix_timestamp = end.timestamp();
    let start = start.date();
    let end = end.date();
    let dives_info = format!(
        "\
        Weekly Deep Dives information for **{start} to {end}**.\n\
        Deep Dives will reset in **<t:{unix_timestamp}:R>**\n\n\
        {dd_info}\n\
        {edd_info}\n\
        "
    );

    return dives_info;
}

fn objectives_fmt(
    DeepDiveStage {
        ref primary_objective,
        ref secondary_objective,
        ..
    }: &DeepDiveStage,
) -> String {
    let primary = primary_objective.emoji_repr();
    let secondary = secondary_objective.emoji_repr();
    format!("**{primary}** + **{secondary}**")
}

fn mutator_fmt(
    DeepDiveStage {
        ref anomaly,
        ref warning,
        ..
    }: &DeepDiveStage,
) -> String {
    match (anomaly, warning) {
        (Some(anomaly), Some(warning)) => {
            let anomaly = anomaly.emoji_repr();
            let warning = warning.emoji_repr();
            format!("**{anomaly}** **{warning}**")
        }
        (Some(anomaly), None) => {
            let anomaly = anomaly.emoji_repr();
            format!("**{anomaly}**")
        }
        (None, Some(warning)) => {
            let warning = warning.emoji_repr();
            format!("**{warning}**")
        }
        (None, None) => {
            format!("**No Mutator**")
        }
    }
}

fn region_codename_fmt(
    DeepDive {
        ref biome,
        ref codename,
        ..
    }: &DeepDive,
) -> String {
    let biome = biome.as_str();
    format!("Region: **{biome}** | Code Name: **{codename}**")
}

fn stages_fmt([ref stage1, ref stage2, ref stage3]: &[DeepDiveStage; 3]) -> String {
    let stage1_objectives = objectives_fmt(stage1);
    let stage1_mutator_repr = mutator_fmt(stage1);
    let stage2_objectives = objectives_fmt(stage2);
    let stage2_mutator_repr = mutator_fmt(stage2);
    let stage3_objectives = objectives_fmt(stage3);
    let stage3_mutator_repr = mutator_fmt(stage3);

    format!(
        "\
        Stage 1: {stage1_objectives} | {stage1_mutator_repr}\n\
        Stage 2: {stage2_objectives} | {stage2_mutator_repr}\n\
        Stage 3: {stage3_objectives} | {stage3_mutator_repr}\n\
        "
    )
}

fn generate_example_json(info_example_json: &Path) -> miette::Result<()> {
    let example = DeepDivesInfo {
        date_range: DateRange {
            start: NaiveDateTime::from_str("2023-07-06T11:00:00").unwrap(),
            end: NaiveDateTime::from_str("2023-07-13T11:00:00").unwrap(),
        },
        deep_dive: DeepDive {
            codename: "High Contact".to_string(),
            biome: Biome::GlacialStrata,
            seed: 3116029769,
            stages: [
                DeepDiveStage {
                    primary_objective: PrimaryObjective::OnSiteRefining,
                    secondary_objective: SecondaryObjective::Mining150,
                    anomaly: None,
                    warning: None,
                },
                DeepDiveStage {
                    primary_objective: PrimaryObjective::MiningExpedition200,
                    secondary_objective: SecondaryObjective::Egg,
                    anomaly: None,
                    warning: Some(Warning::RegenerativeBugs),
                },
                DeepDiveStage {
                    primary_objective: PrimaryObjective::IndustrialSabotage,
                    secondary_objective: SecondaryObjective::MiniMules,
                    anomaly: None,
                    warning: Some(Warning::ExploderInfestation),
                },
            ],
        },
        elite_deep_dive: DeepDive {
            codename: "Uncovered Arm".to_string(),
            biome: Biome::MagmaCore,
            seed: 1688014532,
            stages: [
                DeepDiveStage {
                    primary_objective: PrimaryObjective::MiningExpedition200,
                    secondary_objective: SecondaryObjective::BlackBox,
                    anomaly: None,
                    warning: None,
                },
                DeepDiveStage {
                    primary_objective: PrimaryObjective::PointExtraction10,
                    secondary_objective: SecondaryObjective::BlackBox,
                    anomaly: None,
                    warning: Some(Warning::ShieldDisruption),
                },
                DeepDiveStage {
                    primary_objective: PrimaryObjective::MiniMules3,
                    secondary_objective: SecondaryObjective::Dreadnought(Dreadnought::Hiveguard),
                    anomaly: None,
                    warning: Some(Warning::MacteraPlague),
                },
            ],
        },
    };

    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    example.serialize(&mut ser).into_diagnostic()?;
    let example = String::from_utf8_lossy(&buf);

    std::fs::write(info_example_json, &*example).into_diagnostic()?;

    Ok(())
}

fn setup_logging() {
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer()
        .compact()
        .with_level(true)
        .with_target(true)
        .without_time();
    let filter_layer = EnvFilter::from_env("RUST_LOG");
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
