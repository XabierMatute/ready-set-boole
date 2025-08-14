/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:06:16 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/14 16:29:52 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[cfg(test)]
mod tests {
    use crate::extra::formula::Formula;
    use crate::ex06::cnf::conjunctive_normal_form;

    use regex::Regex;

    fn is_in_cnf(formula: &str) -> bool {
        let invalid_negation = Regex::new(r"[^A-Z]!|^!").unwrap();
        if invalid_negation.is_match(formula) {
            println!("Validation failed: Found a '!' not immediately after an uppercase letter.");
            return false;
        }

        let misplaced_conjunction = Regex::new(r"&[^&]").unwrap();
        if misplaced_conjunction.is_match(formula) {
            println!("Validation failed: Found a '&' that is not at the end of a clause.");
            return false;
        }

        let invalid_chars = Regex::new(r"[^A-Z!|& ]").unwrap();
        if invalid_chars.is_match(formula) {
            println!("Validation failed: Found invalid characters in the formula.");
            return false;
        }

        true
    }

    fn test_cnf(input: &str) {
        println!("Testing CNF for input: {}", input);
        let result = conjunctive_normal_form(&input);
        println!("Result: {}", result);
        assert!(is_in_cnf(&result), "The result is not in CNF: {}", result);
        let input_tt = input.parse::<Formula>()
                            .expect("Parse error (Invalid formula)")
                            .to_truth_table();
        println!("Input truth table:\n{}", input_tt);
        let result_tt = result.parse::<Formula>()
                            .expect("Parse error (Invalid formula)")
                            .to_truth_table();
        println!("Result truth table:\n{}", result_tt);
        assert_eq!(input_tt, result_tt, "Truth tables do not match for input:\n{} and result:\n{}", input_tt, result_tt);
    }

    #[test]
    fn test_cnf_basic() {
        test_cnf("A");
        test_cnf("A B &");
        test_cnf("A B |");
        test_cnf("A B ^");
        test_cnf("A B >");
        test_cnf("A B =");
    }

    #[test]
    fn test_cnf_negation() {
        test_cnf("A !");
        test_cnf("A B & !");
        test_cnf("A B | !");
        test_cnf("A B ^ !");
        test_cnf("A B > !");
        test_cnf("A B = !");
    }

    #[test]
    fn test_cnf_subject() {
        test_cnf("AB&!");
        test_cnf("AB|!");
        test_cnf("AB|C&!");
        test_cnf("AB|C|D|");
        test_cnf("AB&C&D&");
        test_cnf("AB&!C!|");
        test_cnf("AB|!C!&");
    }

    #[test]
    fn test_cnf_extra() {
        test_cnf("AB&C|D&!");
        test_cnf("AB&C|D|E&!");
        test_cnf("AB&C|D|E|F&!");
        test_cnf("AB&C|D|E|F|G&!");
        test_cnf("AB&C&D&E&F&G&");
        test_cnf("A B C D & & & !");
        test_cnf("A B C D | | | !");
        test_cnf("A B C D ^ ^ ^ !");
        test_cnf("A B C D > > > !");
        test_cnf("A B C D = = = !");
        test_cnf("A B C D & | ! >");
        test_cnf("A!B&CD|^!J=A>G&");

        
    }
}