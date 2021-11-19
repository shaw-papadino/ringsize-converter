mod error;

use crate::error::ConvertError;
use std::cmp::PartialEq;
use std::error::Error;
use std::f32::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub struct Diameter {
    size: f32,
}

impl Diameter {
    //TODO: エラー処理をかく
    pub fn new(size: f32) -> Self {
        Diameter { size }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circumference {
    size: f32,
}

impl Circumference {
    //TODO: エラー処理をかく
    pub fn new(size: f32) -> Self {
        Circumference { size }
    }
}

#[derive(Debug, Clone)]
pub struct Ring {
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
pub enum RingSizeDefinition {
    ISO,
    JIS,
    JCS,
    EU,
}

#[derive(Debug)]
pub struct RingSize {
    definition: RingSizeDefinition,
    size: std::string::String,
    circumference: Circumference,
}

pub trait RingSizeGenerator {
    fn generate(ring: &Ring) -> RingSize;
    fn from(ring_size: &RingSize) -> RingSize;
}

pub struct ISORingSizeGenerater {}
impl ISORingSizeGenerater {
    pub fn generate(ring: &Ring) -> RingSize {
        let size = (ring.circumference.size * 10.0).round() / 10.0;
        RingSize {
            definition: RingSizeDefinition::ISO,
            size: size.to_string(),
            circumference: ring.circumference,
        }
    }

    pub fn from(ring_size: &RingSize) -> RingSize {
        let size = (ring_size.circumference.size * 10.0).round() / 10.0;
        RingSize {
            definition: RingSizeDefinition::ISO,
            size: size.to_string(),
            circumference: ring_size.circumference,
        }
    }
}

pub struct JCSRingSizeGenerater {}
impl JCSRingSizeGenerater {
    pub fn generate(ring: &Ring) -> RingSize {
        let size = ((3.0 / PI) * ring.circumference.size - 38.0).round();
        RingSize {
            definition: RingSizeDefinition::JCS,
            size: size.to_string(),
            circumference: ring.circumference,
        }
    }

    pub fn from(ring_size: &RingSize) -> RingSize {
        let size = ((3.0 / PI) * ring_size.circumference.size - 38.0).round();
        RingSize {
            definition: RingSizeDefinition::ISO,
            size: size.to_string(),
            circumference: ring_size.circumference,
        }
    }
}

pub struct RingSizeConverter {}
impl RingSizeConverter {
    pub fn convert(
        ring_size: RingSize,
        to: RingSizeDefinition,
    ) -> Result<RingSize, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn ring_from() {
	let size = 18.5;
	let diameter = Diameter::new(size);
	let ring = Ring::from(diameter);
	assert_eq!(ring.circumference, Circumference::new(size * PI));
	assert_eq!(ring.diameter.size, size);

    }
}
