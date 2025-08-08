/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   eval_formula.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/04 14:04:10 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/08 19:08:31 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::extra::formula::Formula;
// use crate::extra::formula::parse_formula;



pub fn eval_formula(formula: &str) -> bool {
    formula.parse::<Formula>()
        .expect("Invalid formula")
        .eval()
}
