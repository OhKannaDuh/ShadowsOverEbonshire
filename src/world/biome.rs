use crate::world::Point;
use image::Rgba;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum Biome {
    // Oceans
    FrozenOcean,
    DeepFrozenOcean,
    ColdOcean,
    DeepColdOcean,
    Ocean,
    DeepOcean,
    LukewarmOcean,
    DeepLukewarmOcean,
    WarmOcean,

    // Rivers & Frozen Rivers
    River,
    FrozenRiver,

    // Beaches
    SnowyBeach,
    Beach,
    DesertBeach,

    // Middle Biomes
    SnowyPlains,
    IceSpikes,
    Plains,
    FlowerForest,
    SunflowerPlains,
    Savanna,
    Desert,
    SnowyTaiga,
    Taiga,
    BirchForest,
    OldGrowthBirchForest,
    Jungle,
    SparseJungle,
    OldGrowthSpruceTaiga,
    OldGrowthPineTaiga,
    Forest,
    DarkForest,
    BambooJungle,

    // Badlands
    Badlands,
    ErodedBadlands,
    WoodedBadlands,

    // Plateau
    Meadow,
    CherryGrove,
    PaleGarden,
    SavannaPlateau,

    // Shattered
    WindsweptGravellyHills,
    WindsweptHills,
    WindsweptForest,

    // Peaks
    JaggedPeaks,
    FrozenPeaks,
    StonyPeaks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeirdnessSign {
    Any,
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy)]
pub struct BiomeReq {
    pub temp_level: Option<u32>,
    pub humidity_level: Option<u32>,
    pub continentalness_level: Option<u32>,
    pub erosion_level: Option<u32>,
    pub pv_level: Option<u32>,
    pub weirdness: WeirdnessSign,
}

const ANY: BiomeReq = BiomeReq {
    temp_level: None,
    humidity_level: None,
    continentalness_level: None,
    erosion_level: None,
    pv_level: None,
    weirdness: WeirdnessSign::Any,
};

