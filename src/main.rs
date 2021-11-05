use std::cmp::{PartialEq};
use std::f32::consts::PI;
use std::fmt;

fn main() {
    let ring1 = Ring::from(Diameter::new(18.6));
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

    pub fn from(diameter: Diameter) -> Self {
        let circumference = Ring::circumferenceFrom(&diameter);
        Ring {
            diameter,
            circumference
        }
    }
    fn circumferenceFrom(diameter: &Diameter) -> Circumference {
        Circumference {size: diameter.size * PI}
    }
    // pub fn from(circumference: Circumference) -> Self {
    //     let diameter = Ring::diameterFrom(circumference);
    //     Ring {
    //         diameter,
    //         circumference
    //     }
    // }

    // fn diameterFrom(circumference: &Circumference) -> Diameter {
    //     Diameter { size: circumference.size / PI }
    // }

}

#[derive(Debug)]
enum RingSizeDefinition {
    ISO,
    JIS,
    JCS,
    EU
}

#[derive(Debug)]
struct RingSize {
    definition: RingSizeDefinition,
    size: f32
}

impl fmt::Debug for RingSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}

trait RingSizeGenerator {
    fn generate(ring: &Ring) -> RingSize;
}

struct ISORingSizeGenerater {}
impl RingSizeGenerator for ISORingSizeGenerater {
    fn generate(ring: &Ring) -> RingSize {
        let size = (ring.circumference.size * 10.0).round() / 10.0;
        RingSize {
            definition: RingSizeDefinition::ISO,
            size
        }
    }
}

struct JCSRingSizeGenerater {}
impl RingSizeGenerator for JCSRingSizeGenerater {
    fn generate(ring: &Ring) -> RingSize {
        let size = ((3.0 / PI) * ring.circumference.size - 38.0).round();
        RingSize {
            definition: RingSizeDefinition::JCS,
            size
        }
    }
}

trait RingSizeConverter {
    fn convert(ring_size: RingSize, to: RingSizeDefinition) -> RingSize;
}
