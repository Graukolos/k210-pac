#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrlr0: Ctrlr0,
    ctrlr1: Ctrlr1,
    ssienr: Ssienr,
    mwcr: Mwcr,
    ser: Ser,
    baudr: Baudr,
    txftlr: Txftlr,
    rxftlr: Rxftlr,
    txflr: Txflr,
    rxflr: Rxflr,
    sr: Sr,
    imr: Imr,
    isr: Isr,
    risr: Risr,
    txoicr: Txoicr,
    rxoicr: Rxoicr,
    rxuicr: Rxuicr,
    msticr: Msticr,
    icr: Icr,
    dmacr: Dmacr,
    dmatdlr: Dmatdlr,
    dmardlr: Dmardlr,
    idr: Idr,
    ssic_version_id: SsicVersionId,
    dr: [Dr; 36],
    rx_sample_delay: RxSampleDelay,
    spi_ctrlr0: SpiCtrlr0,
    _reserved27: [u8; 0x04],
    xip_mode_bits: XipModeBits,
    xip_incr_inst: XipIncrInst,
    xip_wrap_inst: XipWrapInst,
    xip_ctrl: XipCtrl,
    xip_ser: XipSer,
    xrxoicr: Xrxoicr,
    xip_cnt_time_out: XipCntTimeOut,
    endian: Endian,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register 0"]
    #[inline(always)]
    pub const fn ctrlr0(&self) -> &Ctrlr0 {
        &self.ctrlr0
    }
    #[doc = "0x04 - Control Register 1"]
    #[inline(always)]
    pub const fn ctrlr1(&self) -> &Ctrlr1 {
        &self.ctrlr1
    }
    #[doc = "0x08 - Enable Register"]
    #[inline(always)]
    pub const fn ssienr(&self) -> &Ssienr {
        &self.ssienr
    }
    #[doc = "0x0c - Microwire Control Register"]
    #[inline(always)]
    pub const fn mwcr(&self) -> &Mwcr {
        &self.mwcr
    }
    #[doc = "0x10 - Slave Enable Register"]
    #[inline(always)]
    pub const fn ser(&self) -> &Ser {
        &self.ser
    }
    #[doc = "0x14 - Baud Rate Select"]
    #[inline(always)]
    pub const fn baudr(&self) -> &Baudr {
        &self.baudr
    }
    #[doc = "0x18 - Transmit FIFO Threshold Level"]
    #[inline(always)]
    pub const fn txftlr(&self) -> &Txftlr {
        &self.txftlr
    }
    #[doc = "0x1c - Receive FIFO Threshold Level"]
    #[inline(always)]
    pub const fn rxftlr(&self) -> &Rxftlr {
        &self.rxftlr
    }
    #[doc = "0x20 - Transmit FIFO Level Register"]
    #[inline(always)]
    pub const fn txflr(&self) -> &Txflr {
        &self.txflr
    }
    #[doc = "0x24 - Receive FIFO Level Register"]
    #[inline(always)]
    pub const fn rxflr(&self) -> &Rxflr {
        &self.rxflr
    }
    #[doc = "0x28 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x2c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x30 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x34 - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn risr(&self) -> &Risr {
        &self.risr
    }
    #[doc = "0x38 - Transmit FIFO Overflow Interrupt Clear Register"]
    #[inline(always)]
    pub const fn txoicr(&self) -> &Txoicr {
        &self.txoicr
    }
    #[doc = "0x3c - Receive FIFO Overflow Interrupt Clear Register"]
    #[inline(always)]
    pub const fn rxoicr(&self) -> &Rxoicr {
        &self.rxoicr
    }
    #[doc = "0x40 - Receive FIFO Underflow Interrupt Clear Register"]
    #[inline(always)]
    pub const fn rxuicr(&self) -> &Rxuicr {
        &self.rxuicr
    }
    #[doc = "0x44 - Multi-Master Interrupt Clear Register"]
    #[inline(always)]
    pub const fn msticr(&self) -> &Msticr {
        &self.msticr
    }
    #[doc = "0x48 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x4c - DMA Control Register"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &Dmacr {
        &self.dmacr
    }
    #[doc = "0x50 - DMA Transmit Data Level"]
    #[inline(always)]
    pub const fn dmatdlr(&self) -> &Dmatdlr {
        &self.dmatdlr
    }
    #[doc = "0x54 - DMA Receive Data Level"]
    #[inline(always)]
    pub const fn dmardlr(&self) -> &Dmardlr {
        &self.dmardlr
    }
    #[doc = "0x58 - Identification Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x5c - DWC_ssi component version"]
    #[inline(always)]
    pub const fn ssic_version_id(&self) -> &SsicVersionId {
        &self.ssic_version_id
    }
    #[doc = "0x60..0xf0 - Data Register"]
    #[inline(always)]
    pub const fn dr(&self, n: usize) -> &Dr {
        &self.dr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0xf0 - Data Register"]
    #[inline(always)]
    pub fn dr_iter(&self) -> impl Iterator<Item = &Dr> {
        self.dr.iter()
    }
    #[doc = "0xf0 - RX Sample Delay Register"]
    #[inline(always)]
    pub const fn rx_sample_delay(&self) -> &RxSampleDelay {
        &self.rx_sample_delay
    }
    #[doc = "0xf4 - SPI Control Register"]
    #[inline(always)]
    pub const fn spi_ctrlr0(&self) -> &SpiCtrlr0 {
        &self.spi_ctrlr0
    }
    #[doc = "0xfc - XIP Mode bits"]
    #[inline(always)]
    pub const fn xip_mode_bits(&self) -> &XipModeBits {
        &self.xip_mode_bits
    }
    #[doc = "0x100 - XIP INCR transfer opcode"]
    #[inline(always)]
    pub const fn xip_incr_inst(&self) -> &XipIncrInst {
        &self.xip_incr_inst
    }
    #[doc = "0x104 - XIP WRAP transfer opcode"]
    #[inline(always)]
    pub const fn xip_wrap_inst(&self) -> &XipWrapInst {
        &self.xip_wrap_inst
    }
    #[doc = "0x108 - XIP Control Register"]
    #[inline(always)]
    pub const fn xip_ctrl(&self) -> &XipCtrl {
        &self.xip_ctrl
    }
    #[doc = "0x10c - XIP Slave Enable Register"]
    #[inline(always)]
    pub const fn xip_ser(&self) -> &XipSer {
        &self.xip_ser
    }
    #[doc = "0x110 - XIP Receive FIFO Overflow Interrupt Clear Register"]
    #[inline(always)]
    pub const fn xrxoicr(&self) -> &Xrxoicr {
        &self.xrxoicr
    }
    #[doc = "0x114 - XIP time out register for continuous transfers"]
    #[inline(always)]
    pub const fn xip_cnt_time_out(&self) -> &XipCntTimeOut {
        &self.xip_cnt_time_out
    }
    #[doc = "0x118 - ENDIAN"]
    #[inline(always)]
    pub const fn endian(&self) -> &Endian {
        &self.endian
    }
}
#[doc = "ctrlr0 (rw) register accessor: Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlr0`]
module"]
#[doc(alias = "ctrlr0")]
pub type Ctrlr0 = crate::Reg<ctrlr0::Ctrlr0Spec>;
#[doc = "Control Register 0"]
pub mod ctrlr0;
#[doc = "ctrlr1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlr1`]
module"]
#[doc(alias = "ctrlr1")]
pub type Ctrlr1 = crate::Reg<ctrlr1::Ctrlr1Spec>;
#[doc = "Control Register 1"]
pub mod ctrlr1;
#[doc = "ssienr (rw) register accessor: Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssienr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssienr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssienr`]
module"]
#[doc(alias = "ssienr")]
pub type Ssienr = crate::Reg<ssienr::SsienrSpec>;
#[doc = "Enable Register"]
pub mod ssienr;
#[doc = "mwcr (rw) register accessor: Microwire Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mwcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwcr`]
module"]
#[doc(alias = "mwcr")]
pub type Mwcr = crate::Reg<mwcr::MwcrSpec>;
#[doc = "Microwire Control Register"]
pub mod mwcr;
#[doc = "ser (rw) register accessor: Slave Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ser`]
module"]
#[doc(alias = "ser")]
pub type Ser = crate::Reg<ser::SerSpec>;
#[doc = "Slave Enable Register"]
pub mod ser;
#[doc = "baudr (rw) register accessor: Baud Rate Select\n\nYou can [`read`](crate::Reg::read) this register and get [`baudr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baudr`]
module"]
#[doc(alias = "baudr")]
pub type Baudr = crate::Reg<baudr::BaudrSpec>;
#[doc = "Baud Rate Select"]
pub mod baudr;
#[doc = "txftlr (rw) register accessor: Transmit FIFO Threshold Level\n\nYou can [`read`](crate::Reg::read) this register and get [`txftlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txftlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txftlr`]
module"]
#[doc(alias = "txftlr")]
pub type Txftlr = crate::Reg<txftlr::TxftlrSpec>;
#[doc = "Transmit FIFO Threshold Level"]
pub mod txftlr;
#[doc = "rxftlr (rw) register accessor: Receive FIFO Threshold Level\n\nYou can [`read`](crate::Reg::read) this register and get [`rxftlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxftlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxftlr`]
module"]
#[doc(alias = "rxftlr")]
pub type Rxftlr = crate::Reg<rxftlr::RxftlrSpec>;
#[doc = "Receive FIFO Threshold Level"]
pub mod rxftlr;
#[doc = "txflr (rw) register accessor: Transmit FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txflr`]
module"]
#[doc(alias = "txflr")]
pub type Txflr = crate::Reg<txflr::TxflrSpec>;
#[doc = "Transmit FIFO Level Register"]
pub mod txflr;
#[doc = "rxflr (rw) register accessor: Receive FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxflr`]
module"]
#[doc(alias = "rxflr")]
pub type Rxflr = crate::Reg<rxflr::RxflrSpec>;
#[doc = "Receive FIFO Level Register"]
pub mod rxflr;
#[doc = "sr (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "sr")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "imr (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "imr")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "isr (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "isr")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "risr (rw) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`risr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@risr`]
module"]
#[doc(alias = "risr")]
pub type Risr = crate::Reg<risr::RisrSpec>;
#[doc = "Raw Interrupt Status Register"]
pub mod risr;
#[doc = "txoicr (rw) register accessor: Transmit FIFO Overflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txoicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txoicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txoicr`]
module"]
#[doc(alias = "txoicr")]
pub type Txoicr = crate::Reg<txoicr::TxoicrSpec>;
#[doc = "Transmit FIFO Overflow Interrupt Clear Register"]
pub mod txoicr;
#[doc = "rxoicr (rw) register accessor: Receive FIFO Overflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxoicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxoicr`]
module"]
#[doc(alias = "rxoicr")]
pub type Rxoicr = crate::Reg<rxoicr::RxoicrSpec>;
#[doc = "Receive FIFO Overflow Interrupt Clear Register"]
pub mod rxoicr;
#[doc = "rxuicr (rw) register accessor: Receive FIFO Underflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxuicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxuicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxuicr`]
module"]
#[doc(alias = "rxuicr")]
pub type Rxuicr = crate::Reg<rxuicr::RxuicrSpec>;
#[doc = "Receive FIFO Underflow Interrupt Clear Register"]
pub mod rxuicr;
#[doc = "msticr (rw) register accessor: Multi-Master Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msticr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msticr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msticr`]
module"]
#[doc(alias = "msticr")]
pub type Msticr = crate::Reg<msticr::MsticrSpec>;
#[doc = "Multi-Master Interrupt Clear Register"]
pub mod msticr;
#[doc = "icr (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "icr")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "dmacr (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`]
module"]
#[doc(alias = "dmacr")]
pub type Dmacr = crate::Reg<dmacr::DmacrSpec>;
#[doc = "DMA Control Register"]
pub mod dmacr;
#[doc = "dmatdlr (rw) register accessor: DMA Transmit Data Level\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatdlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatdlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatdlr`]
module"]
#[doc(alias = "dmatdlr")]
pub type Dmatdlr = crate::Reg<dmatdlr::DmatdlrSpec>;
#[doc = "DMA Transmit Data Level"]
pub mod dmatdlr;
#[doc = "dmardlr (rw) register accessor: DMA Receive Data Level\n\nYou can [`read`](crate::Reg::read) this register and get [`dmardlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmardlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmardlr`]
module"]
#[doc(alias = "dmardlr")]
pub type Dmardlr = crate::Reg<dmardlr::DmardlrSpec>;
#[doc = "DMA Receive Data Level"]
pub mod dmardlr;
#[doc = "idr (rw) register accessor: Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "idr")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Identification Register"]
pub mod idr;
#[doc = "ssic_version_id (rw) register accessor: DWC_ssi component version\n\nYou can [`read`](crate::Reg::read) this register and get [`ssic_version_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssic_version_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssic_version_id`]
module"]
#[doc(alias = "ssic_version_id")]
pub type SsicVersionId = crate::Reg<ssic_version_id::SsicVersionIdSpec>;
#[doc = "DWC_ssi component version"]
pub mod ssic_version_id;
#[doc = "dr (rw) register accessor: Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
#[doc(alias = "dr")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "Data Register"]
pub mod dr;
#[doc = "rx_sample_delay (rw) register accessor: RX Sample Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_sample_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_sample_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_sample_delay`]
module"]
#[doc(alias = "rx_sample_delay")]
pub type RxSampleDelay = crate::Reg<rx_sample_delay::RxSampleDelaySpec>;
#[doc = "RX Sample Delay Register"]
pub mod rx_sample_delay;
#[doc = "spi_ctrlr0 (rw) register accessor: SPI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_ctrlr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_ctrlr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ctrlr0`]
module"]
#[doc(alias = "spi_ctrlr0")]
pub type SpiCtrlr0 = crate::Reg<spi_ctrlr0::SpiCtrlr0Spec>;
#[doc = "SPI Control Register"]
pub mod spi_ctrlr0;
#[doc = "xip_mode_bits (rw) register accessor: XIP Mode bits\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_mode_bits::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_mode_bits::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_mode_bits`]
module"]
#[doc(alias = "xip_mode_bits")]
pub type XipModeBits = crate::Reg<xip_mode_bits::XipModeBitsSpec>;
#[doc = "XIP Mode bits"]
pub mod xip_mode_bits;
#[doc = "xip_incr_inst (rw) register accessor: XIP INCR transfer opcode\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_incr_inst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_incr_inst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_incr_inst`]
module"]
#[doc(alias = "xip_incr_inst")]
pub type XipIncrInst = crate::Reg<xip_incr_inst::XipIncrInstSpec>;
#[doc = "XIP INCR transfer opcode"]
pub mod xip_incr_inst;
#[doc = "xip_wrap_inst (rw) register accessor: XIP WRAP transfer opcode\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_wrap_inst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_wrap_inst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_wrap_inst`]
module"]
#[doc(alias = "xip_wrap_inst")]
pub type XipWrapInst = crate::Reg<xip_wrap_inst::XipWrapInstSpec>;
#[doc = "XIP WRAP transfer opcode"]
pub mod xip_wrap_inst;
#[doc = "xip_ctrl (rw) register accessor: XIP Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_ctrl`]
module"]
#[doc(alias = "xip_ctrl")]
pub type XipCtrl = crate::Reg<xip_ctrl::XipCtrlSpec>;
#[doc = "XIP Control Register"]
pub mod xip_ctrl;
#[doc = "xip_ser (rw) register accessor: XIP Slave Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_ser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_ser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_ser`]
module"]
#[doc(alias = "xip_ser")]
pub type XipSer = crate::Reg<xip_ser::XipSerSpec>;
#[doc = "XIP Slave Enable Register"]
pub mod xip_ser;
#[doc = "xrxoicr (rw) register accessor: XIP Receive FIFO Overflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xrxoicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xrxoicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xrxoicr`]
module"]
#[doc(alias = "xrxoicr")]
pub type Xrxoicr = crate::Reg<xrxoicr::XrxoicrSpec>;
#[doc = "XIP Receive FIFO Overflow Interrupt Clear Register"]
pub mod xrxoicr;
#[doc = "xip_cnt_time_out (rw) register accessor: XIP time out register for continuous transfers\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_cnt_time_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_cnt_time_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_cnt_time_out`]
module"]
#[doc(alias = "xip_cnt_time_out")]
pub type XipCntTimeOut = crate::Reg<xip_cnt_time_out::XipCntTimeOutSpec>;
#[doc = "XIP time out register for continuous transfers"]
pub mod xip_cnt_time_out;
#[doc = "endian (rw) register accessor: ENDIAN\n\nYou can [`read`](crate::Reg::read) this register and get [`endian::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endian::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endian`]
module"]
#[doc(alias = "endian")]
pub type Endian = crate::Reg<endian::EndianSpec>;
#[doc = "ENDIAN"]
pub mod endian;
