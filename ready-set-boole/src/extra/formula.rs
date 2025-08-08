/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   formula.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/07 13:11:26 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/08 19:07:39 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// Symbol Mathematical
// equivalent
// Description
// 0 ⊥ false
// 1 ⊤ true
// ! ¬ Negation
// & ∧ Conjunction
// | ∨ Disjunction
// ˆ ⊕ Exclusive disjunction
// > ⇒ Material condition
// = ⇔ Logical equivalence

#[derive(Debug, PartialEq, Clone)]
pub enum Formula {
    False,
    True,
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Xor(Box<Formula>, Box<Formula>),
    Implication(Box<Formula>, Box<Formula>),
    Equivalence(Box<Formula>, Box<Formula>),
}

use std::fmt;

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::False => write!(f, "False"),
            Formula::True => write!(f, "True"),
            Formula::Not(inner) => write!(f, "not {}", inner),
            Formula::And(left, right) => write!(f, "({} and {})", left, right),
            Formula::Or(left, right) => write!(f, "({} or {})", left, right),
            Formula::Xor(left, right) => write!(f, "({} xor {})", left, right),
            Formula::Implication(left, right) => write!(f, "({} => {})", left, right),
            Formula::Equivalence(left, right) => write!(f, "({} <=> {})", left, right),
        }
    }
}

impl Formula {
    pub fn eval(&self) -> bool {
        match self {
            Formula::False => false,
            Formula::True => true,
            Formula::Not(inner) => !inner.eval(),
            Formula::And(left, right) => left.eval() && right.eval(),
            Formula::Or(left, right) => left.eval() || right.eval(),
            Formula::Xor(left, right) => left.eval() != right.eval(),
            Formula::Implication(left, right) => !left.eval() || right.eval(),
            Formula::Equivalence(left, right) => left.eval() == right.eval(),
        }
    }
}



// /// Parses a string representation of a formula in RPN
// pub fn parse_formula(input: &str) -> Formula {
//     let mut stack: Vec<Formula> = Vec::new();
    
//     for c in input.chars() {
//         match c {
//             '0' => stack.push(Formula::False),
//             '1' => stack.push(Formula::True),
//             '!' => {
//                 let a = stack.pop().expect("Invalid formula: missing operand for negation");
//                 stack.push(Formula::Not(Box::new(a)));
//             }
//             '&' => {
//                 let b = stack.pop().expect("Invalid formula: missing right operand for conjunction");
//                 let a = stack.pop().expect("Invalid formula: missing left operand for conjunction");
//                 stack.push(Formula::And(Box::new(a), Box::new(b)));
//             }
//             '|' => {
//                 let b = stack.pop().expect("Invalid formula: missing right operand for disjunction");
//                 let a = stack.pop().expect("Invalid formula: missing left operand for disjunction");
//                 stack.push(Formula::Or(Box::new(a), Box::new(b)));
//             }
//             '^' => {
//                 let b = stack.pop().expect("Invalid formula: missing right operand for exclusive disjunction");
//                 let a = stack.pop().expect("Invalid formula: missing left operand for exclusive disjunction");
//                 stack.push(Formula::Xor(Box::new(a), Box::new(b)));
//             }
//             '>' => {
//                 let b = stack.pop().expect("Invalid formula: missing right operand for implication");
//                 let a = stack.pop().expect("Invalid formula: missing left operand for implication");
//                 stack.push(Formula::Implication(Box::new(a), Box::new(b)));
//             }
//             '=' => {
//                 let b = stack.pop().expect("Invalid formula: missing right operand for equivalence");
//                 let a = stack.pop().expect("Invalid formula: missing left operand for equivalence");
//                 stack.push(Formula::Equivalence(Box::new(a), Box::new(b)));
//             }
//             c if c.is_whitespace() => continue,
//             _ => panic!("Invalid character in formula: {}", c),
//         }
        
//     }
//     if stack.len() != 1 {
//         panic!("Invalid formula: too many operands left in stack");
//     }
//     stack.pop().expect("Invalid formula: stack is empty")
// }

use std::str::FromStr;
use std::error::Error;

impl FromStr for Formula {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Formula> = Vec::new();

        for c in input.chars() {
            match c {
                '0' => stack.push(Formula::False),
                '1' => stack.push(Formula::True),
                '!' => {
                    let a = stack.pop().ok_or("Missing operand for negation")?;
                    stack.push(Formula::Not(Box::new(a)));
                }
                '&' => {
                    let b = stack.pop().ok_or("Missing right operand for conjunction")?;
                    let a = stack.pop().ok_or("Missing left operand for conjunction")?;
                    stack.push(Formula::And(Box::new(a), Box::new(b)));
                }
                '|' => {
                    let b = stack.pop().ok_or("Missing right operand for disjunction")?;
                    let a = stack.pop().ok_or("Missing left operand for disjunction")?;
                    stack.push(Formula::Or(Box::new(a), Box::new(b)));
                }
                '^' => {
                    let b = stack.pop().ok_or("Missing right operand for exclusive disjunction")?;
                    let a = stack.pop().ok_or("Missing left operand for exclusive disjunction")?;
                    stack.push(Formula::Xor(Box::new(a), Box::new(b)));
                }
                '>' => {
                    let b = stack.pop().ok_or("Missing right operand for implication")?;
                    let a = stack.pop().ok_or("Missing left operand for implication")?;
                    stack.push(Formula::Implication(Box::new(a), Box::new(b)));
                }
                '=' => {
                    let b = stack.pop().ok_or("Missing right operand for equivalence")?;
                    let a = stack.pop().ok_or("Missing left operand for equivalence")?;
                    stack.push(Formula::Equivalence(Box::new(a), Box::new(b)));
                }
                c if c.is_whitespace() => continue,
                _ => return Err(format!("Invalid character in formula: {}", c).into()),
            }
        }

        if stack.len() != 1 {
            return Err("Invalid formula: too many operands left in stack".into());
        }

        Ok(stack.pop().unwrap())
    }
}