/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:24:02 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 19:45:06 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ready_set_boole::ex10::curve::map;
use std::env;

pub fn main() {
    println!("Ready, Set, Curve!");
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <x> <y>", args[0]);
        eprintln!("Example: {} 12345 67890", args[0]);
        return;
    }

    let x: u16 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid input for x. Please provide a valid unsigned integer.");
            return;
        }
    };

    let y: u16 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid input for y. Please provide a valid unsigned integer.");
            return;
        }
    };

    let result = map(x, y);

    println!("Mapped value for ({}, {}) is: {}", x, y, result);
}