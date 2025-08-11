/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   nnf.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 14:51:59 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/11 13:24:11 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::extra::formula::Formula;

pub fn negation_normal_form(formula: &str) -> String {
    let parsed_formula = formula.parse::<Formula>()
        .expect("Invalid formula");

    let nnf_formula = parsed_formula.to_nnf();

    nnf_formula.eval().to_rpn()
    // nnf_formula.to_rpn()
}