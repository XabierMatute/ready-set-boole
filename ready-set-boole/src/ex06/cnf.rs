/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   cnf.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:06:08 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 17:15:51 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::extra::formula::Formula;

fn cnf_str(str: String) -> String {
    let ampersand_count = str.chars().filter(|&c| c == '&').count();
    let mut clauses = str.replace('&', "");
    clauses.push_str(&"&".repeat(ampersand_count));
    clauses
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    //REVISA
    let parsed_formula = formula.parse::<Formula>()
        .expect("Invalid formula");

    let cnf_formula = parsed_formula.to_cnf();

    cnf_str(cnf_formula.to_rpn())
    // cnf_formula.to_rpn()
}