impl Biome {
    pub fn requirements(&self) -> BiomeReq {
        match self {
            // -----------------------
            // Oceans (non-inland). Only depend on T + "Ocean vs Deep Ocean".
            // Continentalness levels you used: 0=Deep Ocean, 1=Ocean, 2=Coast, 3..5=Inland
            Biome::FrozenOcean => BiomeReq {
                temp_level: Some(0),
                continentalness_level: Some(1),
                ..ANY
            },
            Biome::DeepFrozenOcean => BiomeReq {
                temp_level: Some(0),
                continentalness_level: Some(0),
                ..ANY
            },
            Biome::ColdOcean => BiomeReq {
                temp_level: Some(1),
                continentalness_level: Some(1),
                ..ANY
            },
            Biome::DeepColdOcean => BiomeReq {
                temp_level: Some(1),
                continentalness_level: Some(0),
                ..ANY
            },
            Biome::Ocean => BiomeReq {
                temp_level: Some(2),
                continentalness_level: Some(1),
                ..ANY
            },
            Biome::DeepOcean => BiomeReq {
                temp_level: Some(2),
                continentalness_level: Some(0),
                ..ANY
            },
            Biome::LukewarmOcean => BiomeReq {
                temp_level: Some(3),
                continentalness_level: Some(1),
                ..ANY
            },
            Biome::DeepLukewarmOcean => BiomeReq {
                temp_level: Some(3),
                continentalness_level: Some(0),
                ..ANY
            },
            Biome::WarmOcean => BiomeReq {
                temp_level: Some(4),
                continentalness_level: Some(1),
                ..ANY
            },

            // -----------------------
            // Rivers: happen in PV = Valleys (PV=0). Frozen if T=0; else River.
            // They occur across coast/near/mid in the table; we leave C/E=Any and let PV dominate.
            Biome::FrozenRiver => BiomeReq {
                temp_level: Some(0),
                pv_level: Some(0),
                ..ANY
            },
            Biome::River => BiomeReq {
                temp_level: None,
                pv_level: Some(0),
                ..ANY
            },

            // -----------------------
            // Beaches (Coast = C=2), chosen by Temperature only.
            Biome::SnowyBeach => BiomeReq {
                temp_level: Some(0),
                continentalness_level: Some(2),
                ..ANY
            },
            Biome::Beach => BiomeReq {
                temp_level: Some(2),
                continentalness_level: Some(2),
                ..ANY
            }, // T=1..3 → Beach; pick T=2 mid
            Biome::DesertBeach => BiomeReq {
                temp_level: Some(4),
                continentalness_level: Some(2),
                ..ANY
            },

            // -----------------------
            // Middle biomes (inland): selected by T/H and sometimes W sign.
            // Inland C is Near/Mid/Far (3..5). We leave C/E/PV = Any so your nearest-distance fallback can resolve.
            Biome::SnowyPlains => BiomeReq {
                temp_level: Some(0),
                humidity_level: Some(0),
                weirdness: WeirdnessSign::Any,
                ..ANY
            },
            Biome::IceSpikes => BiomeReq {
                temp_level: Some(0),
                humidity_level: Some(0),
                weirdness: WeirdnessSign::Positive,
                ..ANY
            },
            Biome::Plains => BiomeReq {
                temp_level: Some(1),
                humidity_level: Some(1),
                ..ANY
            },
            Biome::FlowerForest => BiomeReq {
                temp_level: Some(2),
                humidity_level: Some(0),
                weirdness: WeirdnessSign::Negative,
                ..ANY
            },
            Biome::SunflowerPlains => BiomeReq {
                temp_level: Some(2),
                humidity_level: Some(0),
                weirdness: WeirdnessSign::Positive,
                ..ANY
            },
            Biome::Savanna => BiomeReq {
                temp_level: Some(3),
                humidity_level: Some(0),
                ..ANY
            },
            Biome::Desert => BiomeReq {
                temp_level: Some(4),
                humidity_level: Some(0),
                ..ANY
            },

            Biome::SnowyTaiga => BiomeReq {
                temp_level: Some(0),
                humidity_level: Some(2),
                weirdness: WeirdnessSign::Positive,
                ..ANY
            },
            Biome::Taiga => BiomeReq {
                temp_level: Some(1),
                humidity_level: Some(3),
                ..ANY
            },

            Biome::BirchForest => BiomeReq {
                temp_level: Some(2),
                humidity_level: Some(3),
                weirdness: WeirdnessSign::Negative,
                ..ANY
            },
            Biome::OldGrowthBirchForest => BiomeReq {
                temp_level: Some(2),
                humidity_level: Some(3),
                weirdness: WeirdnessSign::Positive,
                ..ANY
            },

            Biome::Jungle => BiomeReq {
                temp_level: Some(3),
                humidity_level: Some(3),
                weirdness: WeirdnessSign::Negative,
                ..ANY
            },
            Biome::SparseJungle => BiomeReq {
                temp_level: Some(3),
                humidity_level: Some(3),
                weirdness: WeirdnessSign::Positive,
                ..ANY
            },

            Biome::OldGrowthSpruceTaiga => BiomeReq {
                temp_level: Some(4),
                humidity_level: Some(4),
                weirdness: WeirdnessSign::Negative,
                ..ANY
            },
            Biome::OldGrowthPineTaiga => BiomeReq {
                temp_level: Some(4),
                humidity_level: Some(4),
                weirdness: WeirdnessSign::Positive,
                ..ANY
            },

            Biome::Forest => BiomeReq {
                temp_level: Some(2),
                humidity_level: Some(2),
                ..ANY
            }, // (used in middle/plateau tables)
            Biome::DarkForest => BiomeReq {
                temp_level: Some(3),
                humidity_level: Some(4),
                ..ANY
            },
            Biome::BambooJungle => BiomeReq {
                temp_level: Some(3),
                humidity_level: Some(4),
                weirdness: WeirdnessSign::Positive,
                ..ANY
            },

            // -----------------------
            // Badlands group: T=4, chosen by H + W sign.
            Biome::Badlands => BiomeReq {
                temp_level: Some(4),
                humidity_level: Some(2),
                ..ANY
            },
            Biome::ErodedBadlands => BiomeReq {
                temp_level: Some(4),
                humidity_level: Some(0),
                weirdness: WeirdnessSign::Positive,
                ..ANY
            },
            Biome::WoodedBadlands => BiomeReq {
                temp_level: Some(4),
                humidity_level: Some(3),
                ..ANY
            },

            // -----------------------
            // Plateau biomes: inland high terrain w/ moderate erosion.
            // The table uses Mid/High PV and E≈1..2 depending on column; we set PV=Mid (2) as a strong hint.
            Biome::Meadow => BiomeReq {
                temp_level: Some(2),
                humidity_level: Some(1),
                pv_level: Some(2),
                ..ANY
            },
            Biome::CherryGrove => BiomeReq {
                temp_level: Some(2),
                humidity_level: Some(1),
                pv_level: Some(2),
                weirdness: WeirdnessSign::Positive,
                ..ANY
            },
            Biome::PaleGarden => BiomeReq {
                temp_level: Some(4),
                humidity_level: Some(4),
                pv_level: Some(2),
                ..ANY
            }, // per table cell
            Biome::SavannaPlateau => BiomeReq {
                temp_level: Some(3),
                humidity_level: Some(0),
                pv_level: Some(2),
                ..ANY
            },

            // -----------------------
            // Shattered biomes: inland, high erosion (E high). We set E=5 as the representative;
            // your nearest-distance fallback will also let E=6 pick these.
            Biome::WindsweptGravellyHills => BiomeReq {
                temp_level: Some(0),
                humidity_level: Some(0),
                erosion_level: Some(5),
                ..ANY
            },
            Biome::WindsweptHills => BiomeReq {
                temp_level: Some(2),
                humidity_level: Some(2),
                erosion_level: Some(5),
                ..ANY
            },
            Biome::WindsweptForest => BiomeReq {
                temp_level: Some(2),
                humidity_level: Some(3),
                erosion_level: Some(5),
                ..ANY
            },

            // -----------------------
            // Peaks group: PV = Peaks (4), low erosion (E≈0..1), with temp and weirdness steering variants.
            Biome::JaggedPeaks => BiomeReq {
                temp_level: Some(1),
                pv_level: Some(4),
                erosion_level: Some(0),
                weirdness: WeirdnessSign::Negative,
                ..ANY
            },
            Biome::FrozenPeaks => BiomeReq {
                temp_level: Some(1),
                pv_level: Some(4),
                erosion_level: Some(0),
                weirdness: WeirdnessSign::Positive,
                ..ANY
            },
            Biome::StonyPeaks => BiomeReq {
                temp_level: Some(3),
                pv_level: Some(4),
                erosion_level: Some(0),
                ..ANY
            },
        }
    }

