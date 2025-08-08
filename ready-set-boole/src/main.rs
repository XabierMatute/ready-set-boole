/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/04 14:18:55 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/08 18:36:05 by xmatute-         ###   ########.fr       */
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

mod ex02 {
    pub mod gray_code;
    #[cfg(test)]
    mod test;
}

mod ex03 {
    pub mod eval_formula;
    #[cfg(test)]
    mod test;
}

mod extra {
    pub mod formula;
    // pub mod parse_formula;
}

use ex00::adder::adder;
use ex03::eval_formula::eval_formula;

fn main() {
    println!("Ready, Set, Boole!");
    let a = 3;
    let b = 5;
    let sum = adder(a, b);
    println!("The sum of {} and {} is {}", a, b, sum);
    let formula = "11&"; // Example formula in RPN
    let result = eval_formula(formula);
    println!("The result of the formula '{}' is {}", formula, result);
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

// Symbol Mathematical
// equivalent
// Description
// 0 ⊥ false
// 1 ⊤ true
// ! ¬ Negation
// & ∧ Conjunction
// | ∨ Disjunction
// ˆ ⊕ Exclusive disjunction
// > ⇒ Material condition
// = ⇔ Logical equivalence
