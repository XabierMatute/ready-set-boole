/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   nnf.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 14:51:59 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 18:22:11 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::extra::formula::Formula;

pub fn negation_normal_form(formula: &str) -> String {
    // Parse the formula into the Formula enum
    let parsed_formula = formula.parse::<Formula>()
        .expect("Invalid formula");

    // Convert the formula to NNF
    let nnf_formula = to_nnf(&parsed_formula);

    // Convert the NNF formula back to RPN
    nnf_formula.to_rpn()
}

fn to_nnf(formula: &Formula) -> Formula {
    match formula {
        Formula::Not(inner) => match **inner {
            // Apply De Morgan's laws
            Formula::And(ref left, ref right) => Formula::Or(
                Box::new(to_nnf(&Formula::Not(left.clone()))),
                Box::new(to_nnf(&Formula::Not(right.clone()))),
            ),
            Formula::Or(ref left, ref right) => Formula::And(
                Box::new(to_nnf(&Formula::Not(left.clone()))),
                Box::new(to_nnf(&Formula::Not(right.clone()))),
            ),
            // Double negation elimination
            Formula::Not(ref inner_inner) => to_nnf(inner_inner),
            // Negation of a variable remains as is
            Formula::Variable(_) | Formula::False | Formula::True => Formula::Not(Box::new(to_nnf(inner))),
            _ => panic!("Unsupported negation structure"),
        },
        // Replace implications and equivalences with equivalent expressions
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
        // Recursively process binary operators
        Formula::And(ref left, ref right) => Formula::And(
            Box::new(to_nnf(left)),
            Box::new(to_nnf(right)),
        ),
        Formula::Or(ref left, ref right) => Formula::Or(
            Box::new(to_nnf(left)),
            Box::new(to_nnf(right)),
        ),
        // Base cases: variables, true, and false
        Formula::Variable(_) | Formula::False | Formula::True => formula.clone(),
    }
}