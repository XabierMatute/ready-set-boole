/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/13 17:10:33 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/08 18:52:31 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::eval_formula::eval_formula;

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
    assert_eq!(eval_formula("10&1|"), true);
    assert_eq!(eval_formula("10&0&1|"), true);
    assert_eq!(eval_formula("10&0&1|!"), false);
    assert_eq!(eval_formula("10&1|!"), false);
    assert_eq!(eval_formula("10&1|!0&1|"), true);
    assert_eq!(eval_formula("10&1|!0&1|!"), false);
    assert_eq!(eval_formula("10&1|!0&1|!0&1|"), true);
    assert_eq!(eval_formula("10&1|!0&1|!0&1|!"), false);
}

#[test]
fn test_eval_formula_not() {
    assert_eq!(eval_formula("0!"), true);
    assert_eq!(eval_formula("1!"), false);
    assert_eq!(eval_formula("0!!"), false);
    assert_eq!(eval_formula("1!!"), true);
}

#[test]
fn test_eval_formula_and() {
    assert_eq!(eval_formula("11&"), true);
    assert_eq!(eval_formula("10&"), false);
    assert_eq!(eval_formula("01&"), false);
    assert_eq!(eval_formula("00&"), false);
}

#[test]
fn test_eval_formula_or() {
    assert_eq!(eval_formula("11|"), true);
    assert_eq!(eval_formula("10|"), true);
    assert_eq!(eval_formula("01|"), true);
    assert_eq!(eval_formula("00|"), false);
}

#[test]
fn test_eval_formula_xor() {
    assert_eq!(eval_formula("11^"), false);
    assert_eq!(eval_formula("10^"), true);
    assert_eq!(eval_formula("01^"), true);
    assert_eq!(eval_formula("00^"), false);
}

#[test]
fn test_eval_formula_implication() {
    assert_eq!(eval_formula("11>"), true);
    assert_eq!(eval_formula("10>"), false);
    assert_eq!(eval_formula("01>"), true);
    assert_eq!(eval_formula("00>"), true);
}

#[test]
fn test_eval_formula_equivalence() {
    assert_eq!(eval_formula("11="), true);
    assert_eq!(eval_formula("10="), false);
    assert_eq!(eval_formula("01="), false);
    assert_eq!(eval_formula("00="), true);
}

#[test]
fn test_eval_formula_mixed() {
    assert_eq!(eval_formula("10&1|"), true); // (1 AND 0) OR 1
    assert_eq!(eval_formula("10|1&"), true); // 1 OR (0 AND 1)
    assert_eq!(eval_formula("10^1&"), true); // 1 XOR (0 AND 1)
    assert_eq!(eval_formula("10>1|"), true); // (1 => 0) OR 1
    assert_eq!(eval_formula("10=1&"), false); // (1 == 0) AND 1
}

#[test]
fn test_eval_formula_complex_nested() {
    assert_eq!(eval_formula("10&1|0!&"), true); // ((1 AND 0) OR 1) AND NOT 0
    assert_eq!(eval_formula("10|1&0!|"), true); // (1 OR (0 AND 1)) OR NOT 0
    assert_eq!(eval_formula("10^1&0!>"), true); // (1 XOR (0 AND 1)) => NOT 0
    assert_eq!(eval_formula("10>1|0!="), true); // ((1 => 0) OR 1) == NOT 0
}