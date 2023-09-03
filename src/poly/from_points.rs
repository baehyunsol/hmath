use crate::{Polynomial, Matrix, Ratio};

/// f(a) = v1, f(b) = v2, f'(a) = v3, f'(b) = v4\
/// It ignores `v2` and `v4` if `a == b`.
pub fn cubic_2_points(a: &Ratio, b: &Ratio, v1: &Ratio, v2: &Ratio, v3: &Ratio, v4: &Ratio) -> Polynomial {
    // f(x) = r1 x^3 + r2 x^2 + r3 x + r4
    // r1 a^3 + r2 a^2 + r3 a + r4 = v1
    // r1 b^3 + r2 b^2 + r3 b + r4 = v2
    // 3 r1 a^2 + 2 r2 a + r3 = v3
    // 3 r1 b^2 + 2 r2 b + r3 = v4

    // |aaa  aa   a    1|   |r1|  =  |v1|
    // |bbb  bb   b    1|   |r2|     |v2|
    // |3aa  2a   1    0|   |r3|     |v3|
    // |3bb  2b   1    0|   |r4|     |v4|

    // [v1 v2 v3 v4] * Inv([[aaa aa a 1] [bbb bb b 1] [3aa 2a 1 0] [3bb 2b 1 0]]) = [r1 r2 r3 r4]

    let mat1 = Matrix::from_vec(vec![
        vec![v1.clone()],
        vec![v2.clone()],
        vec![v3.clone()],
        vec![v4.clone()],
    ]).unwrap();

    let aa = a.mul_rat(a);
    let bb = b.mul_rat(b);

    let mat2 = match Matrix::from_vec(vec![
        vec![aa.mul_rat(a), aa.clone(), a.clone(), Ratio::one()],
        vec![bb.mul_rat(b), bb.clone(), b.clone(), Ratio::one()],
        vec![aa.mul_i32(3), a.mul_i32(2), Ratio::one(), Ratio::zero()],
        vec![bb.mul_i32(3), b.mul_i32(2), Ratio::one(), Ratio::zero()],
    ]).unwrap().inverse() {
        Ok(m) => m,
        Err(_) if a == b => {
            // v3 (x - a) + v1
            return Polynomial::from_vec(vec![
                v3.clone(),
                v1.sub_rat(&a.mul_rat(v3))
            ]);
        },
        _ => unreachable!(),  // I can't think of this case
    };

    let result = mat2.mul(&mat1).unwrap();

    Polynomial::from_vec(vec![
        result.get(0, 0).clone(),
        result.get(1, 0).clone(),
        result.get(2, 0).clone(),
        result.get(3, 0).clone(),
    ])
}

/// f(a) = v1, f(b) = v2, f(c) = v3\
/// If the input has inconsistent values (eg. f(3) = 4, f(3) = 5), it ignores an arbitrary one.
pub fn quadratic_3_points(a: &Ratio, b: &Ratio, c: &Ratio, v1: &Ratio, v2: &Ratio, v3: &Ratio) -> Polynomial {
    // f(x) = r1 x^2 + r2 x + r3
    // r1 a^2 + r2 a + r3 = v1
    // r1 b^2 + r2 b + r3 = v2
    // r1 c^2 + r2 c + r3 = v3

    // |aa a 1|   |r1|  =  |v1|
    // |bb b 1|   |r2|     |v2|
    // |cc c 1|   |r3|     |v3|

    // [v1 v2 v3] * Inv([[aa a 1] [bb b 1] [cc c 1]]) = [r1 r2 r3]

    let mat1 = Matrix::from_vec(vec![
        vec![v1.clone()],
        vec![v2.clone()],
        vec![v3.clone()],
    ]).unwrap();

    let mat2 = match Matrix::from_vec(vec![
        vec![a.mul_rat(a), a.clone(), Ratio::one()],
        vec![b.mul_rat(b), b.clone(), Ratio::one()],
        vec![c.mul_rat(c), c.clone(), Ratio::one()],
    ]).unwrap().inverse() {
        Ok(m) => m,
        Err(_) => if a == b {
            return linear_2_points(a, c, v1, v3);
        } else {
            return linear_2_points(a, b, v1, v2);
        },
    };

    let result = mat2.mul(&mat1).unwrap();

    Polynomial::from_vec(vec![
        result.get(0, 0).clone(),
        result.get(1, 0).clone(),
        result.get(2, 0).clone(),
    ])
}

/// f(a) = v1, f(b) = v2\
/// If `a == b`, it returns a const function.
pub fn linear_2_points(a: &Ratio, b: &Ratio, v1: &Ratio, v2: &Ratio) -> Polynomial {
    // f(x) = (v2 - v1) / (b - a) * (x - a) + v1

    let tan = if a == b {
        Ratio::zero()
    } else {
        v2.sub_rat(v1).div_rat(&b.sub_rat(a))
    };

    let c = v1.sub_rat(&a.mul_rat(&tan));

    Polynomial::from_vec(vec![tan, c])
}

#[cfg(test)]
mod tests {
    use crate::{cubic_2_points, quadratic_3_points, linear_2_points, Ratio};

    #[test]
    fn sqrt_10_test() {
        let sqrt_approx1 = cubic_2_points(
            &961.into(),
            &1024.into(),
            &31.into(),
            &32.into(),
            &Ratio::from_denom_and_numer_i32(62, 1),
            &Ratio::from_denom_and_numer_i32(64, 1),
        );

        let sqrt_approx2 = quadratic_3_points(
            &961.into(),
            &1024.into(),
            &1089.into(),
            &31.into(),
            &32.into(),
            &33.into(),
        );

        let sqrt_approx3 = linear_2_points(
            &99856.into(),
            &100489.into(),
            &316.into(),
            &317.into(),
        );

        assert_eq!("3.162277", sqrt_approx1.calc(&1000.into()).div_i32(10).to_approx_string(8));
        assert_eq!("3.1622", sqrt_approx2.calc(&1000.into()).div_i32(10).to_approx_string(6));
        assert_eq!("3.16227", sqrt_approx3.calc(&100000.into()).div_i32(100).to_approx_string(7));
    }
}