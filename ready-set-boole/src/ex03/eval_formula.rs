/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   eval_formula.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/04 14:04:10 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/10 20:11:44 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::extra::formula::Formula;

// pub fn eval_formula(formula: &str) -> bool {
//     formula.parse::<Formula>()
//         .expect("Parse error (Invalid formula)")
//         .eval()
// }

pub fn eval_formula(formula: &str) -> bool {
    let r = formula.parse::<Formula>()
        .expect("Parse error (Invalid formula)")
        .eval();
    match r {
        Formula::True => true,
        Formula::False => false,
        _ => panic!("{}", format!("Formula did not evaluate to a boolean: {}", r)),
    }
}