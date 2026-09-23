/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:23:49 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 19:52:19 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![cfg(test)]

mod tests {
    use crate::ex09::set_evaluation::eval_set;

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort();
        v
    }

    #[test]
    fn test_eval_set_basic() {
        let sets = vec![vec![1, 2], vec![2, 3]];
        let formula = "A B &";
        let result = eval_set(formula, sets);
        assert_eq!(sorted(result), vec![2]);
    }

    #[test]
    fn test_eval_set_or() {
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
        let formula = "A B |";
        let result = eval_set(formula, sets);
        assert_eq!(sorted(result), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_eval_set_xor() {
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
        let formula = "A B ^";
        let result = eval_set(formula, sets);
        assert_eq!(sorted(result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_eval_set_not() {
        let sets = vec![vec![0, 1, 2]];
        let formula = "A !";
        let result = eval_set(formula, sets);
        assert_eq!(sorted(result), vec![]);
    }

    #[test]
    fn test_eval_set_not_with_second_set() {
        let sets = vec![vec![0, 2], vec![0, 1, 2, 3]];
        let formula = "A !";
        let result = eval_set(formula, sets);
        assert_eq!(sorted(result), vec![1, 3]);
    }

    #[test]
    fn test_eval_set_implication() {
        let sets = vec![vec![0, 1], vec![1, 2]];
        let formula = "A B >";
        let result = eval_set(formula, sets);
        assert_eq!(sorted(result), vec![1, 2]);
    }

    #[test]
    fn test_eval_set_equivalence() {
        let sets = vec![vec![0, 1, 2], vec![1, 2, 3]];
        let formula = "A B =";
        let result = eval_set(formula, sets);
        assert_eq!(sorted(result), vec![1, 2]);
    }

    #[test]
    fn test_eval_set_equivalence_both_false() {
        let sets = vec![vec![0, 5], vec![1, 5]];
        let formula = "A B =";
        let result = eval_set(formula, sets);
        // global = {0,1,5}; both_true = {5}; A_false={1}, B_false={0} -> both_false = {}
        assert_eq!(sorted(result), vec![5]);
    }

    #[test]
    fn test_eval_set_three_variables() {
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4], vec![0, 1, 4]];
        let formula = "A B & C |";
        let result = eval_set(formula, sets);
        assert_eq!(sorted(result), vec![0, 1, 4]);
    }

    #[test]
    fn test_eval_set_empty_sets() {
        let sets = vec![vec![], vec![]];
        let formula = "A B |";
        let result = eval_set(formula, sets);
        assert_eq!(sorted(result), vec![]);
    }

    #[test]
    fn test_eval_set_identical_sets() {
        let sets = vec![vec![1, 2, 3], vec![1, 2, 3]];
        let formula = "A B ^";
        let result = eval_set(formula, sets);
        assert_eq!(sorted(result), vec![]);
    }

    #[test]
    fn test_eval_set_single_variable() {
        let sets = vec![vec![5, 6, 7]];
        let formula = "A";
        let result = eval_set(formula, sets);
        assert_eq!(sorted(result), vec![5, 6, 7]);
    }
}