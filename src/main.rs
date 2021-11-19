use ringsize_converter::{
    Diameter, ISORingSizeGenerater, JCSRingSizeGenerater, Ring, RingSizeConverter,
    RingSizeDefinition,
};

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
