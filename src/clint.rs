#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    msip: [Msip; 2],
    _reserved1: [u8; 0x3ff8],
    mtimecmp: [Mtimecmp; 2],
    _reserved2: [u8; 0x7fe8],
    mtime: Mtime,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Hart software interrupt register"]
    #[inline(always)]
    pub const fn msip(&self, n: usize) -> &Msip {
        &self.msip[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - Hart software interrupt register"]
    #[inline(always)]
    pub fn msip_iter(&self) -> impl Iterator<Item = &Msip> {
        self.msip.iter()
    }
    #[doc = "0x4000..0x4010 - Hart time comparator register"]
    #[inline(always)]
    pub const fn mtimecmp(&self, n: usize) -> &Mtimecmp {
        &self.mtimecmp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4000..0x4010 - Hart time comparator register"]
    #[inline(always)]
    pub fn mtimecmp_iter(&self) -> impl Iterator<Item = &Mtimecmp> {
        self.mtimecmp.iter()
    }
    #[doc = "0xbff8..0xc000 - Timer register"]
    #[inline(always)]
    pub const fn mtime(&self) -> &Mtime {
        &self.mtime
    }
}
#[doc = "msip (rw) register accessor: Hart software interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`msip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip`]
module"]
#[doc(alias = "msip")]
pub type Msip = crate::Reg<msip::MsipSpec>;
#[doc = "Hart software interrupt register"]
pub mod msip;
#[doc = "mtimecmp (rw) register accessor: Hart time comparator register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp`]
module"]
#[doc(alias = "mtimecmp")]
pub type Mtimecmp = crate::Reg<mtimecmp::MtimecmpSpec>;
#[doc = "Hart time comparator register"]
pub mod mtimecmp;
#[doc = "mtime (rw) register accessor: Timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime`]
module"]
#[doc(alias = "mtime")]
pub type Mtime = crate::Reg<mtime::MtimeSpec>;
#[doc = "Timer register"]
pub mod mtime;
