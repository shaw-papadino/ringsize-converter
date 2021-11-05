use std::f32::consts::PI;
use std::cmp::PartialEq;
use std::error::Error;
use std::fmt;

fn main() {
    let ring1 = Ring::from(Diameter::new(18.3));
    println!("{:?}", ring1);

    let isoRingSize = ISORingSizeGenerater::generate(&ring1);
    let jcsRingSize = JCSRingSizeGenerater::generate(&ring1);
    println!("{:?}", isoRingSize);
    println!("{:?}", jcsRingSize);
}

#[derive(Debug, Clone)]
struct Diameter {
    size: f32
}

impl Diameter {
    //TODO: エラー処理をかく
    pub fn new(size: f32) -> Self {
        Diameter { size }
    }
}
#[derive(Debug, Clone)]
struct Circumference {
    size: f32
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
        Ring { diameter, circumference }
    }

    fn circumferenceFrom(diameter: &Diameter) -> Circumference {
        Circumference {size: diameter.size * PI}
    }

    fn diameterFrom(circumference: &Circumference) -> Diameter {
        Diameter { size: circumference.size / PI }
    }

}

impl From<Diameter> for Ring {
    fn from(diameter: Diameter) -> Self {
        let circumference = Ring::circumferenceFrom(&diameter);
        Ring {
            diameter,
            circumference
        }
    }
}

impl From<Circumference> for Ring {
    fn from(circumference: Circumference) -> Self {
        let diameter = Ring::diameterFrom(&circumference);
        Ring {
            diameter,
            circumference
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RingSizeDefinition {
    ISO,
    JIS,
    JCS,
    EU
}

#[derive(Debug)]
struct RingSize {
    definition: RingSizeDefinition,
    size: std::string::String,
    circumference: Circumference
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
            circumference: ring.circumference
        }
    }

    fn from(ring_size: &RingSize) -> RingSize {
        let size = (ring_size.circumference.size * 10.0).round() / 10.0;
        RingSize {
            definition: RingSizeDefinition::ISO,
            size: size.to_string(),
            circumference: ring_size.circumference
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
            circumference: ring.circumference
        }
    }

    fn from(ring_size: &RingSize) -> RingSize {
        let size = ((3.0 / PI) * ring_size.circumference.size - 38.0).round();
        RingSize {
            definition: RingSizeDefinition::ISO,
            size: size.to_string(),
            circumference: ring_size.circumference
        }
    }
}

#[derive(Debug)]
struct ConvertError {
    cause: std::string::String,
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConvertError!")
    }
}

impl Error for ConvertError {}

trait RingSizeConverter {
    fn convert(ring_size: RingSize, to: RingSizeDefinition) -> Result<RingSize, ConvertError> {
        if ring_size.definition == to {
            return Ok(ring_size)
        }
        match to {
            RingSizeDefinition::ISO => {
                Ok(ISORingSizeGenerater::generate(&Ring::from(ring_size.circumference)))
            }
            RingSizeDefinition::JCS => {
                Ok(JCSRingSizeGenerater::generate(&Ring::from(ring_size.circumference)))
            }
            _ => Err(ConvertError {cause: String::from("missing RingSizeDefinition")})
        }
    }
}
