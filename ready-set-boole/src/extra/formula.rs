/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   formula.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/07 13:11:26 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 20:35:04 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, PartialEq, Clone)]
pub enum Formula {                              // S M Description
    False,                                      // 0 ⊥ false
    True,                                       // 1 ⊤ true
    Variable(char),                             // A...Z A..Z Distinct variables with unknown values
    Not(Box<Formula>),                          // ! ¬ Negation
    And(Box<Formula>, Box<Formula>),            // & ∧ Conjunction
    Or(Box<Formula>, Box<Formula>),             // | ∨ Disjunction
    Xor(Box<Formula>, Box<Formula>),            // ˆ ⊕ Exclusive disjunction
    Implication(Box<Formula>, Box<Formula>),    // > ⇒ Material condition
    Equivalence(Box<Formula>, Box<Formula>),    // = ⇔ Logical equivalence
}

use std::fmt;
use std::collections::HashMap;

impl Formula {
    pub fn eval(&self) -> Formula {
        match self {
            Formula::False => Formula::False,
            Formula::True => Formula::True,
            Formula::Variable(c) => Formula::Variable(*c), 
            Formula::Not(inner) => match inner.eval() {
                Formula::True => Formula::False,
                Formula::False => Formula::True,
                other => Formula::Not(Box::new(other)),
            },
            Formula::And(left, right) => match (left.eval(), right.eval()) {
                (Formula::False, _) | (_, Formula::False) => Formula::False,
                (Formula::True, r) => r,
                (l, Formula::True) => l,
                (l, r) if r == l => l,
                (l, r) => Formula::And(Box::new(l), Box::new(r)),
            },
            Formula::Or(left, right) => match (left.eval(), right.eval()) {
                (Formula::True, _) | (_, Formula::True) => Formula::True,
                (Formula::False, r) => r,
                (l, Formula::False) => l,
                (l, r) if r == l => l,
                (l, r) => Formula::Or(Box::new(l), Box::new(r)),
            },
            Formula::Xor(left, right) => match (left.eval(), right.eval()) {
                (Formula::False, r) => r,
                (l, Formula::False) => l,
                (Formula::True, r) => Formula::Not(Box::new(r)).eval(),
                (l, Formula::True) => Formula::Not(Box::new(l)).eval(),
                (l, r) if l == r => Formula::False,                
                (l, r) => Formula::Xor(Box::new(l), Box::new(r)),
            },
            Formula::Implication(left, right) => match (left.eval(), right.eval()) {
                (Formula::False, _) | (_, Formula::True) => Formula::True,
                (Formula::True, r) => r,
                (l, Formula::False) => Formula::Not(Box::new(l)).eval(),
                (l, r) if l == r => Formula::True,
                (l, r) => Formula::Implication(Box::new(l), Box::new(r)),
            },
            Formula::Equivalence(left, right) => match (left.eval(), right.eval()) {
                (Formula::True, Formula::True) | (Formula::False, Formula::False) => Formula::True,
                (Formula::True, Formula::False) | (Formula::False, Formula::True) => Formula::False,
                (l, r) if l == r => Formula::True,
                (l, r) => Formula::Equivalence(Box::new(l), Box::new(r)),
            },
        }
    }

    pub fn substitute(&self, context: &std::collections::HashMap<char, Formula>) -> Formula {
        match self {
            Formula::False => Formula::False,
            Formula::True => Formula::True,
            Formula::Variable(var) => context.get(var).cloned().unwrap_or_else(|| Formula::Variable(*var)),
            Formula::Not(inner) => Formula::Not(Box::new(inner.substitute(context))),
            Formula::And(left, right) => Formula::And(
                Box::new(left.substitute(context)),
                Box::new(right.substitute(context)),
            ),
            Formula::Or(left, right) => Formula::Or(
                Box::new(left.substitute(context)),
                Box::new(right.substitute(context)),
            ),
            Formula::Xor(left, right) => Formula::Xor(
                Box::new(left.substitute(context)),
                Box::new(right.substitute(context)),
            ),
            Formula::Implication(left, right) => Formula::Implication(
                Box::new(left.substitute(context)),
                Box::new(right.substitute(context)),
            ),
            Formula::Equivalence(left, right) => Formula::Equivalence(
                Box::new(left.substitute(context)),
                Box::new(right.substitute(context)),
            ),
        }
    }

    pub fn variables(&self) -> impl Iterator<Item = char> {
        let mut vars = HashMap::new();
        self.collect_variables(&mut vars);
        vars.into_keys()
    }

    fn collect_variables(&self, vars: &mut HashMap<char, bool>) {
        match self {
            Formula::Variable(var) => {
                vars.insert(*var, true);
            }
            Formula::Not(inner) => inner.collect_variables(vars),
            Formula::And(left, right) | Formula::Or(left, right)
            | Formula::Xor(left, right) | Formula::Implication(left, right)
            | Formula::Equivalence(left, right) => {
                left.collect_variables(vars);
                right.collect_variables(vars);
            }
            _ => {}
        }
    }

    pub fn to_rpn(&self) -> String {
        match self {
            Formula::False => "0".to_string(),
            Formula::True => "1".to_string(),
            Formula::Variable(var) => var.to_string(),
            Formula::Not(inner) => format!("{}!", inner.to_rpn()),
            Formula::And(left, right) => format!("{}{}&", left.to_rpn(), right.to_rpn()),
            Formula::Or(left, right) => format!("{}{}|", left.to_rpn(), right.to_rpn()),
            Formula::Xor(left, right) => format!("{}{}^", left.to_rpn(), right.to_rpn()),
            Formula::Implication(left, right) => format!("{}{}>", left.to_rpn(), right.to_rpn()),
            Formula::Equivalence(left, right) => format!("{}{}=", left.to_rpn(), right.to_rpn()),
        }
    }

    pub fn to_truth_table(&self) -> String {
        let mut table = String::new();
        let variables: Vec<char> = self.variables().collect();
        let num_vars = variables.len();

        for var in &variables {
            table.push_str(&format!("| {} ", var));
        }
        table.push_str("| = |\n");
        table.push_str(format!("{}|\n", "|---".repeat(num_vars + 1)).as_str());

        for i in 0..(1 << num_vars) {
            let mut context = HashMap::new();
            for (j, var) in variables.iter().enumerate() {
                let value = (i & (1 << j)) != 0;
                context.insert(*var, if value { Formula::True } else { Formula::False });
                table.push_str(&format!("| {} ", if value { '1' } else { '0' }));
            }

            let substituted_formula = self.substitute(&context);
            let result = substituted_formula.eval();

            table.push_str(&format!("| {} |\n", if result == Formula::True { '1' } else { '0' }));
        }

        table
    }
}



impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::False => write!(f, "False"),
            Formula::True => write!(f, "True"),
            Formula::Variable(var) => write!(f, "{}", var),
            Formula::Not(inner) => write!(f, "not {}", inner),
            Formula::And(left, right) => write!(f, "({} and {})", left, right),
            Formula::Or(left, right) => write!(f, "({} or {})", left, right),
            Formula::Xor(left, right) => write!(f, "({} xor {})", left, right),
            Formula::Implication(left, right) => write!(f, "({} => {})", left, right),
            Formula::Equivalence(left, right) => write!(f, "({} <=> {})", left, right),
        }
    }
}

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
                'A'..='Z' | 'a'..='z' => stack.push(Formula::Variable(c)),
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
            return Err(format!("Invalid formula: found {} extra operands", stack.len() - 1).into());
        }

        Ok(stack.pop().unwrap())
    }
}
