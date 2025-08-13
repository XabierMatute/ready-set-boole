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
    
    #[test]
    fn test_eval_set_basic() {
        let sets = vec![vec![1, 2], vec![2, 3]];
        let formula = "A B &";
        let result = eval_set(formula, sets);
        assert_eq!(result, vec![2]);
    }

    // TODO: Add more tests for set evaluation
}