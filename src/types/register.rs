use crate::types::Register;

pub enum RegConstr {
    W(Register),
    X(Register),
}

pub const WZR: Register = 31;
pub const XZR: Register = 31;
pub const RSP: Register = 31;
