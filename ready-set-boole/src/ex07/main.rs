/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:24:02 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 14:18:07 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ready_set_boole::ex07::sat::sat;
use std::env;

pub fn main() {
    println!("Ready, Set, SAT!");
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} \"<formula in RPN>\"", args[0]);
        eprintln!("Example: {} \"AB&!\"", args[0]);
        return;
    }

    let formula_str = &args[1];
    let sat_result = sat(formula_str);

    println!("Original formula: \"{}\" {}", formula_str, formula_str.parse::<ready_set_boole::extra::formula::Formula>().unwrap());
    if sat_result {
        println!("The formula is satisfiable.");
    } else {
        println!("The formula is not satisfiable.");
    }
}