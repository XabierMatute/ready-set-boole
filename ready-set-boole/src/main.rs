/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/04 14:18:55 by xmatute-          #+#    #+#             */
/*   Updated: 2025/06/05 12:24:34 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod ex00 {
    pub mod adder;
    #[cfg(test)]
    mod test;
}

mod ex01 {
    pub mod multiplier;
    #[cfg(test)]
    mod test;
}

use ex00::adder::adder;

fn main() {
    println!("Ready, Set, Boole!");
    let a = 3;
    let b = 5;
    let sum = adder(a, b);
    println!("The sum of {} and {} is {}", a, b, sum);
}

// #[test]
// fn test_adder() {
//     assert_eq!(adder(0, 0), 0);
//     assert_eq!(adder(1, 1), 2);
//     assert_eq!(adder(2, 3), 5);
//     assert_eq!(adder(10, 20), 30);
//     assert_eq!(adder(100, 200), 300);
//     assert_eq!(adder(123456789, 987654321), 1111111110);
// }