/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   truth_table.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/08 19:25:44 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/11 07:48:33 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::extra::formula::Formula;

pub fn print_truth_table(formula: &str) {
    let parsed_formula = formula.parse::<Formula>()
        .expect("Invalid formula");

    println!("{}", parsed_formula.to_truth_table());
}