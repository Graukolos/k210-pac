#[repr(C)]
#[doc = "Target Configuration"]
#[doc(alias = "targets")]
pub struct Targets {
    threshold: Threshold,
    claim: Claim,
    _reserved_end: [u8; 0x0ff8],
}
impl Targets {
    #[doc = "0x00 - Priority Threshold Register"]
    #[inline(always)]
    pub const fn threshold(&self) -> &Threshold {
        &self.threshold
    }
    #[doc = "0x04 - Claim/Complete Register"]
    #[inline(always)]
    pub const fn claim(&self) -> &Claim {
        &self.claim
    }
}
#[doc = "threshold (rw) register accessor: Priority Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@threshold`]
module"]
#[doc(alias = "threshold")]
pub type Threshold = crate::Reg<threshold::ThresholdSpec>;
#[doc = "Priority Threshold Register"]
pub mod threshold;
#[doc = "claim (rw) register accessor: Claim/Complete Register\n\nYou can [`read`](crate::Reg::read) this register and get [`claim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim`]
module"]
#[doc(alias = "claim")]
pub type Claim = crate::Reg<claim::ClaimSpec>;
#[doc = "Claim/Complete Register"]
pub mod claim;
