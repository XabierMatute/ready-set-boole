/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   sat.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:19:26 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 19:42:45 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// use crate::extra::formula::Formula;
// use std::collections::HashMap;

pub fn sat(formula: &str) -> bool {
    println!("Checking satisfiability for: {}", formula);
    // // Parse the formula
    // let parsed_formula = formula.parse::<Formula>()
    //     .expect("Invalid formula");

    // // Get the variables used in the formula
    // let variables: Vec<char> = parsed_formula.variables().collect();
    // let num_vars = variables.len();

    // // Generate all combinations of truth values
    // for i in 0..(1 << num_vars) {
    //     let mut context = HashMap::new();

    //     // Assign truth values to variables
    //     for (j, var) in variables.iter().enumerate() {
    //         let value = (i & (1 << j)) != 0;
    //         if value {
    //             context.insert(*var, Formula::True);
    //         } else {
    //             context.insert(*var, Formula::False);
    //         }
    //     }

    //     // Evaluate the formula with the current context
    //     let substituted_formula = parsed_formula.substitute(&context);
    //     if substituted_formula.eval() {
    //         return true; // Satisfiable
    //     }
    // }

    false // Unsatisfiable
}