    pub fn get_color(&self) -> Rgba<u8> {
        match self {
            // Oceans
            // Biome::FrozenOcean => Rgba([180, 220, 255, 255]),
            // Biome::DeepFrozenOcean => Rgba([100, 160, 200, 255]),
            // Biome::ColdOcean => Rgba([80, 140, 200, 255]),
            // Biome::DeepColdOcean => Rgba([50, 100, 160, 255]),
            // Biome::Ocean => Rgba([0, 105, 148, 255]),
            // Biome::DeepOcean => Rgba([0, 70, 110, 255]),
            // Biome::LukewarmOcean => Rgba([0, 130, 160, 255]),
            // Biome::DeepLukewarmOcean => Rgba([0, 90, 120, 255]),
            // Biome::WarmOcean => Rgba([0, 150, 180, 255]),
            Biome::FrozenOcean => Rgba([0, 0, 255, 255]),
            Biome::DeepFrozenOcean => Rgba([0, 0, 255, 255]),
            Biome::ColdOcean => Rgba([0, 0, 255, 255]),
            Biome::DeepColdOcean => Rgba([0, 0, 255, 255]),
            Biome::Ocean => Rgba([0, 0, 255, 255]),
            Biome::DeepOcean => Rgba([0, 0, 255, 255]),
            Biome::LukewarmOcean => Rgba([0, 0, 255, 255]),
            Biome::DeepLukewarmOcean => Rgba([0, 0, 255, 255]),
            Biome::WarmOcean => Rgba([0, 0, 255, 255]),

            // Rivers
            // Biome::River => Rgba([30, 144, 255, 255]), // dodger blue
            // Biome::FrozenRiver => Rgba([200, 230, 255, 255]),
            Biome::River => Rgba([0, 0, 255, 255]),
            Biome::FrozenRiver => Rgba([0, 0, 255, 255]),

            // Beaches
            Biome::SnowyBeach => Rgba([240, 240, 255, 255]), // icy white
            Biome::Beach => Rgba([238, 214, 175, 255]),      // sand
            Biome::DesertBeach => Rgba([237, 201, 175, 255]), // sandy beige

            // Middle biomes
            Biome::SnowyPlains => Rgba([255, 255, 255, 255]),
            Biome::IceSpikes => Rgba([200, 240, 255, 255]),
            Biome::Plains => Rgba([124, 252, 0, 255]),
            Biome::FlowerForest => Rgba([205, 133, 63, 255]),
            Biome::SunflowerPlains => Rgba([255, 215, 0, 255]),
            Biome::Savanna => Rgba([189, 183, 107, 255]),
            Biome::Desert => Rgba([237, 201, 175, 255]),
            Biome::SnowyTaiga => Rgba([175, 238, 238, 255]),
            Biome::Taiga => Rgba([34, 139, 34, 255]),
            Biome::BirchForest => Rgba([152, 251, 152, 255]),
            Biome::OldGrowthBirchForest => Rgba([143, 188, 143, 255]),
            Biome::Jungle => Rgba([0, 100, 0, 255]),
            Biome::SparseJungle => Rgba([60, 179, 113, 255]),
            Biome::OldGrowthSpruceTaiga => Rgba([0, 128, 0, 255]),
            Biome::OldGrowthPineTaiga => Rgba([46, 139, 87, 255]),
            Biome::Forest => Rgba([34, 139, 34, 255]),
            Biome::DarkForest => Rgba([0, 80, 0, 255]),
            Biome::BambooJungle => Rgba([107, 142, 35, 255]),

            // Badlands
            Biome::Badlands => Rgba([210, 105, 30, 255]),
            Biome::ErodedBadlands => Rgba([233, 150, 122, 255]),
            Biome::WoodedBadlands => Rgba([139, 69, 19, 255]),

            // Plateau
            Biome::Meadow => Rgba([124, 252, 0, 255]),
            Biome::CherryGrove => Rgba([255, 182, 193, 255]),
            Biome::PaleGarden => Rgba([255, 239, 213, 255]),
            Biome::SavannaPlateau => Rgba([189, 183, 107, 255]),

            // Shattered
            Biome::WindsweptGravellyHills => Rgba([169, 169, 169, 255]),
            Biome::WindsweptHills => Rgba([85, 107, 47, 255]),
            Biome::WindsweptForest => Rgba([34, 139, 34, 255]),

            // Peaks
            Biome::JaggedPeaks => Rgba([220, 220, 220, 255]),
            Biome::FrozenPeaks => Rgba([245, 245, 255, 255]),
            Biome::StonyPeaks => Rgba([112, 128, 144, 255]),
        }
    }
}

