use crate::concentrate::Concentrate;
use crate::element::Element;
use crate::impl_ConcentrateTrait;
use core::hash::{Hash, Hasher};
use std::collections::HashMap;

impl_ConcentrateTrait!(Vec<(Element, i32)>, Element, i32);

#[derive(Clone)]
pub struct Molecule {
    composition: Vec<(Element, i32)>,
    simple_composition: HashMap<Element, i32>,
    charge: i32,
}

pub fn new(composition: Vec<(Element, i32)>, charge: i32) -> Molecule {
    let simple_composition = composition.concentrate();
    Molecule {
        composition,
        simple_composition,
        charge,
    }
}

impl Molecule {
    #[inline(always)]
    pub fn composition(&self) -> &Vec<(Element, i32)> {
        &self.composition
    }

    /// Denotes the individual elements that make up a molecule
    /// Returns a map with as key, an element, and as value, how many atoms of that element are within this molecule.
    pub fn simple_composition(&self) -> &HashMap<Element, i32> {
        &self.simple_composition
    }

    pub fn charge(&self) -> &i32 {
        &self.charge
    }

    pub fn decompose(&self) -> Vec<Element> {
        let mut atoms: Vec<Element> = Vec::new();
        let mut i = 0;
        for (element, quantity) in &self.composition {
            while i < *quantity {
                atoms.push(element.clone());
                i += 1;
            }
            i = 0;
        }

        atoms
    }

    pub fn how_many(&self, find: &Element) -> i32 {
        let mut total: i32 = 0;
        for (element, quantity) in &self.composition {
            if find == element {
                total += quantity;
            }
        }
        total
    }

    /// Returns how many atoms are in total in this molecule
    pub fn n_atoms(&self) -> i32 {
        let mut total: i32 = 0;
        for quantity in self.simple_composition().values() {
            total += quantity;
        }
        total
    }
}

impl std::string::ToString for Molecule {
    fn to_string(&self) -> String {
        self.composition
            .iter()
            .map(|element_quantity| {
                format!(
                    "{}{}",
                    element_quantity.0.symbol(),
                    if element_quantity.1 > 1 {
                        element_quantity.1.to_string()
                    } else {
                        "".to_string()
                    }
                )
            })
            .collect::<String>()
    }
}

impl PartialEq for Molecule {
    fn eq(&self, other: &Self) -> bool {
        self.composition == other.composition
    }
}
impl Eq for Molecule {}

impl Hash for Molecule {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.composition.hash(hasher)
    }
}
