#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    result: [Result; 8],
    data_in: DataIn,
    _reserved2: [u8; 0x04],
    num_reg: NumReg,
    function_reg_0: FunctionReg0,
    _reserved4: [u8; 0x04],
    function_reg_1: FunctionReg1,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Calculated SHA256 return value"]
    #[inline(always)]
    pub const fn result(&self, n: usize) -> &Result {
        &self.result[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Calculated SHA256 return value"]
    #[inline(always)]
    pub fn result_iter(&self) -> impl Iterator<Item = &Result> {
        self.result.iter()
    }
    #[doc = "0x20 - SHA256 input data is written to this register"]
    #[inline(always)]
    pub const fn data_in(&self) -> &DataIn {
        &self.data_in
    }
    #[doc = "0x28 - Counters register"]
    #[inline(always)]
    pub const fn num_reg(&self) -> &NumReg {
        &self.num_reg
    }
    #[doc = "0x2c - Function configuration register 0"]
    #[inline(always)]
    pub const fn function_reg_0(&self) -> &FunctionReg0 {
        &self.function_reg_0
    }
    #[doc = "0x34 - Function configuration register 1"]
    #[inline(always)]
    pub const fn function_reg_1(&self) -> &FunctionReg1 {
        &self.function_reg_1
    }
}
#[doc = "result (rw) register accessor: Calculated SHA256 return value\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`]
module"]
#[doc(alias = "result")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "Calculated SHA256 return value"]
pub mod result;
#[doc = "data_in (rw) register accessor: SHA256 input data is written to this register\n\nYou can [`read`](crate::Reg::read) this register and get [`data_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_in`]
module"]
#[doc(alias = "data_in")]
pub type DataIn = crate::Reg<data_in::DataInSpec>;
#[doc = "SHA256 input data is written to this register"]
pub mod data_in;
#[doc = "num_reg (rw) register accessor: Counters register\n\nYou can [`read`](crate::Reg::read) this register and get [`num_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`num_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@num_reg`]
module"]
#[doc(alias = "num_reg")]
pub type NumReg = crate::Reg<num_reg::NumRegSpec>;
#[doc = "Counters register"]
pub mod num_reg;
#[doc = "function_reg_0 (rw) register accessor: Function configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`function_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`function_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function_reg_0`]
module"]
#[doc(alias = "function_reg_0")]
pub type FunctionReg0 = crate::Reg<function_reg_0::FunctionReg0Spec>;
#[doc = "Function configuration register 0"]
pub mod function_reg_0;
#[doc = "function_reg_1 (rw) register accessor: Function configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`function_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`function_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function_reg_1`]
module"]
#[doc(alias = "function_reg_1")]
pub type FunctionReg1 = crate::Reg<function_reg_1::FunctionReg1Spec>;
#[doc = "Function configuration register 1"]
pub mod function_reg_1;
