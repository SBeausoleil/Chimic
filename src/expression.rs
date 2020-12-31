use crate::errors::IllegalArgument;
use crate::molecule::Molecule;

pub struct Expression {
    compounds: Vec<(Molecule, f32)>,
}

fn new(compounds: Vec<(Molecule, f32)>) -> Result<Expression, IllegalArgument> {
    if compounds.is_empty() {
        return Err(IllegalArgument {
            msg: "Compounds list is empty".to_string(),
        });
    }
    match compounds.iter().find(|&&mol_n| mol_n.1 == 0 as f32) {
        Some(mol_n) => Err(IllegalArgument {
            msg: format!(
                "The molecule {} is listed with zero occurences.",
                mol_n.0.to_string()
            ),
        }),
        None => Ok(Expression { compounds }),
    }
}
