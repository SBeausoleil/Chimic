use crate::element::Element;
use crate::expression::Expression;
use crate::impl_ConcentrateTrait;
use crate::molecule::Molecule;
use std::collections::HashMap;
use std::fmt;
use std::ptr;

impl_ConcentrateTrait!(Vec<(Molecule, i32)>, Molecule, i32);

pub struct Reaction {
    reactants: Expression,
    products: Expression,
}

pub fn new(reactants: Expression, products: Expression) -> Reaction {
    Reaction {
        reactants,
        products,
    }
}

trait CollapseMolecules {
    fn collapse_molecules(&self) -> HashMap<Element, i32>;
}

impl CollapseMolecules for Vec<(Molecule, i32)> {
    fn collapse_molecules(&self) -> HashMap<Element, i32> {
        let mut makeup: HashMap<Element, i32> = HashMap::new();
        for (molecule, n_molecules) in self {
            for (element, n_elements) in molecule.simple_composition().iter() {
                *makeup.entry(element.clone()).or_insert(0) += n_molecules * n_elements;
            }
        }
        makeup
    }
}

#[inline]
fn is_balanced(reactants: &HashMap<Element, i32>, products: &HashMap<Element, i32>) -> bool {
    products == reactants
}

impl Reaction {
    /*fn find_most_complex_substance(&self) -> (&Molecule, &Vec<(Molecule, i32)>) {
        let mut most_complex: &Molecule = &self.products[0].0;
        let mut side_with_most_complex: &Vec<(Molecule, i32)> = &self.products;
        let mut highest_score: i32 = 0;
        let mut n_atoms: i32;

        for (molecule, _) in &self.reactants {
            n_atoms = molecule.n_atoms();
            if n_atoms > highest_score {
                most_complex = &molecule;
                side_with_most_complex = &self.reactants;
                highest_score = n_atoms;
            }
        }

        for (molecule, _) in &self.products {
            n_atoms = molecule.n_atoms();
            if n_atoms > highest_score {
                most_complex = &molecule;
                side_with_most_complex = &self.products;
                highest_score = n_atoms;
            }
        }

        (most_complex, side_with_most_complex)
    }

    pub fn balance(&self) -> Result<&Self, String> {
        let mut required_atoms: HashMap<Element, i32> = self.products.collapse_molecules();
        let mut provided_atoms: HashMap<Element, i32> = self.reactants.collapse_molecules();

        if !is_balanced(&provided_atoms, &required_atoms) {
            // Attempt to balance
            let complex = self.find_most_complex_substance();
            let less_complex_side = if ptr::eq(&self.reactants, complex.1) {
                &self.products
            } else {
                &self.reactants
            };
        }
        Ok(self)
    }*/
}
