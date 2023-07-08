use std::vec::Vec;
use crate::polynomial::mono::Mononomial;

// structure of a polynomial
pub struct Polynomial<'a> {
    // list of mononomials
    monos: Vec<&'a Mononomial>,
}

impl Polynomial<'_> {
    // check if a polynomial is valid, i. e. all mono should have the same number of variables
    pub fn _check_if_valid(&self) -> bool {
        if self.monos.len() <= 1 {
            true
        } else {
            let variables_num = self.monos[0].get_num_variables();
            for i in 0..self.monos.len() {
                if self.monos[i].get_num_variables() != variables_num {
                    return false;
                }
            }
            true
        }
    }

    pub fn all_with_variable(&self, index: u64) -> Polynomial {
        let mut result = Vec::new();
        for i in self.monos.iter() {
            if i.has_variable(index) {
                result.push(i.clone());
            }
        }
        return Polynomial {
            monos: result
        };
    }

    pub fn all_without_variable(&self, index: u64) -> Polynomial {
        let mut result = Vec::new();
        for i in self.monos.iter() {
            if !(i.has_variable(index)) {
                result.push(i.clone());
            }
        }
        return Polynomial {
            monos: result
        };
    }

    pub fn eval(&self, variables: &Vec<f64>) -> Option<f64> {
        if !self._check_if_valid() {
            println!("Internal Error: The polynomial is invalid!");
            None
        } else {
            let mut sum = 0.0;
            for i in self.monos.iter() {
                sum += i.eval(&variables)?;
            }
            Some(sum)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::polynomial::mono::Mononomial;
    use crate::polynomial::repr::Polynomial;
    use crate::mathlib::_is_inside_epsilon_about;

    #[test]
    fn polynomial_variable_check() {
        let mono1 = Mononomial {
            deg: Vec::from([0, 3, 2]),
            coeff: -0.5,
        };

        let mono2 = Mononomial {
            deg: Vec::from([1, 3, 0]),
            coeff: -0.1,
        };

        let mono3 = Mononomial {
            deg: Vec::from([1, 3, 3]),
            coeff: 0.4,
        };

        let mono4 = Mononomial {
            deg: Vec::from([2, 4, 2]),
            coeff: -1.0,
        };

        let poly = Polynomial {
            monos: Vec::from([&mono1, &mono2, &mono3, &mono4]),
        };

        let variables = Vec::from([5.0, 2.0, 4.0]);

        assert_eq!(poly.all_with_variable(2).eval(&variables).unwrap(), -5440.0);
        assert_eq!(poly.all_without_variable(2).eval(&variables).unwrap(), -4.0);
    }
}