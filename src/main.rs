mod error;

use crate::error::ConvertError;
use std::cmp::PartialEq;
use std::error::Error;
use std::f32::consts::PI;

fn main() {
    let ring1 = Ring::from(Diameter::new(18.5));
    println!("{:?}", ring1);

    let iso_ring_size = ISORingSizeGenerater::generate(&ring1);
    let jcs_ring_size = JCSRingSizeGenerater::generate(&ring1);
    println!("{:?}", iso_ring_size);
    println!("{:?}", jcs_ring_size);
    let converted_jcs_ring_size =
        RingSizeConverter::convert(iso_ring_size, RingSizeDefinition::JCS).unwrap();
    println!("{:?}, {:?}", jcs_ring_size, converted_jcs_ring_size);
}

#[derive(Debug, Clone)]
struct Diameter {
    size: f32,
}

impl Diameter {
    //TODO: エラー処理をかく
    pub fn new(size: f32) -> Self {
        Diameter { size }
    }
}
#[derive(Debug, Clone, Copy)]
struct Circumference {
    size: f32,
}

impl Circumference {
    //TODO: エラー処理をかく
    pub fn new(size: f32) -> Self {
        Circumference { size }
    }
}

#[derive(Debug, Clone)]
struct Ring {
    diameter: Diameter,
    circumference: Circumference,
}

impl Ring {
    pub fn new(diameter: Diameter, circumference: Circumference) -> Self {
        Ring {
            diameter,
            circumference,
        }
    }

    fn circumference_from(diameter: &Diameter) -> Circumference {
        Circumference {
            size: diameter.size * PI,
        }
    }

    fn diameter_from(circumference: &Circumference) -> Diameter {
        Diameter {
            size: circumference.size / PI,
        }
    }
}

impl From<Diameter> for Ring {
    fn from(diameter: Diameter) -> Self {
        let circumference = Ring::circumference_from(&diameter);
        Ring {
            diameter,
            circumference,
        }
    }
}

impl From<Circumference> for Ring {
    fn from(circumference: Circumference) -> Self {
        let diameter = Ring::diameter_from(&circumference);
        Ring {
            diameter,
            circumference,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RingSizeDefinition {
    ISO,
    JIS,
    JCS,
    EU,
}

#[derive(Debug)]
struct RingSize {
    definition: RingSizeDefinition,
    size: std::string::String,
    circumference: Circumference,
}

trait RingSizeGenerator {
    fn generate(ring: &Ring) -> RingSize;
    fn from(ring_size: &RingSize) -> RingSize;
}

struct ISORingSizeGenerater {}
impl RingSizeGenerator for ISORingSizeGenerater {
    fn generate(ring: &Ring) -> RingSize {
        let size = (ring.circumference.size * 10.0).round() / 10.0;
        RingSize {
            definition: RingSizeDefinition::ISO,
            size: size.to_string(),
            circumference: ring.circumference,
        }
    }

    fn from(ring_size: &RingSize) -> RingSize {
        let size = (ring_size.circumference.size * 10.0).round() / 10.0;
        RingSize {
            definition: RingSizeDefinition::ISO,
            size: size.to_string(),
            circumference: ring_size.circumference,
        }
    }
}

struct JCSRingSizeGenerater {}
impl RingSizeGenerator for JCSRingSizeGenerater {
    fn generate(ring: &Ring) -> RingSize {
        let size = ((3.0 / PI) * ring.circumference.size - 38.0).round();
        RingSize {
            definition: RingSizeDefinition::JCS,
            size: size.to_string(),
            circumference: ring.circumference,
        }
    }

    fn from(ring_size: &RingSize) -> RingSize {
        let size = ((3.0 / PI) * ring_size.circumference.size - 38.0).round();
        RingSize {
            definition: RingSizeDefinition::ISO,
            size: size.to_string(),
            circumference: ring_size.circumference,
        }
    }
}

struct RingSizeConverter {}
impl RingSizeConverter {
    fn convert(ring_size: RingSize, to: RingSizeDefinition) -> Result<RingSize, Box<dyn Error>> {
        if ring_size.definition == to {
            return Ok(ring_size);
        }
        match to {
            RingSizeDefinition::ISO => Ok(ISORingSizeGenerater::generate(&Ring::from(
                ring_size.circumference,
            ))),
            RingSizeDefinition::JCS => Ok(JCSRingSizeGenerater::generate(&Ring::from(
                ring_size.circumference,
            ))),
            _ => Err(Box::new(ConvertError {
                cause: String::from("missing RingSizeDefinition"),
            })),
        }
    }
}
