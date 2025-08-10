/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:24:02 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 20:27:30 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ready_set_boole::ex04::truth_table::print_truth_table;
use std::env;

fn main() {
    println!("Ready, Set, Truth!");
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} \"<formula>\"", args[0]);
        return;
    }

    let formula_str = &args[1];

    print_truth_table(formula_str);
}