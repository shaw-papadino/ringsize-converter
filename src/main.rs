use std::cmp::{PartialEq};

fn main() {
    let ring1 = ISORing::from(48.7);
    let ring2 = ISORing::from(49.49);
    if ring1 == ring2 {
        println!("eq")
    } else {
        println!("not eq")
    }
}

// ISO国際規格とJCS規格の変換処理
#[derive(Debug, Clone)]
struct ISORing {
    size: String,
    circumference: f32
}

impl ISORing {
    // NOTE: Intoを使ってStringと&strを両方受け取れるようにしている
    pub fn new(size: impl Into<String>, circumference: f32) -> Self {
        Self {
            size: size.into(),
            circumference
        }
    }

    pub fn from(circumference: f32) -> Self {
        let size = circumference.round();
        ISORing::new(
            size.to_string(),
            circumference
        )
    }
}

impl PartialEq for ISORing {
    fn eq(&self, other: &ISORing) -> bool {
        println!("{} == {}", self.size, other.size);
        &self.size == &other.size
    }

    fn ne(&self, other: &ISORing) -> bool {
        &self.size != &other.size
    }
}


// TODO: JCS規格の実装
#[derive(Debug, Clone)]
struct JCSRing {
    size: String,
    circumference: f32
}

impl JCSRing {
    // NOTE: Intoを使ってStringと&strを両方受け取れるようにしている
    pub fn new(size: impl Into<String>, circumference: f32) -> Self {
        Self {
            size: size.into(),
            circumference
        }
    }

    pub fn from(circumference: f32) -> Self {
        let size = circumference.round();
        JCSRing::new(
            size.to_string(),
            circumference
        )
    }
}

impl PartialEq for JCSRing {
    fn eq(&self, other: &JCSRing) -> bool {
        println!("{} == {}", self.size, other.size);
        &self.size == &other.size
    }

    fn ne(&self, other: &JCSRing) -> bool {
        &self.size != &other.size
    }
}