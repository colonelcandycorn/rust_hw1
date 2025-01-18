fn rule110(bits: u8) -> u8 {
    match bits {
        0b111 => 0,
        0b110 => 1,
        0b101 => 1,
        0b100 => 0,
        0b011 => 1,
        0b010 => 1,
        0b001 => 1,
        0b000 => 0,
        _ => panic!("Function only works 3-bits at a time"),
    }
}

fn apply_rule110_over_eight_bits(mut bits: u8) -> u8 {
    let mask: u8 = 0b111;
    let mut output = 0;

    for _ in 0..8 {
        let tmp = rule110(bits & mask);
        output |= tmp;
        output = output.rotate_left(1);
        bits = bits.rotate_left(1);
    }

    output.rotate_left(1)
}

fn u8_to_cellular_automaton_string(mut bits: u8) -> String {
    let mask: u8 = 0b1;
    let mut char_array: [char; 8] = [' '; 8];

    for index in (0..8).rev() {
        let tmp = bits & mask;
        let conv = match tmp {
            0 => '.',
            1 => '*',
            _ => unreachable!("When adding with 1 it is impossible to be anything else")
        };
        char_array[index] = conv;
        bits = bits.rotate_right(1)
    }

    char_array.iter().collect()
}

fn cellular_automaton_string_to_u8(word: &str) -> u8 {
    let mut output: u8 = 0;

    for letter in word.chars().rev() {
        let tmp = match letter {
            '*' => 1,
            '.' => 0,
            _ => panic!("String can only contain * and .")
        };

        output |= tmp;
        output = output.rotate_right(1);
    }

    output
}

fn next_cellular_automaton_string(word: &str) -> String {
    let bits = cellular_automaton_string_to_u8(word);
    let bits_110 = apply_rule110_over_eight_bits(bits);

    u8_to_cellular_automaton_string(bits_110)
}

pub fn display_rule110_n_times(start: &str, n: i32) {
    println!("{start}");
    let mut next = next_cellular_automaton_string(start); // don't really need to do this but I like it

    for _ in 0..n - 1{
        println!("{next}");
        next = next_cellular_automaton_string(&next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_should_zero() {
        let test: u8 = 0b111;
        let expected: u8 = 0b000;

        assert_eq!(rule110(test), expected);
    }

    #[test]
    #[should_panic]
    fn using_more_than_three_bits_should_panic() {
        rule110(0b1000);
    }

    #[test]
    fn two_fifty_five_should_return_zero() {
        let test: u8 = 0b1111_1111;
        let expected: u8 = 0;

        assert_eq!(apply_rule110_over_eight_bits(test), expected);
    }

    #[test]
    fn one_sixty_four_should_return_two_thirty_seven() {
        let test: u8 = 0b1010_0100;
        let expected: u8 = 0b1110_1101;

        assert_eq!(apply_rule110_over_eight_bits(test), expected);
    }

    #[test]
    fn two_thirty_seven_should_return_sixty_three() {
        let test: u8 = 0b1110_1101;
        let expected: u8 = 0b0011_1111;

        assert_eq!(apply_rule110_over_eight_bits(test), expected);
    }

    #[test]
    fn one_sixty_four_should_output_correct_string() {
        let test: u8 = 164;
        let expected = "*.*..*..";

        assert_eq!(u8_to_cellular_automaton_string(test), expected);
    }

    #[test]
    fn two_thirty_seven_should_output_correct_string() {
        let test: u8 = 237;
        let expected = "***.**.*";

        assert_eq!(u8_to_cellular_automaton_string(test), expected)
    }

    #[test]
    fn string_to_one_sixty_four() {
        let test = "*.*..*..";
        let expected: u8 = 164;

        assert_eq!(cellular_automaton_string_to_u8(test), expected);
    }

    #[test]
    fn string_to_two_thirty_seven() {
        let test = "***.**.*";
        let expected: u8 = 237;

        assert_eq!(cellular_automaton_string_to_u8(test), expected);
    }

    #[test]
    fn get_next_cellular_string_164() {
        let test = "*.*..*..";
        let expected = "***.**.*";

        assert_eq!(next_cellular_automaton_string(test), expected);
    }
}

/*
*.*..*.. 10100100 164
***.**.* 11101101 237
..****** 00111111 063
.**....* 01100001 097
***...** 11100011 227
 */
