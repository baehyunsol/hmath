use crate::{Ratio, BigInt, ln2_iter};

/// It returns `ln(x)`. It gets more accurate as `iter` gets bigger. It panics when `x` is less than 0.
pub fn ln_iter(x: &Ratio, iter: usize) -> Ratio {

    if x.is_neg() {
        panic!("logarithm of a negative number is undefined");
    }

    // ln(x) = ln(1 + a) = sum{k=1}{inf} -(-a)^k/k = a - a^2/2 + a^3/3 - a^4/4...
    // it's best when a is close to 0 -> log_2(1 + a) = log_2(x) is close to 0
    // approximation of log_2 is very easily calculated: log2_accurate
    let log2_approx = x.numer.log2_accurate().sub_bi(&x.denom.log2_accurate()).shift_right(1).to_i64().unwrap();
    let mut x_iter = x.clone();
    let mut log2_approx_counter = log2_approx.abs();

    // x /= 2^log2_approx
    if log2_approx > 0 {

        while log2_approx_counter > 0 && x_iter.numer.rem_pow2(2).is_zero() {
            log2_approx_counter -= 1;
            x_iter.numer.div_i32_mut(2);
        }

        if log2_approx_counter % 32 == 31 {
            log2_approx_counter -= 1;
            x_iter.denom.mul_i32_mut(2);
        }

        x_iter.denom.mul_i32_mut((1 << (log2_approx_counter % 32)) as i32);
        x_iter.denom.shift_left_mut((log2_approx_counter / 32) as usize);
    }

    // x *= 2^log2_approx.abs()
    else {

        while log2_approx_counter > 0 && x_iter.denom.rem_pow2(2).is_zero() {
            log2_approx_counter -= 1;
            x_iter.denom.div_i32_mut(2);
        }

        if log2_approx_counter % 32 == 31 {
            log2_approx_counter -= 1;
            x_iter.numer.mul_i32_mut(2);
        }

        x_iter.numer.mul_i32_mut((1 << (log2_approx_counter % 32)) as i32);
        x_iter.numer.shift_left_mut((log2_approx_counter / 32) as usize);
    }

    // now, x = x_iter * 2^log2_approx
    // ln(x) = ln(x_iter) + log2_approx * ln(2)
    x_iter.sub_i32_mut(1);
    let a = x_iter.clone();
    let mut result = a.clone();

    for k in 0..iter {
        x_iter.mul_rat_mut(&a);
        result.sub_rat_mut(&x_iter.div_i32((2 * k + 2) as i32));
        x_iter.mul_rat_mut(&a);
        result.add_rat_mut(&x_iter.div_i32((2 * k + 3) as i32));
    }

    result.add_rat(&ln2_iter(iter).mul_bi(&BigInt::from_i64(log2_approx)))
}

#[cfg(test)]
mod tests {
    use crate::{Ratio, ln_iter, exp_iter};

    #[test]
    fn ln_test() {
        assert_eq!("3.141592", exp_iter(&ln_iter(&Ratio::from_string("3.14159265").unwrap(), 11), 11).to_approx_string(8));
        assert_eq!("9.999999", exp_iter(&ln_iter(&Ratio::from_string("10").unwrap(), 6), 6).to_approx_string(8));
        assert_eq!("1", ln_iter(&Ratio::from_string("2.718281828459045").unwrap(), 6).to_approx_string(8));
        assert_eq!("0.6931471", ln_iter(&Ratio::from_string("2").unwrap(), 6).to_approx_string(9));
        assert_eq!("-1.386294", ln_iter(&Ratio::from_string("0.25").unwrap(), 6).to_approx_string(9));
    }

}