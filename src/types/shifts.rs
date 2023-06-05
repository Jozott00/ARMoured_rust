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

