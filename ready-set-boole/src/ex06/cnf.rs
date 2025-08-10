/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   cnf.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:06:08 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 18:23:28 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::extra::formula::Formula;

pub fn conjunctive_normal_form(formula: &str) -> String {
    // Step 1: Convert the formula to Negation Normal Form (NNF)
    let nnf_formula = formula.parse::<Formula>()
        .expect("Invalid formula");
    let nnf_formula = to_nnf(&nnf_formula);

    // Step 2: Distribute OR over AND to achieve CNF
    let cnf_formula = to_cnf(&nnf_formula);

    // Step 3: Convert the CNF formula back to RPN
    cnf_formula.to_rpn()
}

fn to_cnf(formula: &Formula) -> Formula {
    match formula {
        // Base cases: variables, true, false, or negations
        Formula::False | Formula::True | Formula::Variable(_) | Formula::Not(_) => formula.clone(),

        // Distribute OR over AND: (A | (B & C)) -> ((A | B) & (A | C))
        Formula::Or(left, right) => {
            let left_cnf = to_cnf(left);
            let right_cnf = to_cnf(right);

            match (left_cnf, right_cnf) {
                (Formula::And(l1, l2), r) => Formula::And(
                    Box::new(to_cnf(&Formula::Or(l1, Box::new(r.clone())))),
                    Box::new(to_cnf(&Formula::Or(l2, Box::new(r)))),
                ),
                (l, Formula::And(r1, r2)) => Formula::And(
                    Box::new(to_cnf(&Formula::Or(Box::new(l.clone()), r1))),
                    Box::new(to_cnf(&Formula::Or(Box::new(l), r2))),
                ),
                (l, r) => Formula::Or(Box::new(l), Box::new(r)),
            }
        }

        // Recursively process AND
        Formula::And(left, right) => Formula::And(
            Box::new(to_cnf(left)),
            Box::new(to_cnf(right)),
        ),

        _ => panic!("Unsupported formula structure for CNF conversion"),
    }
}

fn to_nnf(formula: &Formula) -> Formula {
    match formula {
        Formula::Not(inner) => match **inner {
            Formula::And(ref left, ref right) => Formula::Or(
                Box::new(to_nnf(&Formula::Not(left.clone()))),
                Box::new(to_nnf(&Formula::Not(right.clone()))),
            ),
            Formula::Or(ref left, ref right) => Formula::And(
                Box::new(to_nnf(&Formula::Not(left.clone()))),
                Box::new(to_nnf(&Formula::Not(right.clone()))),
            ),
            Formula::Not(ref inner_inner) => to_nnf(inner_inner),
            Formula::Variable(_) | Formula::False | Formula::True => Formula::Not(Box::new(to_nnf(inner))),
            _ => panic!("Unsupported negation structure"),
        },
        Formula::Implication(ref left, ref right) => to_nnf(&Formula::Or(
            Box::new(Formula::Not(left.clone())),
            Box::new(*right.clone()),
        )),
        Formula::Equivalence(ref left, ref right) => to_nnf(&Formula::And(
            Box::new(Formula::Or(
                Box::new(Formula::Not(left.clone())),
                Box::new(*right.clone()),
            )),
            Box::new(Formula::Or(
                Box::new(Formula::Not(right.clone())),
                Box::new(*left.clone()),
            )),
        )),
        Formula::Xor(ref left, ref right) => to_nnf(&Formula::Or(
            Box::new(Formula::And(
                Box::new(*left.clone()),
                Box::new(Formula::Not(right.clone())),
            )),
            Box::new(Formula::And(
                Box::new(Formula::Not(left.clone())),
                Box::new(*right.clone()),
            )),
        )),
        Formula::And(ref left, ref right) => Formula::And(
            Box::new(to_nnf(left)),
            Box::new(to_nnf(right)),
        ),
        Formula::Or(ref left, ref right) => Formula::Or(
            Box::new(to_nnf(left)),
            Box::new(to_nnf(right)),
        ),
        Formula::Variable(_) | Formula::False | Formula::True => formula.clone(),
    }
}