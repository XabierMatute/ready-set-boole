/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   truth_table.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/08 19:25:44 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 20:32:02 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::extra::formula::Formula;
// use std::collections::HashMap;

// pub fn print_truth_table(formula: &str) {
//     let parsed_formula = formula.parse::<Formula>()
//         .expect("Invalid formula");

//     let variables: Vec<char> = parsed_formula.variables().collect();
//     let num_vars = variables.len();

//     // Print header
//     for var in &variables {
//         print!("| {} ", var);
//     }
//     println!("| = |");
//     println!("{}", "|---".repeat(num_vars + 1) + "|");

//     for i in 0..(1 << num_vars) {
//         let mut context = HashMap::new();

//         // Assign truth values to variables
//         for (j, var) in variables.iter().enumerate() {
//             let value = (i & (1 << j)) != 0;
//             context.insert(*var, if value { Formula::True } else { Formula::False });
//             print!("| {} ", if value { '1' } else { '0' });
//         }

//         // Evaluate the formula with the current context
//         let substituted_formula = parsed_formula.substitute(&context);
//         let result = substituted_formula.eval();

//         // Print the result
//         println!("| {} |", if result == Formula::True { '1' } else { '0' });
//     }
// }

pub fn print_truth_table(formula: &str) {
    let parsed_formula = formula.parse::<Formula>()
        .expect("Invalid formula");

    println!("{}", parsed_formula.to_truth_table());
}