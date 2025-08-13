/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   set_evaluation.rs                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:27:19 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 19:31:04 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::extra::formula::Formula;

fn get_global_set(sets: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut global_set = vec![];
    for set in sets {
        for item in set {
            if !global_set.contains(item) {
                global_set.push(*item);
            }
        }
    }
    global_set
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let parsed_formula = formula.parse::<Formula>()
        .expect("Invalid formula");

    let global_set = get_global_set(&sets);

    let context: std::collections::HashMap<char, Formula> = ('A'..='Z')
        .zip(sets.into_iter())
        .map(|(c, s)| (c, Formula::Set(s)))
        .collect();

    let substituted_formula = parsed_formula.substitute(&context);

    let result = substituted_formula.eval_set(&global_set);

    match result {
        Formula::Set(set) => set,
        _ => panic!("Formula did not evaluate to a set: {}", result),
    }
}