/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/11 07:57:05 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/11 08:38:02 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[cfg(test)]
mod tests {
    use crate::extra::formula::Formula;


    fn test_truth_table(formula_str: &str, expected_table: &str) {
        let formula = formula_str.parse::<Formula>().unwrap();
        println!("Testing formula: {}", formula);
        println!("Expected table:\n{}", expected_table);
        println!("Generated table:\n{}", formula.to_truth_table());
        assert_eq!(formula.to_truth_table(), expected_table);
    }

    #[test]
    fn test_and_operator() {
        let formula = "A B &";
        let expected = "| A | B | = |\n\
                        |---|---|---|\n\
                        | 0 | 0 | 0 |\n\
                        | 1 | 0 | 0 |\n\
                        | 0 | 1 | 0 |\n\
                        | 1 | 1 | 1 |\n";
        test_truth_table(formula, expected);
    }

    #[test]
    fn test_or_operator() {
        let formula = "A B |";
        let expected = "| A | B | = |\n\
                        |---|---|---|\n\
                        | 0 | 0 | 0 |\n\
                        | 1 | 0 | 1 |\n\
                        | 0 | 1 | 1 |\n\
                        | 1 | 1 | 1 |\n";
        test_truth_table(formula, expected);
    }

    #[test]
    fn test_xor_operator() {
        let formula = "A B ^";
        let expected = "| A | B | = |\n\
                        |---|---|---|\n\
                        | 0 | 0 | 0 |\n\
                        | 1 | 0 | 1 |\n\
                        | 0 | 1 | 1 |\n\
                        | 1 | 1 | 0 |\n";
        test_truth_table(formula, expected);
    }

    #[test]
    fn test_not_operator() {
        let formula = "A !";
        let expected = "| A | = |\n\
                        |---|---|\n\
                        | 0 | 1 |\n\
                        | 1 | 0 |\n";
        test_truth_table(formula, expected);
    }

    #[test]
    fn test_subject_formula() {
        let formula = "A B & C |"; // (A & B) | C
        let expected = "| A | B | C | = |\n\
                        |---|---|---|---|\n\
                        | 0 | 0 | 0 | 0 |\n\
                        | 1 | 0 | 0 | 0 |\n\
                        | 0 | 1 | 0 | 0 |\n\
                        | 1 | 1 | 0 | 1 |\n\
                        | 0 | 0 | 1 | 1 |\n\
                        | 1 | 0 | 1 | 1 |\n\
                        | 0 | 1 | 1 | 1 |\n\
                        | 1 | 1 | 1 | 1 |\n";
        test_truth_table(formula, expected);
    }

    #[test]
    fn test_implication_operator() {
        let formula = "A B >";
        let expected = "| A | B | = |\n\
                        |---|---|---|\n\
                        | 0 | 0 | 1 |\n\
                        | 1 | 0 | 0 |\n\
                        | 0 | 1 | 1 |\n\
                        | 1 | 1 | 1 |\n";
        test_truth_table(formula, expected);
    }

    #[test]
    fn test_biconditional_operator() {
        let formula = "A B =";
        let expected = "| A | B | = |\n\
                        |---|---|---|\n\
                        | 0 | 0 | 1 |\n\
                        | 1 | 0 | 0 |\n\
                        | 0 | 1 | 0 |\n\
                        | 1 | 1 | 1 |\n";
        test_truth_table(formula, expected);
    }

    #[test]
    fn test_complex_formula() {
        let formula = "A B & C | D >"; // ((A & B) | C) > D
        let expected = "| A | B | C | D | = |\n\
                        |---|---|---|---|---|\n\
                        | 0 | 0 | 0 | 0 | 1 |\n\
                        | 1 | 0 | 0 | 0 | 1 |\n\
                        | 0 | 1 | 0 | 0 | 1 |\n\
                        | 1 | 1 | 0 | 0 | 0 |\n\
                        | 0 | 0 | 1 | 0 | 0 |\n\
                        | 1 | 0 | 1 | 0 | 0 |\n\
                        | 0 | 1 | 1 | 0 | 0 |\n\
                        | 1 | 1 | 1 | 0 | 0 |\n\
                        | 0 | 0 | 0 | 1 | 1 |\n\
                        | 1 | 0 | 0 | 1 | 1 |\n\
                        | 0 | 1 | 0 | 1 | 1 |\n\
                        | 1 | 1 | 0 | 1 | 1 |\n\
                        | 0 | 0 | 1 | 1 | 1 |\n\
                        | 1 | 0 | 1 | 1 | 1 |\n\
                        | 0 | 1 | 1 | 1 | 1 |\n\
                        | 1 | 1 | 1 | 1 | 1 |\n";
        test_truth_table(formula, expected);
    }
}