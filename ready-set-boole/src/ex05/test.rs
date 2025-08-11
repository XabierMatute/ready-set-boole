/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/08/09 15:06:05 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/11 13:23:19 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[cfg(test)]
mod tests {
    use crate::extra::formula::Formula;
    use crate::ex05::nnf::negation_normal_form;

    fn is_in_nnf(formula: &str) -> bool {
        let s = formula.as_bytes();
        for i in 0..formula.len() {
            match s[i] {
                b'A'..=b'Z' => continue,
                b'&' | b'|' => continue,
                b'!' => {
                    if i == 0 {
                        println!("Negation at the start of the formula is not allowed.");
                        return false;
                    }
                    match s[i - 1] {
                        b'A'..=b'Z' => continue,
                        _ => {
                            println!("Negation must be followed by a variable not {}, its position is: {}", s[i - 1], i);
                            return false
                        },
                    }
                }
                _ => {
                    println!("Invalid character '{}' in the formula at position {}", s[i], i);
                    return false;
                }
            }
        }
        return true;
    }

    fn test_nnf(input: &str) {
        println!("Testing NNF for input: {}", input);
        let result = negation_normal_form(input);
        println!("Result: {}", result);
        assert!(is_in_nnf(&result), "The result is not in NNF: {}", result);
        let input_tt = input.parse::<Formula>()
                            .expect("Parse error (Invalid formula)")
                            .to_truth_table();
        println!("Input truth table:\n{}", input_tt);
        let result_tt = result.parse::<Formula>()
                            .expect("Parse error (Invalid formula)")
                            .to_truth_table();
        println!("Result truth table:\n{}", result_tt);
        assert_eq!(input_tt, result_tt, "Truth tables do not match for input: {} and result: {}", input_tt, result_tt);
    }

    fn test_nnf_compare(input: &str, expected: &str) {
        println!("Testing NNF for input: {}", input);
        let result = negation_normal_form(input);
        println!("Result: {}", result);
        assert_eq!(result, expected, "Expected NNF result '{}' but got '{}'", expected, result);
        assert!(is_in_nnf(&result), "The result is not in NNF: {}", result);
    }
    
    fn double_test_nnf(input: &str, expected: &str) {
        test_nnf(input);

        test_nnf_compare(input, expected);
        test_nnf(input);
    }

    #[test]
    fn test_nnf_basic() {
        test_nnf("A");
        test_nnf("A B &");
        test_nnf("A B |");
        test_nnf("A B ^");
        test_nnf("A B >");
        test_nnf("A B =");
    }

    #[test]
    fn test_nnf_negation() {
        test_nnf("A!");
        test_nnf("A B &!");
        test_nnf("A B |!");
        test_nnf("A B ^!");
        test_nnf("A B >!");
        test_nnf("A B =!");
    }


//     println!("{}", negation_normal_form("AB&!"));
// // A!B!|
// println!("{}", negation_normal_form("AB|!"));
// // A!B!&
// println!("{}", negation_normal_form("AB>"));
// // A!B|
// println!("{}", negation_normal_form("AB="));
// // AB&A!B!&|
// println!("{}", negation_normal_form("AB|C&!"));
// // A!B!&C!|
    #[test]
    fn test_nnf_subject() { 
        double_test_nnf("AB&!", "A!B!|");
        double_test_nnf("AB|!", "A!B!&");
        double_test_nnf("AB>!", "AB!&"); // subject example is "A!B|", I think mine is prettier
        double_test_nnf("AB=!", "A!B&");
        // double_test_nnf("AB|C&!", "A!B!&C!");
        
        
    }

    // #[test]
    // fn test_nnf_extra() {
    //     test_nnf("A B C & |");
    //     test_nnf("A B C D & | E F & |");
    //     test_nnf("A B C D & | E F & G H & | I J & |");
    //     test_nnf("A B C D & | E F & G H & | I J & K L & |");
    // }

    #[test]
    fn test_nnf_complex() {
        test_nnf("A B & C | D >");
        test_nnf("A B | C D & E = > F |");
        test_nnf("A B & C D | E F & G > H | I =|!|");
        test_nnf("A B & !C D | E F & G > H | I =|!|");
    }
}