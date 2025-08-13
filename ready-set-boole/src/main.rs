/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/04 14:18:55 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 20:36:41 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn debug_format(n: u32) -> String {
    format!("{0:#032b} {0}", n)
}

fn main() {
    println!("Ready, Set, Boole!");
    
    let a = 40;
    let b = 2;
    let sum = ready_set_boole::ex00::adder::adder(a, b);

    println!("The sum of {} and {} is: {}", a, b, sum);

    let c = 10;
    let mult = ready_set_boole::ex01::multiplier::multiplier(sum, c);

    println!("The product of {} and {} is: {}", sum, c, mult);
    
    let gray = ready_set_boole::ex02::gray_code::gray_code(mult);

    println!("The Gray code of {} is: {}", debug_format(mult), debug_format(gray));

    let formula = "0!1&10|^!";
    let result = ready_set_boole::ex03::eval_formula::eval_formula(formula);

    println!("The evaluation of the formula '{}' is: {}", formula, result);

    let formula = "A!B&CD|^!";
    println!("The truth table for the formula '{}' is:", formula);

    ready_set_boole::ex04::truth_table::print_truth_table(formula);

    let nnf = ready_set_boole::ex05::nnf::negation_normal_form(formula);
    println!("The NNF of the formula '{}' is: {}", formula, nnf);
    ready_set_boole::ex04::truth_table::print_truth_table(&nnf);
    
    let cnf = ready_set_boole::ex06::cnf::conjunctive_normal_form(&nnf);
    println!("The CNF of the formula '{}' is: {}", nnf, cnf);
    ready_set_boole::ex04::truth_table::print_truth_table(&cnf);

    if ready_set_boole::ex07::sat::sat(&formula) {
        println!("The formula '{}' is satisfiable.", formula);
    } else {
        println!("The formula '{}' is not satisfiable.", formula);
    }

    let set : Vec<i32> = vec![1, 2, 3];
    let power_set = ready_set_boole::ex08::powerset::powerset(set.clone());
    println!("The power set of {:?} is: {:?}", set, power_set);

    let sets : Vec<Vec<i32>> = vec![
        vec![1, 2],
        vec![2, 3],
        vec![1, 3],
        vec![4, 5],
    ];

    let result_set = ready_set_boole::ex09::set_evaluation::eval_set(formula, sets.clone());
    println!("The evaluation of the set formula 'A&B|C' with sets {:?} is: {:?}", sets, result_set);

    let x: u16 = 5;
    let y: u16 = 10;

    let mapped_value = ready_set_boole::ex10::curve::map(x, y);
    println!("The mapped value for coordinates ({}, {}) is: {}", x, y, mapped_value);
    let coordinates = ready_set_boole::ex11::inverse_function::reverse_map(mapped_value);
    println!("The coordinates for the mapped value {} are: ({}, {})", mapped_value, coordinates.0, coordinates.1);
}
