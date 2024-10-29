#[repr(C)]
#[doc = "Target Interrupt Enables"]
#[doc(alias = "target_enables")]
pub struct TargetEnables {
    enable: [Enable; 32],
}
impl TargetEnables {
    #[doc = "0x00..0x80 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn enable(&self, n: usize) -> &Enable {
        &self.enable[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn enable_iter(&self) -> impl Iterator<Item = &Enable> {
        self.enable.iter()
    }
}
#[doc = "enable (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "enable")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Interrupt Enable Register"]
pub mod enable;
