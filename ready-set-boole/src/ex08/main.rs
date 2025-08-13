/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:24:02 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 17:55:25 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ready_set_boole::ex08::powerset::powerset;
use std::env;

pub fn main() {
    println!("Ready, Set, Powerset!");
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} \"<set elements separated by spaces>\"", args[0]);
        eprintln!("Example: {} \"1 2 3\"", args[0]);
        return;
    }

    let set_str = &args[1];
    let set: Vec<i32> = match set_str
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
    {
        Ok(numbers) => numbers,
        Err(e) => {
            eprintln!("Error: Failed to parse input. '{}'. Please provide only integers separated by spaces.", e);
            std::process::exit(1);
        }
    };

    let powerset_result = powerset(set);
    println!("Original set: \"{}\"", set_str);
    println!("Powerset result: {:?}", powerset_result);    
}
