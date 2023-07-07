use std::vec::Vec;

// struct of a mononomial
pub struct Mononomial {
    // degrees of variables
    deg: Vec<u64>,
    // coefficient
    coeff: f64,
}

impl Mononomial {
    // evaluate the mononomial with the input variables
    pub fn eval(&self, variables: Vec<f64>) -> Option<f64> {
        if variables.len() != self.deg.len() {
            println!("The number of the parameters does not match!");
            None
        } else {
            let mut sum = self.coeff;
            for i in 0..variables.len() {
                sum *= variables[i].powi(self.deg[i].try_into().unwrap());
            }
            Some(sum)
        }
    }

    // if the two mononomials have same degree
    pub fn has_same_degree(&self, other: &Mononomial) -> bool {
        if self.deg.len() != other.deg.len() {
            false
        } else {
            for i in 0..self.deg.len() {
                if self.deg[i] != other.deg[i] {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::polynomial::mono::Mononomial;
    use crate::mathlib::_is_inside_epsilon_about;

    #[test]
    fn mononomial_eval() {
        let mono1 = Mononomial {
            deg: Vec::from([1, 3, 2]),
            coeff: -0.5,
        };
        let res1 = mono1.eval(Vec::from([0.3, 0.4, 0.5])).unwrap();
        assert!(_is_inside_epsilon_about(-0.0024, res1, 0.001));

        let mono2 = Mononomial {
            deg: Vec::from([1, 3, 2, 4]),
            coeff: -0.5,
        };
        let res2 = mono2.eval(Vec::from([0.3, 0.4, 0.5]));
        assert_eq!(res2, None);
    }

    #[test]
    fn mononomial_has_same_degree() {
        let mono1 = Mononomial {
            deg: Vec::from([1, 3, 2]),
            coeff: -0.5,
        };

        let mono2 = Mononomial {
            deg: Vec::from([1, 3, 2, 5]),
            coeff: -0.5,
        };

        let mono3 = Mononomial {
            deg: Vec::from([1, 3, 3]),
            coeff: -0.5,
        };

        let mono4 = Mononomial {
            deg: Vec::from([1, 3, 2]),
            coeff: -1.0,
        };

        assert_eq!(mono1.has_same_degree(&mono2), false);
        assert_eq!(mono1.has_same_degree(&mono3), false);
        assert_eq!(mono1.has_same_degree(&mono4), true);
    }
}