#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    con: Con,
    tar: Tar,
    sar: Sar,
    _reserved3: [u8; 0x04],
    data_cmd: DataCmd,
    ss_scl_hcnt: SsSclHcnt,
    ss_scl_lcnt: SsSclLcnt,
    _reserved6: [u8; 0x10],
    intr_stat: IntrStat,
    intr_mask: IntrMask,
    raw_intr_stat: RawIntrStat,
    rx_tl: RxTl,
    tx_tl: TxTl,
    clr_intr: ClrIntr,
    clr_rx_under: ClrRxUnder,
    clr_rx_over: ClrRxOver,
    clr_tx_over: ClrTxOver,
    clr_rd_req: ClrRdReq,
    clr_tx_abrt: ClrTxAbrt,
    clr_rx_done: ClrRxDone,
    clr_activity: ClrActivity,
    clr_stop_det: ClrStopDet,
    clr_start_det: ClrStartDet,
    clr_gen_call: ClrGenCall,
    enable: Enable,
    status: Status,
    txflr: Txflr,
    rxflr: Rxflr,
    sda_hold: SdaHold,
    tx_abrt_source: TxAbrtSource,
    _reserved28: [u8; 0x04],
    dma_cr: DmaCr,
    dma_tdlr: DmaTdlr,
    dma_rdlr: DmaRdlr,
    sda_setup: SdaSetup,
    general_call: GeneralCall,
    enable_status: EnableStatus,
    fs_spklen: FsSpklen,
    _reserved35: [u8; 0x50],
    comp_param_1: CompParam1,
    comp_version: CompVersion,
    comp_type: CompType,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn con(&self) -> &Con {
        &self.con
    }
    #[doc = "0x04 - Target Address Register"]
    #[inline(always)]
    pub const fn tar(&self) -> &Tar {
        &self.tar
    }
    #[doc = "0x08 - Slave Address Register"]
    #[inline(always)]
    pub const fn sar(&self) -> &Sar {
        &self.sar
    }
    #[doc = "0x10 - Data Buffer and Command Register"]
    #[inline(always)]
    pub const fn data_cmd(&self) -> &DataCmd {
        &self.data_cmd
    }
    #[doc = "0x14 - Standard Speed Clock SCL High Count Register"]
    #[inline(always)]
    pub const fn ss_scl_hcnt(&self) -> &SsSclHcnt {
        &self.ss_scl_hcnt
    }
    #[doc = "0x18 - Standard Speed Clock SCL Low Count Register"]
    #[inline(always)]
    pub const fn ss_scl_lcnt(&self) -> &SsSclLcnt {
        &self.ss_scl_lcnt
    }
    #[doc = "0x2c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intr_stat(&self) -> &IntrStat {
        &self.intr_stat
    }
    #[doc = "0x30 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x34 - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn raw_intr_stat(&self) -> &RawIntrStat {
        &self.raw_intr_stat
    }
    #[doc = "0x38 - Receive FIFO Threshold Register"]
    #[inline(always)]
    pub const fn rx_tl(&self) -> &RxTl {
        &self.rx_tl
    }
    #[doc = "0x3c - Transmit FIFO Threshold Register"]
    #[inline(always)]
    pub const fn tx_tl(&self) -> &TxTl {
        &self.tx_tl
    }
    #[doc = "0x40 - Clear Combined and Individual Interrupt Register"]
    #[inline(always)]
    pub const fn clr_intr(&self) -> &ClrIntr {
        &self.clr_intr
    }
    #[doc = "0x44 - Clear RX_UNDER Interrupt Register"]
    #[inline(always)]
    pub const fn clr_rx_under(&self) -> &ClrRxUnder {
        &self.clr_rx_under
    }
    #[doc = "0x48 - Clear RX_OVER Interrupt Register"]
    #[inline(always)]
    pub const fn clr_rx_over(&self) -> &ClrRxOver {
        &self.clr_rx_over
    }
    #[doc = "0x4c - Clear TX_OVER Interrupt Register"]
    #[inline(always)]
    pub const fn clr_tx_over(&self) -> &ClrTxOver {
        &self.clr_tx_over
    }
    #[doc = "0x50 - Clear RD_REQ Interrupt Register"]
    #[inline(always)]
    pub const fn clr_rd_req(&self) -> &ClrRdReq {
        &self.clr_rd_req
    }
    #[doc = "0x54 - Clear TX_ABRT Interrupt Register"]
    #[inline(always)]
    pub const fn clr_tx_abrt(&self) -> &ClrTxAbrt {
        &self.clr_tx_abrt
    }
    #[doc = "0x58 - Clear RX_DONE Interrupt Register"]
    #[inline(always)]
    pub const fn clr_rx_done(&self) -> &ClrRxDone {
        &self.clr_rx_done
    }
    #[doc = "0x5c - Clear ACTIVITY Interrupt Register"]
    #[inline(always)]
    pub const fn clr_activity(&self) -> &ClrActivity {
        &self.clr_activity
    }
    #[doc = "0x60 - Clear STOP_DET Interrupt Register"]
    #[inline(always)]
    pub const fn clr_stop_det(&self) -> &ClrStopDet {
        &self.clr_stop_det
    }
    #[doc = "0x64 - Clear START_DET Interrupt Register"]
    #[inline(always)]
    pub const fn clr_start_det(&self) -> &ClrStartDet {
        &self.clr_start_det
    }
    #[doc = "0x68 - I2C Clear GEN_CALL Interrupt Register"]
    #[inline(always)]
    pub const fn clr_gen_call(&self) -> &ClrGenCall {
        &self.clr_gen_call
    }
    #[doc = "0x6c - Enable Register"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x70 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x74 - Transmit FIFO Level Register"]
    #[inline(always)]
    pub const fn txflr(&self) -> &Txflr {
        &self.txflr
    }
    #[doc = "0x78 - Receive FIFO Level Register"]
    #[inline(always)]
    pub const fn rxflr(&self) -> &Rxflr {
        &self.rxflr
    }
    #[doc = "0x7c - SDA Hold Time Length Register"]
    #[inline(always)]
    pub const fn sda_hold(&self) -> &SdaHold {
        &self.sda_hold
    }
    #[doc = "0x80 - Transmit Abort Source Register"]
    #[inline(always)]
    pub const fn tx_abrt_source(&self) -> &TxAbrtSource {
        &self.tx_abrt_source
    }
    #[doc = "0x88 - I2C DMA Control Register"]
    #[inline(always)]
    pub const fn dma_cr(&self) -> &DmaCr {
        &self.dma_cr
    }
    #[doc = "0x8c - DMA Transmit Data Level Register"]
    #[inline(always)]
    pub const fn dma_tdlr(&self) -> &DmaTdlr {
        &self.dma_tdlr
    }
    #[doc = "0x90 - DMA Receive Data Level Register"]
    #[inline(always)]
    pub const fn dma_rdlr(&self) -> &DmaRdlr {
        &self.dma_rdlr
    }
    #[doc = "0x94 - SDA Setup Register"]
    #[inline(always)]
    pub const fn sda_setup(&self) -> &SdaSetup {
        &self.sda_setup
    }
    #[doc = "0x98 - ACK General Call Register"]
    #[inline(always)]
    pub const fn general_call(&self) -> &GeneralCall {
        &self.general_call
    }
    #[doc = "0x9c - Enable Status Register"]
    #[inline(always)]
    pub const fn enable_status(&self) -> &EnableStatus {
        &self.enable_status
    }
    #[doc = "0xa0 - SS, FS or FM+ spike suppression limit"]
    #[inline(always)]
    pub const fn fs_spklen(&self) -> &FsSpklen {
        &self.fs_spklen
    }
    #[doc = "0xf4 - Component Parameter Register 1"]
    #[inline(always)]
    pub const fn comp_param_1(&self) -> &CompParam1 {
        &self.comp_param_1
    }
    #[doc = "0xf8 - Component Version Register"]
    #[inline(always)]
    pub const fn comp_version(&self) -> &CompVersion {
        &self.comp_version
    }
    #[doc = "0xfc - Component Type Register"]
    #[inline(always)]
    pub const fn comp_type(&self) -> &CompType {
        &self.comp_type
    }
}
#[doc = "con (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con`]
module"]
#[doc(alias = "con")]
pub type Con = crate::Reg<con::ConSpec>;
#[doc = "Control Register"]
#[path = "i2c0/con_.rs"]
pub mod con;
#[doc = "tar (rw) register accessor: Target Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar`]
module"]
#[doc(alias = "tar")]
pub type Tar = crate::Reg<tar::TarSpec>;
#[doc = "Target Address Register"]
pub mod tar;
#[doc = "sar (rw) register accessor: Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`]
module"]
#[doc(alias = "sar")]
pub type Sar = crate::Reg<sar::SarSpec>;
#[doc = "Slave Address Register"]
pub mod sar;
#[doc = "data_cmd (rw) register accessor: Data Buffer and Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cmd`]
module"]
#[doc(alias = "data_cmd")]
pub type DataCmd = crate::Reg<data_cmd::DataCmdSpec>;
#[doc = "Data Buffer and Command Register"]
pub mod data_cmd;
#[doc = "ss_scl_hcnt (rw) register accessor: Standard Speed Clock SCL High Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_scl_hcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_scl_hcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_scl_hcnt`]
module"]
#[doc(alias = "ss_scl_hcnt")]
pub type SsSclHcnt = crate::Reg<ss_scl_hcnt::SsSclHcntSpec>;
#[doc = "Standard Speed Clock SCL High Count Register"]
pub mod ss_scl_hcnt;
#[doc = "ss_scl_lcnt (rw) register accessor: Standard Speed Clock SCL Low Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_scl_lcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_scl_lcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_scl_lcnt`]
module"]
#[doc(alias = "ss_scl_lcnt")]
pub type SsSclLcnt = crate::Reg<ss_scl_lcnt::SsSclLcntSpec>;
#[doc = "Standard Speed Clock SCL Low Count Register"]
pub mod ss_scl_lcnt;
#[doc = "intr_stat (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_stat`]
module"]
#[doc(alias = "intr_stat")]
pub type IntrStat = crate::Reg<intr_stat::IntrStatSpec>;
#[doc = "Interrupt Status Register"]
pub mod intr_stat;
#[doc = "intr_mask (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "intr_mask")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod intr_mask;
#[doc = "raw_intr_stat (rw) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_intr_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_intr_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_intr_stat`]
module"]
#[doc(alias = "raw_intr_stat")]
pub type RawIntrStat = crate::Reg<raw_intr_stat::RawIntrStatSpec>;
#[doc = "Raw Interrupt Status Register"]
pub mod raw_intr_stat;
#[doc = "rx_tl (rw) register accessor: Receive FIFO Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_tl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_tl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_tl`]
module"]
#[doc(alias = "rx_tl")]
pub type RxTl = crate::Reg<rx_tl::RxTlSpec>;
#[doc = "Receive FIFO Threshold Register"]
pub mod rx_tl;
#[doc = "tx_tl (rw) register accessor: Transmit FIFO Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_tl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_tl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_tl`]
module"]
#[doc(alias = "tx_tl")]
pub type TxTl = crate::Reg<tx_tl::TxTlSpec>;
#[doc = "Transmit FIFO Threshold Register"]
pub mod tx_tl;
#[doc = "clr_intr (r) register accessor: Clear Combined and Individual Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_intr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_intr`]
module"]
#[doc(alias = "clr_intr")]
pub type ClrIntr = crate::Reg<clr_intr::ClrIntrSpec>;
#[doc = "Clear Combined and Individual Interrupt Register"]
pub mod clr_intr;
#[doc = "clr_rx_under (r) register accessor: Clear RX_UNDER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_rx_under::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_rx_under`]
module"]
#[doc(alias = "clr_rx_under")]
pub type ClrRxUnder = crate::Reg<clr_rx_under::ClrRxUnderSpec>;
#[doc = "Clear RX_UNDER Interrupt Register"]
pub mod clr_rx_under;
#[doc = "clr_rx_over (r) register accessor: Clear RX_OVER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_rx_over::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_rx_over`]
module"]
#[doc(alias = "clr_rx_over")]
pub type ClrRxOver = crate::Reg<clr_rx_over::ClrRxOverSpec>;
#[doc = "Clear RX_OVER Interrupt Register"]
pub mod clr_rx_over;
#[doc = "clr_tx_over (r) register accessor: Clear TX_OVER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_tx_over::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_tx_over`]
module"]
#[doc(alias = "clr_tx_over")]
pub type ClrTxOver = crate::Reg<clr_tx_over::ClrTxOverSpec>;
#[doc = "Clear TX_OVER Interrupt Register"]
pub mod clr_tx_over;
#[doc = "clr_rd_req (r) register accessor: Clear RD_REQ Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_rd_req::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_rd_req`]
module"]
#[doc(alias = "clr_rd_req")]
pub type ClrRdReq = crate::Reg<clr_rd_req::ClrRdReqSpec>;
#[doc = "Clear RD_REQ Interrupt Register"]
pub mod clr_rd_req;
#[doc = "clr_tx_abrt (r) register accessor: Clear TX_ABRT Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_tx_abrt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_tx_abrt`]
module"]
#[doc(alias = "clr_tx_abrt")]
pub type ClrTxAbrt = crate::Reg<clr_tx_abrt::ClrTxAbrtSpec>;
#[doc = "Clear TX_ABRT Interrupt Register"]
pub mod clr_tx_abrt;
#[doc = "clr_rx_done (r) register accessor: Clear RX_DONE Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_rx_done::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_rx_done`]
module"]
#[doc(alias = "clr_rx_done")]
pub type ClrRxDone = crate::Reg<clr_rx_done::ClrRxDoneSpec>;
#[doc = "Clear RX_DONE Interrupt Register"]
pub mod clr_rx_done;
#[doc = "clr_activity (r) register accessor: Clear ACTIVITY Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_activity::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_activity`]
module"]
#[doc(alias = "clr_activity")]
pub type ClrActivity = crate::Reg<clr_activity::ClrActivitySpec>;
#[doc = "Clear ACTIVITY Interrupt Register"]
pub mod clr_activity;
#[doc = "clr_stop_det (r) register accessor: Clear STOP_DET Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_stop_det::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_stop_det`]
module"]
#[doc(alias = "clr_stop_det")]
pub type ClrStopDet = crate::Reg<clr_stop_det::ClrStopDetSpec>;
#[doc = "Clear STOP_DET Interrupt Register"]
pub mod clr_stop_det;
#[doc = "clr_start_det (r) register accessor: Clear START_DET Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_start_det::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_start_det`]
module"]
#[doc(alias = "clr_start_det")]
pub type ClrStartDet = crate::Reg<clr_start_det::ClrStartDetSpec>;
#[doc = "Clear START_DET Interrupt Register"]
pub mod clr_start_det;
#[doc = "clr_gen_call (r) register accessor: I2C Clear GEN_CALL Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_gen_call::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_gen_call`]
module"]
#[doc(alias = "clr_gen_call")]
pub type ClrGenCall = crate::Reg<clr_gen_call::ClrGenCallSpec>;
#[doc = "I2C Clear GEN_CALL Interrupt Register"]
pub mod clr_gen_call;
#[doc = "enable (rw) register accessor: Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "enable")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable Register"]
pub mod enable;
#[doc = "status (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
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
#[doc = "sda_hold (rw) register accessor: SDA Hold Time Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sda_hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sda_hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sda_hold`]
module"]
#[doc(alias = "sda_hold")]
pub type SdaHold = crate::Reg<sda_hold::SdaHoldSpec>;
#[doc = "SDA Hold Time Length Register"]
pub mod sda_hold;
#[doc = "tx_abrt_source (rw) register accessor: Transmit Abort Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_abrt_source::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_abrt_source::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_abrt_source`]
module"]
#[doc(alias = "tx_abrt_source")]
pub type TxAbrtSource = crate::Reg<tx_abrt_source::TxAbrtSourceSpec>;
#[doc = "Transmit Abort Source Register"]
pub mod tx_abrt_source;
#[doc = "dma_cr (rw) register accessor: I2C DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cr`]
module"]
#[doc(alias = "dma_cr")]
pub type DmaCr = crate::Reg<dma_cr::DmaCrSpec>;
#[doc = "I2C DMA Control Register"]
pub mod dma_cr;
#[doc = "dma_tdlr (rw) register accessor: DMA Transmit Data Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tdlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tdlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tdlr`]
module"]
#[doc(alias = "dma_tdlr")]
pub type DmaTdlr = crate::Reg<dma_tdlr::DmaTdlrSpec>;
#[doc = "DMA Transmit Data Level Register"]
pub mod dma_tdlr;
#[doc = "dma_rdlr (rw) register accessor: DMA Receive Data Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rdlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rdlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rdlr`]
module"]
#[doc(alias = "dma_rdlr")]
pub type DmaRdlr = crate::Reg<dma_rdlr::DmaRdlrSpec>;
#[doc = "DMA Receive Data Level Register"]
pub mod dma_rdlr;
#[doc = "sda_setup (rw) register accessor: SDA Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sda_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sda_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sda_setup`]
module"]
#[doc(alias = "sda_setup")]
pub type SdaSetup = crate::Reg<sda_setup::SdaSetupSpec>;
#[doc = "SDA Setup Register"]
pub mod sda_setup;
#[doc = "general_call (rw) register accessor: ACK General Call Register\n\nYou can [`read`](crate::Reg::read) this register and get [`general_call::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`general_call::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@general_call`]
module"]
#[doc(alias = "general_call")]
pub type GeneralCall = crate::Reg<general_call::GeneralCallSpec>;
#[doc = "ACK General Call Register"]
pub mod general_call;
#[doc = "enable_status (r) register accessor: Enable Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_status`]
module"]
#[doc(alias = "enable_status")]
pub type EnableStatus = crate::Reg<enable_status::EnableStatusSpec>;
#[doc = "Enable Status Register"]
pub mod enable_status;
#[doc = "fs_spklen (rw) register accessor: SS, FS or FM+ spike suppression limit\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_spklen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_spklen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_spklen`]
module"]
#[doc(alias = "fs_spklen")]
pub type FsSpklen = crate::Reg<fs_spklen::FsSpklenSpec>;
#[doc = "SS, FS or FM+ spike suppression limit"]
pub mod fs_spklen;
#[doc = "comp_param_1 (r) register accessor: Component Parameter Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_param_1`]
module"]
#[doc(alias = "comp_param_1")]
pub type CompParam1 = crate::Reg<comp_param_1::CompParam1Spec>;
#[doc = "Component Parameter Register 1"]
pub mod comp_param_1;
#[doc = "comp_version (r) register accessor: Component Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_version`]
module"]
#[doc(alias = "comp_version")]
pub type CompVersion = crate::Reg<comp_version::CompVersionSpec>;
#[doc = "Component Version Register"]
pub mod comp_version;
#[doc = "comp_type (r) register accessor: Component Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_type::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_type`]
module"]
#[doc(alias = "comp_type")]
pub type CompType = crate::Reg<comp_type::CompTypeSpec>;
#[doc = "Component Type Register"]
pub mod comp_type;
