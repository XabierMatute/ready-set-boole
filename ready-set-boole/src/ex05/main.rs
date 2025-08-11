/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:24:02 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/11 09:52:10 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ready_set_boole::ex05::nnf::negation_normal_form;
use std::env;

pub fn main() {
    println!("Ready, Set, NNF!");
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} \"<formula in RPN>\"", args[0]);
        eprintln!("Example: {} \"AB&!\"", args[0]);
        return;
    }

    let formula_str = &args[1];
    let nnf_result = negation_normal_form(formula_str);

    println!("Original formula: \"{}\" {}", formula_str, formula_str.parse::<ready_set_boole::extra::formula::Formula>().unwrap());
    println!("NNF result:       \"{}\" {}", nnf_result, nnf_result.parse::<ready_set_boole::extra::formula::Formula>().unwrap());
}