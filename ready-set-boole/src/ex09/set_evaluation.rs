/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   set_evaluation.rs                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:27:19 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 18:24:44 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashSet;

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    // Convert input sets to HashSet for efficient operations
    let set_map: Vec<HashSet<i32>> = sets.into_iter().map(|s| s.into_iter().collect()).collect();

    // Compute the universal set (union of all sets)
    let universal_set: HashSet<i32> = set_map.iter().cloned().flatten().collect();

    // Stack for evaluating the formula
    let mut stack: Vec<HashSet<i32>> = Vec::new();

    for c in formula.chars() {
        match c {
            'A'..='Z' => {
                let index = (c as usize) - ('A' as usize);
                if index >= set_map.len() {
                    panic!("Invalid formula: not enough sets provided for variable {}", c);
                }
                stack.push(set_map[index].clone());
            }
            '!' => {
                let a = stack.pop().expect("Invalid formula: missing operand for negation");
                let negation: HashSet<i32> = universal_set.difference(&a).cloned().collect();
                stack.push(negation);
            }
            '&' => {
                let b = stack.pop().expect("Invalid formula: missing right operand for conjunction");
                let a = stack.pop().expect("Invalid formula: missing left operand for conjunction");
                let intersection: HashSet<i32> = a.intersection(&b).cloned().collect();
                stack.push(intersection);
            }
            '|' => {
                let b = stack.pop().expect("Invalid formula: missing right operand for disjunction");
                let a = stack.pop().expect("Invalid formula: missing left operand for disjunction");
                let union: HashSet<i32> = a.union(&b).cloned().collect();
                stack.push(union);
            }
            '^' => {
                let b = stack.pop().expect("Invalid formula: missing right operand for exclusive disjunction");
                let a = stack.pop().expect("Invalid formula: missing left operand for exclusive disjunction");
                let xor: HashSet<i32> = a.symmetric_difference(&b).cloned().collect();
                stack.push(xor);
            }
            '>' => {
                let b = stack.pop().expect("Invalid formula: missing right operand for implication");
                let a = stack.pop().expect("Invalid formula: missing left operand for implication");
                let implication: HashSet<i32> = universal_set
                    .difference(&a)
                    .cloned()
                    .chain(b.clone())
                    .collect();
                stack.push(implication);
            }
            '=' => {
                let b = stack.pop().expect("Invalid formula: missing right operand for equivalence");
                let a = stack.pop().expect("Invalid formula: missing left operand for equivalence");
                let equivalence: HashSet<i32> = a
                    .intersection(&b)
                    .cloned()
                    .chain(
                        universal_set
                            .difference(&a)
                            .cloned()
                            .collect::<HashSet<_>>()
                            .intersection(
                                &universal_set
                                    .difference(&b)
                                    .cloned()
                                    .collect::<HashSet<_>>(),
                            )
                            .cloned(),
                    )
                    .collect();
                stack.push(equivalence);
            }
            _ => panic!("Invalid character in formula: {}", c),
        }
    }

    if stack.len() != 1 {
        panic!("Invalid formula: too many operands left in stack");
    }

    // Convert the resulting set back to a Vec<i32>
    stack.pop().unwrap().into_iter().collect()
}

