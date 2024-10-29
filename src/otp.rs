#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dummy: Dummy,
}
impl RegisterBlock {
    #[doc = "0x00 - Dummy register: this peripheral is not implemented yet"]
    #[inline(always)]
    pub const fn dummy(&self) -> &Dummy {
        &self.dummy
    }
}
#[doc = "dummy (rw) register accessor: Dummy register: this peripheral is not implemented yet\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dummy`]
module"]
#[doc(alias = "dummy")]
pub type Dummy = crate::Reg<dummy::DummySpec>;
#[doc = "Dummy register: this peripheral is not implemented yet"]
pub mod dummy;
