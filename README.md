# Deep Rock Galactic Weekly Deep Dive Information Formatter

Helps to format weekly deep dive info in DRG discord server's
`#deep-dive-discussions` channel.

## Prerequisites

Since this is mostly a small script for personal usage, this requires installing
the [Rust](https://rustup.rs/) toolchain (stable is fine).

## Usage

Copy `input/info.example.json` to `input/info.json` and update the weekly deep
dive information.

```bash
$ cargo run
```

The formatted file will be written to `output/formatted.md`.

## Valid Values

### Primary Objectives

```
"200 Morkite"
"225 Morkite"
"250 Morkite"
"4 Eggs"
"6 Eggs"
"On-Site Refining"
"2 Mini-mules"
"3 Mini-mules"
"7 Aquarqs"
"10 Aquarqs"
"Escort Duty"
"2 Dreadnoughts"
"3 Dreadnoughts"
"Industrial Sabotage"
```

#### Remark on Dreadnoughts

For non-dreadnought primary objectives, it suffices to write e.g.

```json
"primary_objective": "7 Aquarqs"
```

For dreadnoughts, you need to specify their types. For example, if the primary
objective is 2 Dreadnoughts (Twins + Classic), you can write:

```json
"primary_objective": {
    "2 Dreadnoughts": ["Twins, "Hiveguard"]
}
```

### Secondary Objectives

```
"150 Morkite"
"2 Eggs"
"2 Mini-mules"
"Dreadnought"
"Black Box"
```

### Biomes

```
"Sandblasted Corridors"
"Crystalline Caverns"
"Salt Pits"
"Fungus Bogs"
"Radioactive Exclusion Zone"
"Dense Biozone"
"Glacial Strata"
"Hollow Bough"
"Azure Weald"
"Magma Core"
```

### Anomaly

```
"Critical Weakness"
"Low Gravity"
"Rich Atmosphere"
"Volatile Guts"
```

### Warnings

```
"Cave Leech Cluster"
"Elite Threat"
"Exploder Infestation"
"Haunted Cave"
"Lethal Enemies"
"Low Oxygen"
"Mactera Plague"
"Parasites"
"Regenerative Bugs"
"Rival Presence"
"Shield Disruption"
"Swarmageddon"
```

## Example Input

```json
{
    "start": "2023-07-06",
    "end": "2023-07-13",
    "deep_dive": {
        "codename": "High Contact",
        "biome": "Glacial Strata",
        "seed": 3116029769,
        "stages": [
            {
                "primary_objective": "On-Site Refining",
                "secondary_objective": "150 Morkite",
                "anomaly": null,
                "warning": null
            },
            {
                "primary_objective": "200 Morkite",
                "secondary_objective": "2 Eggs",
                "anomaly": null,
                "warning": "Regenerative Bugs"
            },
            {
                "primary_objective": "Industrial Sabotage",
                "secondary_objective": "2 Mini-mules",
                "anomaly": null,
                "warning": "Exploder Infestation"
            }
        ]
    },
    "elite_deep_dive": {
        "codename": "Uncovered Arm",
        "biome": "Magma Core",
        "seed": 1688014532,
        "stages": [
            {
                "primary_objective": "200 Morkite",
                "secondary_objective": "Black Box",
                "anomaly": null,
                "warning": null
            },
            {
                "primary_objective": "10 Aquarqs",
                "secondary_objective": "Black Box",
                "anomaly": null,
                "warning": "Shield Disruption"
            },
            {
                "primary_objective": "3 Mini-mules",
                "secondary_objective": {
                    "Dreadnought": "Hiveguard"
                },
                "anomaly": null,
                "warning": "Mactera Plague"
            }
        ]
    }
}
```

## Example Output

```
Weekly Deep Dives Information for **2023-07-06 to 2023-07-13**.
Deep Dives will reset in **<t:1689246000:R>**

:Deep_Dive: **DEEP DIVE** :Deep_Dive:
Region: **Glacial Strata** | Code Name: **High Contact**
Stage 1: **:refinerywell: On-Site Refinery** + **:morkite: 150 Morkite** | **No Mutator**
Stage 2: **:morkite: 200 Morkite** + **:gegg: 2 Eggs** | **:tothebone: Regenerative Bugs**
Stage 3: **:caretaker: Industrial Sabotage** + **:molly: 2 Mini-mules** | **:tothebone: Exploder Infestation**

:Deep_Dive: **ELITE DEEP DIVE** :Deep_Dive:
Region: **Magma Core** | Code Name: **Uncovered Arm**
Stage 1: **:morkite: 200 Morkite** + **:uplink: Black Box** | **No Mutator**
Stage 2: **:aquarq: 10 Aquarqs** + **:uplink: Black Box** | **:tothebone: Shield Disruption**
Stage 3: **:molly: 3 Mini-mules** + **:dreadegg: Hiveguard** | **:tothebone: Mactera Plague**
```
