/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/04 14:18:55 by xmatute-          #+#    #+#             */
/*   Updated: 2025/06/05 08:40:05 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// mod ex00;
// use ex00::adder;
// use adder::adder;

extern crate adder;
fn main() {
    println!("Ready, Set, Boole!");
    let a = 3;
    let b = 5;
    // #[path = "ex00/adder.rs"]
    // mod adder;
    let sum = adder::adder(a, b);
    println!("The sum of {} and {} is {}", a, b, sum);
}
