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
}

/*
*.*..*.. 10100100 164
***.**.* 11101101 237
..****** 00111111 063
.**....* 01100001 097
***...** 11100011 227
 */
