use crate::types::UImm5;
macro_rules! create_shift {
    ($enum_name:ident, [$(($case:ident, $val:expr)),*]) => {
        #[derive(Debug)]
        pub enum $enum_name {
            $(
                $case,
            )*
        }

        impl From<u8> for $enum_name {
            fn from(num: u8) -> Self {
                match num {
                    $(
                        $val => Self::$case,
                    )*
                    _ => panic!("Invalid value for {}", stringify!($enum_name)),
                }
            }
        }

        impl From<$enum_name> for u8 {
            fn from(enum_value: $enum_name) -> Self {
                match enum_value {
                    $(
                        $enum_name::$case => $val,
                    )*
                }
            }
        }
    };
}

create_shift!(Shift1, [(LSL0, 0), (LSL12, 1)]);

// create_shift!(Shift, [(LSL, 0), (LSR, 1), (ASR, 2), (ROR, 3)]);


#[derive(Debug)]
pub enum Shift4<T> {
    LSL(T),
    LSR(T),
    ASR(T),
    ROR(T),
}

impl<T> From<(u8, T)> for Shift4<T> {
    fn from((i, n): (u8, T)) -> Self {
        match i {
            0b00 => Shift4::LSL(n),
            0b01 => Shift4::LSR(n),
            0b10 => Shift4::ASR(n),
            0b11 => Shift4::ROR(n),
            _ => panic!("shift options in range of 0b00 - 0b11"),
        }
    }
}

impl<T> From<Shift4<T>> for (u8, T) {
    fn from(value: Shift4<T>) -> Self {
        match value {
            Shift4::LSL(n) => (0b00, n),
            Shift4::LSR(n) => (0b01, n),
            Shift4::ASR(n) => (0b10, n),
            Shift4::ROR(n) => (0b11, n),
        }
    }
}


#[derive(Debug)]
pub enum Shift3<T> {
    LSL(T),
    LSR(T),
    ASR(T),
}

impl<T> From<(u8, T)> for Shift3<T> {
    fn from((i, n): (u8, T)) -> Self {
        match i {
            0b00 => Shift3::LSL(n),
            0b01 => Shift3::LSR(n),
            0b10 => Shift3::ASR(n),
            _ => panic!("shift options in range of 0b00 - 0b11"),
        }
    }
}

impl<T> From<Shift3<T>> for (u8, T) {
    fn from(value: Shift3<T>) -> Self {
        match value {
            Shift3::LSL(n) => (0b00, n),
            Shift3::LSR(n) => (0b01, n),
            Shift3::ASR(n) => (0b10, n),
        }
    }
}