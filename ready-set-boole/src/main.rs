/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/04 14:18:55 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 18:45:18 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ready_set_boole::ex03::eval_formula::eval_formula;

fn main() {
    println!("Ready, Set, Boole!");
    
    println!("{}", eval_formula("11&")); // Example formula in RPN
    println!("{}", eval_formula("1A&")); // Example formula in RPN
}



// fn main() {
//     println!("Ready, Set, Boole!");
//     // let a = 3;
//     // let b = 5;
//     // let sum = adder(a, b);
//     // println!("The sum of {} and {} is {}", a, b, sum);
//     // let formula = "11&"; // Example formula in RPN
//     // let result = eval_formula(formula);
//     // println!("The result of the formula '{}' is {}", formula, result);
//     // let truth_table_formula = "A B &"; // Example formula for truth table
//     // let f = truth_table_formula.parse::<extra::formula::Formula>()
//     //     .expect("Invalid formula");
//     // println!("Truth table for formula '{}':", truth_table_formula.parse::<extra::formula::Formula>().unwrap());
//     // let s = f.substitute(&HashMap::from([
//     //     ('A', Formula::True),
//     // ]));

//     // truth_table::print_truth_table("A B C & |"); // Example formula for truth table with three variables
//     // println!("{}", f.to_rpn());

//     // println!("{}", negation_normal_form("AB&!")); // Output: A!B!|
//     // println!("{}", negation_normal_form("AB|!")); // Output: A!B!&
//     // println!("{}", negation_normal_form("AB>"));  // Output: A!B|
//     // println!("{}", negation_normal_form("AB="));  // Output: AB&A!B!&|
//     // println!("{}", negation_normal_form("AB|C&!")); // Output: A!B!&C!|
        
//     // println!("{}", conjunctive_normal_form("AB&!")); // Output: A!B!|
//     // println!("{}", conjunctive_normal_form("AB|!")); // Output: A!B!&
//     // println!("{}", conjunctive_normal_form("AB|C&")); // Output: AB|C&
//     // println!("{}", conjunctive_normal_form("AB|C|D|")); // Output: ABCD|||
//     // println!("{}", conjunctive_normal_form("AB&C&D&")); // Output: ABCD&&&
//     // println!("{}", conjunctive_normal_form("AB&!C!|")); // Output: A!B!C!||
//     // println!("{}", conjunctive_normal_form("AB|!C!&")); // Output: A!B!C!&&

//     // println!("{}", sat("AB|")); // Output: true
//     // println!("{}", sat("AB&")); // Output: true
//     // println!("{}", sat("AA!&")); // Output: false
//     // println!("{}", sat("AA^")); // Output: false

//     // let set = vec![1, 2, 3];
//     // let result = powerset(set);

//     // for subset in result {
//     //     println!("{:?}", subset);
//     // }

//     // let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
//     // let result = eval_set("AB&", sets);
//     // println!("{:?}", result); // Output: [0]

//     // let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];
//     // let result = eval_set("AB|", sets);
//     // println!("{:?}", result); // Output: [0, 1, 2, 3, 4, 5]

//     // let sets = vec![vec![0, 1, 2]];
//     // let result = eval_set("A!", sets);
//     // println!("{:?}", result); // Output: []

//     // let x = 42;
//     // let y = 84;

//     // let result = map(x, y);
//     // println!("Mapping ({}, {}) -> {}", x, y, result);

//     // let x = 0;
//     // let y = 0;
//     // println!("Mapping ({}, {}) -> {}", x, y, map(x, y));

//     // let x = u16::MAX;
//     // let y = u16::MAX;
//     // println!("Mapping ({}, {}) -> {}", x, y, map(x, y));

//     // // Example usage
//     // let x = 42;
//     // let y = 84;

//     // // Map (x, y) to a value in [0, 1]
//     // let mapped_value = crate::ex10::curve::map(x, y);
//     // println!("Mapped ({}, {}) -> {}", x, y, mapped_value);

//     // // Reverse the mapping
//     // let (decoded_x, decoded_y) = reverse_map(mapped_value);
//     // println!("Reverse mapped {} -> ({}, {})", mapped_value, decoded_x, decoded_y);

//     // // Test with edge cases
//     // let mapped_value = crate::ex10::curve::map(0, 0);
//     // println!("Reverse mapped {} -> {:?}", mapped_value, reverse_map(mapped_value));

//     // let mapped_value = crate::ex10::curve::map(u16::MAX, u16::MAX);
//     // println!("Reverse mapped {} -> {:?}", mapped_value, reverse_map(mapped_value));
    
// }


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
