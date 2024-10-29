#[repr(C)]
#[doc = "Channel cluster"]
#[doc(alias = "channel")]
pub struct Channel {
    left_rxtx: LeftRxtx,
    right_rxtx: RightRxtx,
    rer: Rer,
    ter: Ter,
    rcr: Rcr,
    tcr: Tcr,
    isr: Isr,
    imr: Imr,
    ror: Ror,
    tor: Tor,
    rfcr: Rfcr,
    tfcr: Tfcr,
    rff: Rff,
    tff: Tff,
    _reserved_end: [u8; 0x08],
}
impl Channel {
    #[doc = "0x00 - Left Receive or Left Transmit Register"]
    #[inline(always)]
    pub const fn left_rxtx(&self) -> &LeftRxtx {
        &self.left_rxtx
    }
    #[doc = "0x04 - Right Receive or Right Transmit Register"]
    #[inline(always)]
    pub const fn right_rxtx(&self) -> &RightRxtx {
        &self.right_rxtx
    }
    #[doc = "0x08 - Receive Enable Register"]
    #[inline(always)]
    pub const fn rer(&self) -> &Rer {
        &self.rer
    }
    #[doc = "0x0c - Transmit Enable Register"]
    #[inline(always)]
    pub const fn ter(&self) -> &Ter {
        &self.ter
    }
    #[doc = "0x10 - Receive Configuration Register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &Rcr {
        &self.rcr
    }
    #[doc = "0x14 - Transmit Configuration Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0x18 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x1c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x20 - Receive Overrun Register"]
    #[inline(always)]
    pub const fn ror(&self) -> &Ror {
        &self.ror
    }
    #[doc = "0x24 - Transmit Overrun Register"]
    #[inline(always)]
    pub const fn tor(&self) -> &Tor {
        &self.tor
    }
    #[doc = "0x28 - Receive FIFO Configuration Register"]
    #[inline(always)]
    pub const fn rfcr(&self) -> &Rfcr {
        &self.rfcr
    }
    #[doc = "0x2c - Transmit FIFO Configuration Register"]
    #[inline(always)]
    pub const fn tfcr(&self) -> &Tfcr {
        &self.tfcr
    }
    #[doc = "0x30 - Receive FIFO Flush Register"]
    #[inline(always)]
    pub const fn rff(&self) -> &Rff {
        &self.rff
    }
    #[doc = "0x34 - Transmit FIFO Flush Register"]
    #[inline(always)]
    pub const fn tff(&self) -> &Tff {
        &self.tff
    }
}
#[doc = "left_rxtx (rw) register accessor: Left Receive or Left Transmit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`left_rxtx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`left_rxtx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@left_rxtx`]
module"]
#[doc(alias = "left_rxtx")]
pub type LeftRxtx = crate::Reg<left_rxtx::LeftRxtxSpec>;
#[doc = "Left Receive or Left Transmit Register"]
pub mod left_rxtx;
#[doc = "right_rxtx (rw) register accessor: Right Receive or Right Transmit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`right_rxtx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`right_rxtx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@right_rxtx`]
module"]
#[doc(alias = "right_rxtx")]
pub type RightRxtx = crate::Reg<right_rxtx::RightRxtxSpec>;
#[doc = "Right Receive or Right Transmit Register"]
pub mod right_rxtx;
#[doc = "rer (rw) register accessor: Receive Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rer`]
module"]
#[doc(alias = "rer")]
pub type Rer = crate::Reg<rer::RerSpec>;
#[doc = "Receive Enable Register"]
pub mod rer;
#[doc = "ter (rw) register accessor: Transmit Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ter`]
module"]
#[doc(alias = "ter")]
pub type Ter = crate::Reg<ter::TerSpec>;
#[doc = "Transmit Enable Register"]
pub mod ter;
#[doc = "rcr (rw) register accessor: Receive Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
#[doc(alias = "rcr")]
pub type Rcr = crate::Reg<rcr::RcrSpec>;
#[doc = "Receive Configuration Register"]
pub mod rcr;
#[doc = "tcr (rw) register accessor: Transmit Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
#[doc(alias = "tcr")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Transmit Configuration Register"]
pub mod tcr;
#[doc = "isr (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "isr")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "imr (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "imr")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ror (r) register accessor: Receive Overrun Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ror::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ror`]
module"]
#[doc(alias = "ror")]
pub type Ror = crate::Reg<ror::RorSpec>;
#[doc = "Receive Overrun Register"]
pub mod ror;
#[doc = "tor (r) register accessor: Transmit Overrun Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tor`]
module"]
#[doc(alias = "tor")]
pub type Tor = crate::Reg<tor::TorSpec>;
#[doc = "Transmit Overrun Register"]
pub mod tor;
#[doc = "rfcr (rw) register accessor: Receive FIFO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcr`]
module"]
#[doc(alias = "rfcr")]
pub type Rfcr = crate::Reg<rfcr::RfcrSpec>;
#[doc = "Receive FIFO Configuration Register"]
pub mod rfcr;
#[doc = "tfcr (rw) register accessor: Transmit FIFO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfcr`]
module"]
#[doc(alias = "tfcr")]
pub type Tfcr = crate::Reg<tfcr::TfcrSpec>;
#[doc = "Transmit FIFO Configuration Register"]
pub mod tfcr;
#[doc = "rff (rw) register accessor: Receive FIFO Flush Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rff`]
module"]
#[doc(alias = "rff")]
pub type Rff = crate::Reg<rff::RffSpec>;
#[doc = "Receive FIFO Flush Register"]
pub mod rff;
#[doc = "tff (rw) register accessor: Transmit FIFO Flush Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tff`]
module"]
#[doc(alias = "tff")]
pub type Tff = crate::Reg<tff::TffSpec>;
#[doc = "Transmit FIFO Flush Register"]
pub mod tff;
