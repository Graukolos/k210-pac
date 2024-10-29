#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ier: Ier,
    irer: Irer,
    iter: Iter,
    cer: Cer,
    ccr: Ccr,
    rxffr: Rxffr,
    txffr: Txffr,
    _reserved7: [u8; 0x04],
    channel: [Channel; 4],
    _reserved8: [u8; 0xa0],
    rxdma: Rxdma,
    rrxdma: Rrxdma,
    txdma: Txdma,
    rtxdma: Rtxdma,
    _reserved12: [u8; 0x20],
    i2s_comp_param_2: I2sCompParam2,
    i2s_comp_param_1: I2sCompParam1,
    i2s_comp_version_1: I2sCompVersion1,
    i2s_comp_type: I2sCompType,
}
impl RegisterBlock {
    #[doc = "0x00 - Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x04 - Receiver Block Enable Register"]
    #[inline(always)]
    pub const fn irer(&self) -> &Irer {
        &self.irer
    }
    #[doc = "0x08 - Transmitter Block Enable Register"]
    #[inline(always)]
    pub const fn iter(&self) -> &Iter {
        &self.iter
    }
    #[doc = "0x0c - Clock Generation enable"]
    #[inline(always)]
    pub const fn cer(&self) -> &Cer {
        &self.cer
    }
    #[doc = "0x10 - Clock Configuration Register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x14 - Receiver Block FIFO Reset Register"]
    #[inline(always)]
    pub const fn rxffr(&self) -> &Rxffr {
        &self.rxffr
    }
    #[doc = "0x18 - Transmitter Block FIFO Reset Register"]
    #[inline(always)]
    pub const fn txffr(&self) -> &Txffr {
        &self.txffr
    }
    #[doc = "0x20..0x120 - Channel cluster"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        &self.channel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x120 - Channel cluster"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        self.channel.iter()
    }
    #[doc = "0x1c0 - Receiver Block DMA Register"]
    #[inline(always)]
    pub const fn rxdma(&self) -> &Rxdma {
        &self.rxdma
    }
    #[doc = "0x1c4 - Reset Receiver Block DMA Register"]
    #[inline(always)]
    pub const fn rrxdma(&self) -> &Rrxdma {
        &self.rrxdma
    }
    #[doc = "0x1c8 - Transmitter Block DMA Register"]
    #[inline(always)]
    pub const fn txdma(&self) -> &Txdma {
        &self.txdma
    }
    #[doc = "0x1cc - Reset Transmitter Block DMA Register"]
    #[inline(always)]
    pub const fn rtxdma(&self) -> &Rtxdma {
        &self.rtxdma
    }
    #[doc = "0x1f0 - Component Parameter Register 2"]
    #[inline(always)]
    pub const fn i2s_comp_param_2(&self) -> &I2sCompParam2 {
        &self.i2s_comp_param_2
    }
    #[doc = "0x1f4 - Component Parameter Register 1"]
    #[inline(always)]
    pub const fn i2s_comp_param_1(&self) -> &I2sCompParam1 {
        &self.i2s_comp_param_1
    }
    #[doc = "0x1f8 - Component Version Register"]
    #[inline(always)]
    pub const fn i2s_comp_version_1(&self) -> &I2sCompVersion1 {
        &self.i2s_comp_version_1
    }
    #[doc = "0x1fc - Component Type Register"]
    #[inline(always)]
    pub const fn i2s_comp_type(&self) -> &I2sCompType {
        &self.i2s_comp_type
    }
}
#[doc = "ier (rw) register accessor: Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "ier")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Enable Register"]
pub mod ier;
#[doc = "irer (rw) register accessor: Receiver Block Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irer`]
module"]
#[doc(alias = "irer")]
pub type Irer = crate::Reg<irer::IrerSpec>;
#[doc = "Receiver Block Enable Register"]
pub mod irer;
#[doc = "iter (rw) register accessor: Transmitter Block Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iter`]
module"]
#[doc(alias = "iter")]
pub type Iter = crate::Reg<iter::IterSpec>;
#[doc = "Transmitter Block Enable Register"]
pub mod iter;
#[doc = "cer (rw) register accessor: Clock Generation enable\n\nYou can [`read`](crate::Reg::read) this register and get [`cer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cer`]
module"]
#[doc(alias = "cer")]
pub type Cer = crate::Reg<cer::CerSpec>;
#[doc = "Clock Generation enable"]
pub mod cer;
#[doc = "ccr (rw) register accessor: Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
#[doc(alias = "ccr")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Clock Configuration Register"]
pub mod ccr;
#[doc = "rxffr (rw) register accessor: Receiver Block FIFO Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxffr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxffr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxffr`]
module"]
#[doc(alias = "rxffr")]
pub type Rxffr = crate::Reg<rxffr::RxffrSpec>;
#[doc = "Receiver Block FIFO Reset Register"]
pub mod rxffr;
#[doc = "txffr (rw) register accessor: Transmitter Block FIFO Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txffr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txffr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txffr`]
module"]
#[doc(alias = "txffr")]
pub type Txffr = crate::Reg<txffr::TxffrSpec>;
#[doc = "Transmitter Block FIFO Reset Register"]
pub mod txffr;
#[doc = "Channel cluster"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "Channel cluster"]
pub mod channel;
#[doc = "rxdma (rw) register accessor: Receiver Block DMA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma`]
module"]
#[doc(alias = "rxdma")]
pub type Rxdma = crate::Reg<rxdma::RxdmaSpec>;
#[doc = "Receiver Block DMA Register"]
pub mod rxdma;
#[doc = "rrxdma (rw) register accessor: Reset Receiver Block DMA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rrxdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrxdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrxdma`]
module"]
#[doc(alias = "rrxdma")]
pub type Rrxdma = crate::Reg<rrxdma::RrxdmaSpec>;
#[doc = "Reset Receiver Block DMA Register"]
pub mod rrxdma;
#[doc = "txdma (rw) register accessor: Transmitter Block DMA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdma`]
module"]
#[doc(alias = "txdma")]
pub type Txdma = crate::Reg<txdma::TxdmaSpec>;
#[doc = "Transmitter Block DMA Register"]
pub mod txdma;
#[doc = "rtxdma (rw) register accessor: Reset Transmitter Block DMA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtxdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtxdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtxdma`]
module"]
#[doc(alias = "rtxdma")]
pub type Rtxdma = crate::Reg<rtxdma::RtxdmaSpec>;
#[doc = "Reset Transmitter Block DMA Register"]
pub mod rtxdma;
#[doc = "i2s_comp_param_2 (rw) register accessor: Component Parameter Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_param_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_comp_param_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_comp_param_2`]
module"]
#[doc(alias = "i2s_comp_param_2")]
pub type I2sCompParam2 = crate::Reg<i2s_comp_param_2::I2sCompParam2Spec>;
#[doc = "Component Parameter Register 2"]
pub mod i2s_comp_param_2;
#[doc = "i2s_comp_param_1 (rw) register accessor: Component Parameter Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_param_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_comp_param_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_comp_param_1`]
module"]
#[doc(alias = "i2s_comp_param_1")]
pub type I2sCompParam1 = crate::Reg<i2s_comp_param_1::I2sCompParam1Spec>;
#[doc = "Component Parameter Register 1"]
pub mod i2s_comp_param_1;
#[doc = "i2s_comp_version_1 (rw) register accessor: Component Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_version_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_comp_version_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_comp_version_1`]
module"]
#[doc(alias = "i2s_comp_version_1")]
pub type I2sCompVersion1 = crate::Reg<i2s_comp_version_1::I2sCompVersion1Spec>;
#[doc = "Component Version Register"]
pub mod i2s_comp_version_1;
#[doc = "i2s_comp_type (rw) register accessor: Component Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_comp_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_comp_type`]
module"]
#[doc(alias = "i2s_comp_type")]
pub type I2sCompType = crate::Reg<i2s_comp_type::I2sCompTypeSpec>;
#[doc = "Component Type Register"]
pub mod i2s_comp_type;
