/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/05 11:47:51 by xmatute-          #+#    #+#             */
/*   Updated: 2025/06/05 12:20:40 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod adder;

use adder::adder;
use std::env;

fn debug_format(n: u32) -> String {
    format!("{0:#032b} {0}", n)
}

fn main() {
    println!("Ready, Set, add!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <number1> <number2>", args[0]);
        return;
    }

    let a : u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid first number: {}", args[1]);
            return;
        }
    };

    let b : u32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid second number: {}", args[2]);
            return;
        }
    };

    let sum = adder(a, b);
    println!("{}", debug_format(a));
    println!("+");
    println!("{}", debug_format(b));
    println!("=");
    println!("{}", debug_format(sum));
}