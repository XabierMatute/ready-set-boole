/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:24:02 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 19:50:04 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ready_set_boole::ex11::inverse_function::reverse_map;
use std::env;

pub fn main() {
    println!("Ready, Set, Inverse!");
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <value>", args[0]);
        eprintln!("Example: {} 0.5", args[0]);
        return;
    }

    let value: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid input. Please provide a valid floating-point number.");
            return;
        }
    };
    
    let (x, y) = reverse_map(value);
    println!("Inverse mapping for value {}: x = {}, y = {}", value, x, y);
}