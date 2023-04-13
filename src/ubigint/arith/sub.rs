use crate::UBigInt;
use crate::utils::remove_suffix_0;

impl UBigInt {

    /// it panics when `other` > `self` (TODO: it doesn't panic on release mode)
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sub_ubi(&self, other: &UBigInt) -> Self {
        let mut result = self.clone();
        result.sub_ubi_mut(other);

        result
    }

    /// it panics when `other` > `self` (TODO: it doesn't panic on release mode)
    pub fn sub_ubi_mut(&mut self, other: &UBigInt) {
        let mut carry = false;

        for i in 0..other.len() {

            if carry {

                if other.0[i] != u32::MAX && self.0[i] >= other.0[i] + 1 {
                    self.0[i] -= other.0[i] + 1;
                    carry = false;
                }

                else {
                    self.0[i] = u32::MAX - (other.0[i] - self.0[i]);
                }

            }

            else {

                if self.0[i] >= other.0[i] {
                    self.0[i] -= other.0[i];
                }

                else {
                    self.0[i] = u32::MAX - (other.0[i] - self.0[i]) + 1;
                    carry = true;
                }

            }

        }

        if carry {

            for i in other.len()..self.len() {

                if self.0[i] > 0 {
                    self.0[i] -= 1;
                    break;
                }

                else {
                    self.0[i] = u32::MAX;
                }

            }

        }

        remove_suffix_0(&mut self.0);
        #[cfg(test)] assert!(self.is_valid());
    }

    /// it panics when `other` > `self`
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sub_u32(&self, other: u32) -> Self {
        let mut result = self.clone();
        result.sub_u32_mut(other);

        #[cfg(test)] {
            let t = self.sub_ubi(&UBigInt::from_u32(other));
            assert_eq!(t, result);
            assert!(result.is_valid());
        }

        result
    }

    /// it panics when `other` > `self`
    pub fn sub_u32_mut(&mut self, other: u32) {

        if self.0[0] >= other {
            self.0[0] -= other;
        }

        else if self.len() > 1 {
            let mut i = 1;

            while self.0[i] == 0 {
                self.0[i] = u32::MAX;
                i += 1;
            }


            self.0[i] -= 1;
            self.0[0] = u32::MAX - other + self.0[0] + 1;

            remove_suffix_0(&mut self.0);
            #[cfg(test)] assert!(self.is_valid());
        }

        else {
            panic!("attempt to subtract with overflow");
        }

    }

}

#[cfg(test)]
mod tests {
    use crate::UBigInt;

    #[test]
    fn sub_carry_test() {
        assert_eq!(
            UBigInt::from_raw(vec![0, 0, 0, 1]).sub_ubi(&UBigInt::from_u32(1)),
            UBigInt::from_raw(vec![u32::MAX, u32::MAX, u32::MAX])
        );

        let mut pow2 = UBigInt::from_u32(256);

        for i in 1..=16 {
            pow2.mul_u32_mut(256);
            pow2.sub_u32_mut(1);
            pow2.add_u32_mut(1);
            pow2.add_u32_mut(1);
            pow2.sub_u32_mut(1);
            pow2.sub_u32_mut(1);
            pow2.add_u32_mut(1);
            assert_eq!(pow2, UBigInt::from_u32(256).pow_u32(i + 1));
        }

    }

}