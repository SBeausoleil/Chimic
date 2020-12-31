use core::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Element {
    atomic_number: u8,
    symbol: String,
    name: String,
    n_orbits: u8,
    n_valence_electrons: u8,
    mass: f32,
}

pub fn new(
    atomic_number: u8,
    symbol: String,
    name: String,
    n_orbits: u8,
    n_valence_electrons: u8,
    mass: f32,
) -> Element {
    return Element {
        atomic_number,
        symbol,
        name,
        n_orbits,
        n_valence_electrons,
        mass,
    };
}

impl Element {
    #[inline(always)]
    pub fn atomic_number(&self) -> &u8 {
        &self.atomic_number
    }
    #[inline(always)]
    pub fn n_protons(&self) -> &u8 {
        &self.atomic_number
    }
    #[inline(always)]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    #[inline(always)]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[inline(always)]
    pub fn n_neutrons(&self) -> u16 {
        self.mass as u16 - self.atomic_number as u16
    }
    #[inline(always)]
    pub fn n_orbits(&self) -> &u8 {
        &self.n_orbits
    }
    #[inline(always)]
    pub fn n_valence_electrons(&self) -> &u8 {
        &self.n_valence_electrons
    }
    #[inline(always)]
    pub fn mass(&self) -> &f32 {
        &self.mass
    }

    pub fn is_same_element(&self, other: &Element) -> bool {
        self.atomic_number == other.atomic_number
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}{}", self.name, self.mass as u16)
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.atomic_number == other.atomic_number && self.mass == other.mass
    }
}
impl Eq for Element {}

impl Hash for Element {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        ((self.atomic_number as f32 * self.mass) as i32).hash(hasher);
    }
}