pub fn pick_biome(point: &Point) -> Biome {
    use WeirdnessSign::*;

    let mut best: Option<(Biome, f32)> = None;

    for biome in Biome::iter() {
        let req = biome.requirements();

        // For biomes with "any" in a parameter, use the point's value in the distance calc
        let t_req = req.temp_level.unwrap_or(point.temperature_level);
        let h_req = req.humidity_level.unwrap_or(point.humidity_level);
        let c_req = req
            .continentalness_level
            .unwrap_or(point.continentalness_level);
        let e_req = req.erosion_level.unwrap_or(point.erosion_level);
        let pv_req = req.pv_level.unwrap_or(point.peaks_and_valleys_level);

        // weirdness penalty
        let mut weird_penalty = 0.0;
        match req.weirdness {
            Positive if !point.is_weird => weird_penalty = 0.5, // tweakable
            Negative if point.is_weird => weird_penalty = 0.5,
            _ => {}
        }

        // Euclidean distance in discrete space
        let dist = ((t_req as f32 - point.temperature_level as f32).powi(2)
            + (h_req as f32 - point.humidity_level as f32).powi(2)
            + (c_req as f32 - point.continentalness_level as f32).powi(2)
            + (e_req as f32 - point.erosion_level as f32).powi(2)
            + (pv_req as f32 - point.peaks_and_valleys_level as f32).powi(2))
        .sqrt()
            + weird_penalty;

        match best {
            Some((_, best_dist)) if dist >= best_dist => {}
            _ => best = Some((biome, dist)),
        }
    }

    best.map(|(b, _)| b).unwrap()
}
