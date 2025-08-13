/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   formula.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/07 13:11:26 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 20:55:46 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
pub enum Formula {                              // S M Description
    False,                                      // 0 ⊥ false
    True,                                       // 1 ⊤ true
    Variable(char),                             // A...Z A..Z Distinct variables with unknown values
    Set(HashSet<i32>),                              // {1, 2, 3} Set of integers
    Not(Box<Formula>),                          // ! ¬ Negation
    And(Box<Formula>, Box<Formula>),            // & ∧ Conjunction
    Or(Box<Formula>, Box<Formula>),             // | ∨ Disjunction
    Xor(Box<Formula>, Box<Formula>),            // ˆ ⊕ Exclusive disjunction
    Implication(Box<Formula>, Box<Formula>),    // > ⇒ Material condition
    Equivalence(Box<Formula>, Box<Formula>),    // = ⇔ Logical equivalence
}



impl Formula {
    pub fn eval(&self) -> Formula {
        match self {
            Formula::False => Formula::False,
            Formula::True => Formula::True,
            Formula::Variable(c) => Formula::Variable(*c),
            Formula::Set(set) => Formula::Set(set.clone()),
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

    pub fn eval_set(&self,  global_set: &HashSet<i32>) -> Formula {
        match self {
            Formula::False => Formula::False,
            Formula::True => Formula::True,
            Formula::Variable(c) => Formula::Variable(*c),
            Formula::Set(set) => Formula::Set(set.clone()),
            Formula::Not(inner) => match inner.eval_set(global_set) {
                Formula::True => Formula::False,
                Formula::False => Formula::True,
                Formula::Set(set) => {
                    let complement = global_set.difference(&set).cloned().collect();
                    Formula::Set(complement)
                }
                other => Formula::Not(Box::new(other)),
            },
            Formula::And(left, right) => {
                match (left.eval_set(global_set), right.eval_set(global_set)) {
                    (Formula::False, _) | (_, Formula::False) => Formula::False,
                    (Formula::True, r) => r,
                    (l, Formula::True) => l,
                    (l, r) if r == l => l,
                    (Formula::Set(l_set), Formula::Set(r_set)) => {
                        let intersection: HashSet<i32> = l_set.intersection(&r_set).cloned().collect();
                        Formula::Set(intersection)
                    }
                    (l, r) => Formula::And(Box::new(l), Box::new(r)),
                }
            }
            Formula::Or(left, right) => {
                match (left.eval_set(global_set), right.eval_set(global_set)) {
                    (Formula::True, _) | (_, Formula::True) => Formula::True,
                    (Formula::False, r) => r,
                    (l, Formula::False) => l,
                    (l, r) if r == l => l,
                    (Formula::Set(l_set), Formula::Set(r_set)) => {
                        let union: HashSet<i32> = l_set.union(&r_set).cloned().collect();
                        Formula::Set(union)
                    }
                    (l, r) => Formula::Or(Box::new(l), Box::new(r)),
                }
            }
            Formula::Xor(left, right) => {
                match (left.eval_set(global_set), right.eval_set(global_set)) {
                    (Formula::False, r) => r,
                    (l, Formula::False) => l,
                    (Formula::True, r) => Formula::Not(Box::new(r)).eval_set(global_set),
                    (l, Formula::True) => Formula::Not(Box::new(l)).eval_set(global_set),
                    (l, r) if l == r => Formula::False,
                    (Formula::Set(l_set), Formula::Set(r_set)) => {
                        let xor: HashSet<i32> = l_set.symmetric_difference(&r_set).cloned().collect();
                        Formula::Set(xor)
                    }
                    (l, r) => Formula::Xor(Box::new(l), Box::new(r)),
                }
            }
            Formula::Implication(left, right) => {
                match (left.eval_set(global_set), right.eval_set(global_set)) {
                    (Formula::False, _) | (_, Formula::True) => Formula::True,
                    (Formula::True, r) => r,
                    (l, Formula::False) => Formula::Not(Box::new(l)).eval_set(global_set),
                    (Formula::Set(l_set), Formula::Set(r_set)) => {
                        let implication: HashSet<i32> = l_set.difference(&r_set).cloned().collect();
                        Formula::Set(implication)
                    }
                    (l, r) if l == r => Formula::True,
                    (l, r) => Formula::Implication(Box::new(l), Box::new(r)),
                }
            }
            Formula::Equivalence(left, right) => {
                match (left.eval_set(global_set), right.eval_set(global_set)) {
                    (Formula::True, Formula::True) | (Formula::False, Formula::False) => Formula::True,
                    (Formula::True, Formula::False) | (Formula::False, Formula::True) => Formula::False,
                    (l, r) if l == r => Formula::True,
                    (Formula::Set(l_set), Formula::Set(r_set)) => {
                        let equivalence: HashSet<i32> = l_set.intersection(&r_set).cloned().collect();
                        Formula::Set(equivalence)
                    }
                    (l, r) => Formula::Equivalence(Box::new(l), Box::new(r)),
                }
            }
        }
    }

    pub fn substitute(&self, context: &std::collections::HashMap<char, Formula>) -> Formula {
        match self {
            Formula::False => Formula::False,
            Formula::True => Formula::True,
            Formula::Set(set) => Formula::Set(set.clone()),
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

    pub fn substitute_set(&self, context: &std::collections::HashMap<char, Vec<i32>>) -> Formula {
        match self {
            Formula::False => Formula::False,
            Formula::True => Formula::True,
            Formula::Set(set) => Formula::Set(set.clone()),
            Formula::Variable(var) => context.get(var).cloned().map_or_else(
                || Formula::Variable(*var),
                |set| Formula::Set(set.into_iter().collect())
            ),
            Formula::Not(inner) => Formula::Not(Box::new(inner.substitute_set(context))),
            Formula::And(left, right) => Formula::And(
                Box::new(left.substitute_set(context)),
                Box::new(right.substitute_set(context)),
            ),
            Formula::Or(left, right) => Formula::Or(
                Box::new(left.substitute_set(context)),
                Box::new(right.substitute_set(context)),
            ),
            Formula::Xor(left, right) => Formula::Xor(
                Box::new(left.substitute_set(context)),
                Box::new(right.substitute_set(context)),
            ),
            Formula::Implication(left, right) => Formula::Implication(
                Box::new(left.substitute_set(context)),
                Box::new(right.substitute_set(context)),
            ),
            Formula::Equivalence(left, right) => Formula::Equivalence(
                Box::new(left.substitute_set(context)),
                Box::new(right.substitute_set(context)),
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
            Formula::Set(set) => {
                let set_str: String = set.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");
                format!("{{{}}}", set_str)
            }
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
        let mut variables: Vec<char> = self.variables().collect();
        variables.sort();
        let num_vars = variables.len();

        for var in &variables {
            table.push_str(&format!("| {} ", var));
        }
        table.push_str("| = |\n");
        table.push_str(format!("{}|\n", "|---".repeat(num_vars + 1)).as_str());

        for i in 0..(1 << num_vars) {
        // for i in 0..(2u32.pow(num_vars as u32)) {
            let mut current_values = HashMap::new();
            for (j, var) in variables.iter().enumerate() {
                let value = (i >> j) & 1;
                // let value = (i / 2u32.pow(j as u32)) % 2;
                current_values.insert(*var, match value {
                    0 => Formula::False,
                    1 => Formula::True,
                    _ => unreachable!("Unexpected value in truth table"),
                });
                table.push_str(&format!("| {} ", value.to_string()));
            }

            table.push_str(&format!("| {} |\n", match self.substitute(&current_values).eval() {
                Formula::True => '1',
                Formula::False => '0',
                _ => unreachable!("Unexpected result in truth table"),
            }));
        }

        table
    }

    pub fn to_nnf(&self) -> Formula {
        self.eliminate_complex_operators().carry_negation()
    }

    fn eliminate_complex_operators(&self) -> Formula {
        match self {
            Formula::False | Formula::True | Formula::Variable(_) => self.clone(),
            Formula::Set(set) => Formula::Set(set.clone()),
            Formula::Not(inner) => Formula::Not(Box::new(inner.eliminate_complex_operators())),
            Formula::And(left, right) => Formula::And(
                Box::new(left.eliminate_complex_operators()),
                Box::new(right.eliminate_complex_operators()),
            ),
            Formula::Or(left, right) => Formula::Or(
                Box::new(left.eliminate_complex_operators()),
                Box::new(right.eliminate_complex_operators()),
            ),
            Formula::Xor(left, right) => Formula::Or(
                Box::new(Formula::And(
                    Box::new(left.eliminate_complex_operators()),
                    Box::new(Formula::Not(Box::new(right.eliminate_complex_operators()))),
                )),
                Box::new(Formula::And(
                    Box::new(Formula::Not(Box::new(left.eliminate_complex_operators()))),
                    Box::new(right.eliminate_complex_operators()),
                )),
            ),
            Formula::Implication(left, right) => Formula::Or(
                Box::new(Formula::Not(Box::new(left.eliminate_complex_operators()))),
                Box::new(right.eliminate_complex_operators()),
            ),
            Formula::Equivalence(left, right) => Formula::And(
                Box::new(Formula::Or(
                    Box::new(Formula::Not(Box::new(left.eliminate_complex_operators()))),
                    Box::new(right.eliminate_complex_operators()),
                )),
                Box::new(Formula::Or(
                    Box::new(Formula::Not(Box::new(right.eliminate_complex_operators()))),
                    Box::new(left.eliminate_complex_operators()),
                )),
            ),
            // literally unreachable
            // _ => unreachable!("Unexpected formula in complex operator elimination: {}", self),
        }
    }

    fn carry_negation(&self) -> Formula {
        match self {
            Formula::False | Formula::True | Formula::Variable(_) => self.clone(),
            Formula::Not(inner) => match inner.carry_negation() {
                Formula::True => Formula::False,
                Formula::False => Formula::True,
                Formula::Variable(c) => Formula::Not(Box::new(Formula::Variable(c))),
                //Elimination of double negation
                Formula::Not(inner_inner) => *inner_inner,
                //First Law of Morgan
                Formula::And(left, right) => {
                    Formula::Or(Box::new(Formula::Not(Box::new(*left)).carry_negation()),
                                Box::new(Formula::Not(Box::new(*right)).carry_negation()))
                }
                //Second Law of Morgan
                Formula::Or(left, right) => {
                    Formula::And(Box::new(Formula::Not(Box::new(*left)).carry_negation()),
                                 Box::new(Formula::Not(Box::new(*right)).carry_negation()))
                }
                _ => unreachable!("Unexpected formula in NNF conversion: {}", inner),               
            },
            Formula::And(left, right) => Formula::And(
                Box::new(left.carry_negation()),
                Box::new(right.carry_negation()),
            ),
            Formula::Or(left, right) => Formula::Or(
                Box::new(left.carry_negation()),
                Box::new(right.carry_negation()),
            ),
            _ => unreachable!("Unexpected formula in NNF conversion: {}", self),
        }
    }

    pub fn to_cnf(&self) -> Formula {
        // self.to_nnf().carry_conjunction()
        self.to_nnf().carry_disjunction().carry_conjunction()
    }
    
    fn carry_disjunction(&self) -> Formula {
        match self {
            Formula::False | Formula::True | Formula::Variable(_) => self.clone(),
            Formula::Not(inner) => Formula::Not(Box::new(inner.carry_disjunction())),
            Formula::And(left, right) => Formula::And(
                Box::new(left.carry_disjunction()),
                Box::new(right.carry_disjunction()),
            ),
            Formula::Or(left, right) => {
                let left_cnf = left.carry_disjunction();
                let right_cnf = right.carry_disjunction();
                match (left_cnf, right_cnf) {
                    (Formula::And(ll, lr), Formula::And(rl, rr)) => {
                        Formula::And(
                            Box::new(Formula::And(
                                Box::new(Formula::Or(Box::new(*ll.clone()), Box::new(*rl.clone())).carry_disjunction()),
                                Box::new(Formula::Or(Box::new(*lr.clone()), Box::new(*rr.clone())).carry_disjunction()),
                            )),
                            Box::new(Formula::And(
                                Box::new(Formula::Or(Box::new(*ll), Box::new(*rr)).carry_disjunction()),
                                Box::new(Formula::Or(Box::new(*lr), Box::new(*rl)).carry_disjunction()),
                            )),
                        )

                    }
                    (Formula::And(ll, lr), r) => {
                        Formula::And(Box::new(Formula::Or(Box::new(*ll), Box::new(r.clone())).carry_disjunction()),
                                     Box::new(Formula::Or(Box::new(*lr), Box::new(r)).carry_disjunction()))
                    }
                    (l, Formula::And(rl, rr)) => {
                        Formula::And(Box::new(Formula::Or(Box::new(l.clone()), Box::new(*rl)).carry_disjunction()),
                                     Box::new(Formula::Or(Box::new(l), Box::new(*rr)).carry_disjunction()))
                    }
                    (l, r) => Formula::Or(Box::new(l), Box::new(r)),
                }
            }
            _ => unreachable!("Unexpected formula in CNF conversion: {}", self),
        }
    }

    fn carry_conjunction(&self) -> Formula {
        match self {
            Formula::False | Formula::True | Formula::Variable(_) | Formula::Not(_) | Formula::Or(_, _) => self.clone(),
            Formula::And(left, right) => {
                match (left.carry_conjunction(), right.carry_conjunction()) {
                    (Formula::And(ll, lr), r) => {
                        Formula::And(
                            Box::new(*ll),
                            Box::new(Formula::And(
                                Box::new(*lr),
                                Box::new(r),
                            )),
                        )
                    }
                    (l, r) => {
                        Formula::And(Box::new(l), Box::new(r))
                    }
                }
            }
            _ => unreachable!("Unexpected formula in CNF conversion: {}", self),
        }
    }

    pub fn is_satisfiable(&self) -> bool {
        let variables: Vec<char> = self.variables().collect();
        let num_vars = variables.len();

        for i in 0..(1 << num_vars) {
            let mut current_values = HashMap::new();
            for (j, var) in variables.iter().enumerate() {
                let value = (i >> j) & 1;
                // let value = (i / 2u32.pow(j as u32)) % 2;
                current_values.insert(*var, match value {
                    0 => Formula::False,
                    1 => Formula::True,
                    _ => unreachable!("Unexpected value in truth table"),
                });
            }

            if self.substitute(&current_values).eval() == Formula::True {
                return true; // Satisfiable
            }
        }

        false // Unsatisfiable
    }
}



impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::False => write!(f, "False"),
            Formula::True => write!(f, "True"),
            Formula::Variable(var) => write!(f, "{}", var),
            Formula::Set(set) => {
                let set_str: String = set.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "{{{}}}", set_str)
            }
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
                'A'..='Z' => stack.push(Formula::Variable(c)),
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
            return Err(format!("Invalid formula: found {} extra operands:{:?}", stack.len() - 1, stack).into());
        }

        Ok(stack.pop().unwrap())
    }
}
