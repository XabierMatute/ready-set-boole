/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:23:49 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/13 17:57:33 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![cfg(test)]

mod tests {
    use crate::ex08::powerset::powerset;

    #[test]
    fn test_powerset_empty() {
        assert_eq!(powerset(vec![]), vec![vec![]]);
    }

    #[test]
    fn test_powerset_single_element() {
        assert_eq!(powerset(vec![1]), vec![vec![], vec![1]]);
    }

    #[test]
    fn test_powerset_multiple_elements() {
        let set = vec![1, 2, 3];
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        assert_eq!(powerset(set), expected);
    }

    #[test]
    fn test_powerset_big() {
        let set = vec![1, 2, 3, 4];
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
            vec![4],
            vec![1, 4],
            vec![2, 4],
            vec![1, 2, 4],
            vec![3, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
            vec![1, 2, 3, 4],
        ];
        assert_eq!(powerset(set), expected);
    }
}