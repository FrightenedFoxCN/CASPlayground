use std::vec::Vec;
use crate::polynomial::mono::Mononomial;

// structure of a polynomial
struct Polynomial {
    // list of mononomials
    monos: Vec<Mononomial>,
}