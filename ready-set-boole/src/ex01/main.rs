/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:24:02 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 20:13:16 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ready_set_boole::ex01::multiplier::multiplier;
use std::env;

fn debug_format(n: u32) -> String {
    format!("{0:#032b} {0}", n)
}

fn main() {
    println!("Ready, Set, multiply!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <number1> <number2>", args[0]);
        return;
    }

    let a: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid first number: {}", args[1]);
            return;
        }
    };

    let b: u32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid second number: {}", args[2]);
            return;
        }
    };

    let product = multiplier(a, b);
    println!("{}", debug_format(a));
    println!("*");
    println!("{}", debug_format(b));
    println!("=");
    println!("{}", debug_format(product));
}