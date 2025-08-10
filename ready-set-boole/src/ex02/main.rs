/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:24:02 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 20:13:08 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ready_set_boole::ex02::gray_code::gray_code;
use std::env;

fn debug_format(n: u32) -> String {
    format!("{0:#034b} ({0})", n)
}

fn main() {
    println!("Ready, Set, Gray!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        return;
    }

    let n: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number: {}", args[1]);
            return;
        }
    };

    let gray = gray_code(n);

    println!("Gray code for {}", n);
    println!("Binary: {}", debug_format(n));
    println!("Gray:   {}", debug_format(gray));
}