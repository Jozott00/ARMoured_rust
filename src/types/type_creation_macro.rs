macro_rules! make_enum {
    ($enum_name:ident, [$(($case:ident, $val:expr)),*]) => {
        #[derive(Debug, Clone, Copy)]
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

        impl Display for $enum_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $enum_name::$case => f.write_str(stringify!($case)),
                    )*
                }
            }
        }
    };
}

pub(crate) use make_enum;
