/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   sat.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:19:26 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 14:13:49 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::extra::formula::Formula;

pub fn sat(formula: &str) -> bool {
    let parsed_formula = formula.parse::<Formula>()
        .expect("Invalid formula");

    parsed_formula.is_satisfiable()
}
