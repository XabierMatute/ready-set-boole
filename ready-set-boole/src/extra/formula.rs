/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   formula.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/07 13:11:26 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/08 18:13:48 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, PartialEq, Clone)]
enum Formula {
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

// fn parse_character(c: char) -> Formula {
//     match c {
//         '0' => Formula::False,
//         '1' => Formula::True,
//         '!' => Formula::Not(Box::new(Formula::False)), // Placeholder, will be replaced in parsing
//         '&' => Formula::And(Box::new(Formula::False), Box::new(Formula::False)), // Placeholder
//         '|' => Formula::Or(Box::new(Formula::False), Box::new(Formula::False)), // Placeholder
//         '^' => Formula::Xor(Box::new(Formula::False), Box::new(Formula::False)), // Placeholder
//         '>' => Formula::Implication(Box::new(Formula::False), Box::new(Formula::False)), // Placeholder
//         '=' => Formula::Equivalence(Box::new(Formula::False), Box::new(Formula::False)), // Placeholder
//         _ => panic!("Invalid character in formula: {}", c),
//     }
// }

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

/// Parses a string representation of a formula in RPN
fn parse_formula(input: &str) -> Formula {
    let mut stack: Vec<Formula> = Vec::new();
    
    for c in input.chars() {
        match c {
            '0' => stack.push(Formula::False),
            '1' => stack.push(Formula::True),
            '!' => {
                let a = stack.pop().expect("Invalid formula: missing operand for negation");
                stack.push(Formula::Not(Box::new(a)));
            }
            '&' => {
                let b = stack.pop().expect("Invalid formula: missing right operand for conjunction");
                let a = stack.pop().expect("Invalid formula: missing left operand for conjunction");
                stack.push(Formula::And(Box::new(a), Box::new(b)));
            }
            '|' => {
                let b = stack.pop().expect("Invalid formula: missing right operand for disjunction");
                let a = stack.pop().expect("Invalid formula: missing left operand for disjunction");
                stack.push(Formula::Or(Box::new(a), Box::new(b)));
            }
            '^' => {
                let b = stack.pop().expect("Invalid formula: missing right operand for exclusive disjunction");
                let a = stack.pop().expect("Invalid formula: missing left operand for exclusive disjunction");
                stack.push(Formula::Xor(Box::new(a), Box::new(b)));
            }
            '>' => {
                let b = stack.pop().expect("Invalid formula: missing right operand for implication");
                let a = stack.pop().expect("Invalid formula: missing left operand for implication");
                stack.push(Formula::Implication(Box::new(a), Box::new(b)));
            }
            '=' => {
                let b = stack.pop().expect("Invalid formula: missing right operand for equivalence");
                let a = stack.pop().expect("Invalid formula: missing left operand for equivalence");
                stack.push(Formula::Equivalence(Box::new(a), Box::new(b)));
            }
            c if c.is_whitespace() => continue,
            _ => panic!("Invalid character in formula: {}", c),
        }
        
    }
    if stack.len() != 1 {
        panic!("Invalid formula: too many operands left in stack");
    }
    stack.pop().expect("Invalid formula: stack is empty")
}

fn main() {
    // Example usage of the Formula enum
    let formula = Formula::And(
        Box::new(Formula::True),
        Box::new(Formula::Not(Box::new(Formula::False))),
    );

    println!("{:#?}", formula);

    let large_formula = Formula::Implication(
        Box::new(Formula::Or(
            Box::new(Formula::And(
                Box::new(Formula::True),
                Box::new(Formula::False),
            )),
            Box::new(Formula::Xor(
                Box::new(Formula::Not(Box::new(Formula::True))),
                Box::new(Formula::False),
            )),
        )),
        Box::new(Formula::Equivalence(
            Box::new(Formula::Not(Box::new(Formula::False))),
            Box::new(Formula::True),
        )),
    );
    println!("{:?}", large_formula);
    let parsed_formula = parse_formula("1 0 1& !  |");
    println!("{:?}", parsed_formula);
    println!("{}", parsed_formula);
    println!("{}", large_formula);
    println!("{:?}", large_formula);
}