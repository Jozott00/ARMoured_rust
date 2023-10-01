use bit_seq::{bseq_16, bseq_8};

/// Bitmask Immediate encoding type
pub struct BitmaskImmediate {
    pub n: u8,
    pub imms: u8,
    pub immr: u8,
}

impl BitmaskImmediate {
    pub fn as_u16(&self) -> u16 {
        let n = self.n;
        let imms = self.imms;
        let immr = self.immr;
        bseq_16!(n:1 immr:6 imms:6)
    }

    /// Highly inspired by [Kevin Newton](https://kddnewton.com/2022/08/11/aarch64-bitmask-immediates.html) who
    /// uses the [LLVM implementation](https://github.com/llvm-mirror/llvm/blob/5c95b810cb3a7dee6d49c030363e5bf0bb41427e/lib/Target/AArch64/MCTargetDesc/AArch64AddressingModes.h#L213)
    fn encode(mask: u64) -> Option<BitmaskImmediate> {
        // 0 and max can not be represented
        if mask == 0 || mask == u64::MAX {
            return None;
        }

        let mut imm = mask;
        let mut size = 64;

        // find size of pattern
        loop {
            // from 64 -> 32 -> 16
            size >>= 1;
            let mask = (1 << size) - 1; // size times 1s

            // imm in half (of current size) an check if both sides
            // are equal
            if (imm & mask) != ((imm >> size) & mask) {
                // as they are not equal, we cannot split them up
                // into smaller sizes
                size <<= 1;
                break;
            }

            if size <= 2 {
                // nothing smaller than 2
                break;
            }
        }

        // immr and imms
        let trailing_ones: u32;
        let left_rotations: u32;

        // determine actual immr and imms

        let mask = u64::MAX >> (64 - size);
        imm &= mask;

        if is_shifted_mask(imm) {
            // trivial, as the rotation equals the number of trailing zeros
            // (e.g. 1100 -> trailing_ones=2, left_rotations=2)
            left_rotations = imm.trailing_zeros();
            trailing_ones = (imm >> left_rotations).trailing_ones();
        } else {
            // non trivial case like 1001

            // mask zeros from imm, so we get 0110
            imm |= !mask;
            if !is_shifted_mask(!imm) {
                return None;
            }

            let leading_ones = imm.leading_ones();
            left_rotations = 64 - leading_ones;
            trailing_ones = leading_ones + imm.trailing_ones() - (64 - size);
        }

        // immr is the number of right rotations it takes to get from the
        // matching unrotated pattern to the target value
        let immr = (size - left_rotations) & (size - 1);

        // imms is encoded as the size of the pattern, a 0, and then one less
        // than the number of sequential 1s.
        let imms = (!(size - 1) << 1) | (trailing_ones - 1);

        // n is 1 if the element size is 64-bits, and 0 otherwise.
        let n = ((imms >> 6) & 1) ^ 1;

        Some(BitmaskImmediate {
            n: n as u8,
            imms: bseq_8!(imms:6),
            immr: bseq_8!(immr:6),
        })
    }
}

impl TryFrom<u64> for BitmaskImmediate {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        BitmaskImmediate::encode(value).ok_or(())
    }
}

// Helper functions

/// Is binary representation all 1s?
#[inline(always)]
fn is_mask(imm: u64) -> bool {
    ((imm + 1) & imm) == 0
}

/// Is binary representation some 1s followed by some 0s?
#[inline(always)]
fn is_shifted_mask(imm: u64) -> bool {
    is_mask((imm - 1) | imm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_16_minimum() {
        let bitmask = BitmaskImmediate::try_from(0x0001000100010001);
        assert!(matches!(
            bitmask,
            Ok(BitmaskImmediate {
                n: 0,
                immr: 0b000000,
                imms: 0b100000
            })
        ));
    }

    #[test]
    fn test_size_16_rotated() {
        let bitmask = BitmaskImmediate::try_from(0xff8fff8fff8fff8f);
        assert!(matches!(
            bitmask,
            Ok(BitmaskImmediate {
                n: 0,
                immr: 0b001001,
                imms: 0b101100
            })
        ));
        // 1 00 100100 0 001001 101100 0000100000
    }

    #[test]
    fn test_size_16_maximum() {
        let bitmask = BitmaskImmediate::try_from(0xfffefffefffefffe);
        assert!(matches!(
            bitmask,
            Ok(BitmaskImmediate {
                n: 0,
                immr: 0b001111,
                imms: 0b101110
            })
        ));
    }

    #[test]
    fn test_size_4_rotated() {
        let bitmask = BitmaskImmediate::try_from(
            0b1001100110011001100110011001100110011001100110011001100110011001,
        );
        assert!(matches!(
            bitmask,
            Ok(BitmaskImmediate {
                n: 0,
                immr: 0b000001,
                imms: 0b111001
            })
        ));
    }

    #[test]
    fn test_size_64_minimum() {
        let bitmask = BitmaskImmediate::try_from(0b1);
        assert!(matches!(
            bitmask,
            Ok(BitmaskImmediate {
                n: 1,
                immr: 0b000000,
                imms: 0b000000
            })
        ));
    }
}
