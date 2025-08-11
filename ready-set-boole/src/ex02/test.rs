/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: xmatute- <xmatute-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/13 10:36:49 by xmatute-          #+#    #+#             */
/*   Updated: 2025/08/11 11:28:11 by xmatute-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[cfg(test)]

mod tests {
    use crate::ex02::gray_code::gray_code;

    #[test]
    fn test_gray_code_zero() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(1), 1);
    }

    #[test]
    fn test_gray_code_subject() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(1), 1);
        assert_eq!(gray_code(2), 3);
        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(4), 6);
        assert_eq!(gray_code(5), 7);
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(7), 4);
        assert_eq!(gray_code(8), 12);
    }

    use gray_codes::GrayCode32;

    #[test]
    fn test_gray_code_32_partial() {
        for (i, gray) in Iterator::zip(0..=100, GrayCode32::with_bits(32)) {
            assert_eq!(gray_code(i), gray);
        }
        for (i, gray) in Iterator::zip(4242..4252, GrayCode32::over_range(4242..4252)) {
            assert_eq!(gray_code(i), gray);
        }
        for (i, gray) in Iterator::zip(u32::MAX - 10..u32::MAX, GrayCode32::over_range(u32::MAX - 10..u32::MAX)) {
            assert_eq!(gray_code(i), gray);
        }
    }

    #[test]
    fn test_gray_code_32_random() {
        use rand::Rng;
        let mut rng = rand::rng();
        for _ in 0..100 {
            let start = rng.random_range(0..u32::MAX - 100);
            let end = start + 100;
            for (i, gray) in Iterator::zip(start..end, GrayCode32::over_range(start..end)) {
                assert_eq!(gray_code(i), gray);
            }
        }
    }
}