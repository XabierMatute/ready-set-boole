/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/13 17:10:33 by xmatute-          #+#    #+#             */
/*   Updated: 2025/06/13 17:28:22 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::eval_formula::eval_formula;

#[test]
fn test_eval_formula_zero() {
    assert_eq!(eval_formula("0"), false);
    assert_eq!(eval_formula("1"), true);
}

#[test]
fn test_eval_formula_subject() {
    assert_eq!(eval_formula("10&"), false);
    assert_eq!(eval_formula("10|"), true);
    assert_eq!(eval_formula("11>"), true);
    assert_eq!(eval_formula("10="), false);
    assert_eq!(eval_formula("1011||="), true);
}

#[test]
fn test_eval_formula_complex() {
    assert_eq!(eval_formula("10&0&1|"), true);
    assert_eq!(eval_formula("10&0&1|!"), false);

}