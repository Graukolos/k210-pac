#[repr(C)]
#[doc = "Channel configuration"]
#[doc(alias = "channel")]
pub struct Channel {
    sar: Sar,
    dar: Dar,
    block_ts: BlockTs,
    ctl: Ctl,
    cfg: Cfg,
    llp: Llp,
    status: Status,
    swhssrc: Swhssrc,
    swhsdst: Swhsdst,
    blk_tfr: BlkTfr,
    axi_id: AxiId,
    axi_qos: AxiQos,
    _reserved12: [u8; 0x20],
    intstatus_en: IntstatusEn,
    intstatus: Intstatus,
    intsignal_en: IntsignalEn,
    intclear: Intclear,
    _reserved_end: [u8; 0x60],
}
impl Channel {
    #[doc = "0x00..0x08 - SAR Address Register"]
    #[inline(always)]
    pub const fn sar(&self) -> &Sar {
        &self.sar
    }
    #[doc = "0x08..0x10 - DAR Address Register"]
    #[inline(always)]
    pub const fn dar(&self) -> &Dar {
        &self.dar
    }
    #[doc = "0x10..0x18 - Block Transfer Size Register"]
    #[inline(always)]
    pub const fn block_ts(&self) -> &BlockTs {
        &self.block_ts
    }
    #[doc = "0x18..0x20 - Control Register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x20..0x28 - Configure Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x28..0x30 - Linked List Pointer register"]
    #[inline(always)]
    pub const fn llp(&self) -> &Llp {
        &self.llp
    }
    #[doc = "0x30..0x38 - Channel Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x38..0x40 - Channel Software handshake Source Register"]
    #[inline(always)]
    pub const fn swhssrc(&self) -> &Swhssrc {
        &self.swhssrc
    }
    #[doc = "0x40..0x48 - Channel Software handshake Destination Register"]
    #[inline(always)]
    pub const fn swhsdst(&self) -> &Swhsdst {
        &self.swhsdst
    }
    #[doc = "0x48..0x50 - Channel Block Transfer Resume Request Register"]
    #[inline(always)]
    pub const fn blk_tfr(&self) -> &BlkTfr {
        &self.blk_tfr
    }
    #[doc = "0x50..0x58 - Channel AXI ID Register"]
    #[inline(always)]
    pub const fn axi_id(&self) -> &AxiId {
        &self.axi_id
    }
    #[doc = "0x58..0x60 - AXI QOS Register"]
    #[inline(always)]
    pub const fn axi_qos(&self) -> &AxiQos {
        &self.axi_qos
    }
    #[doc = "0x80..0x88 - Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn intstatus_en(&self) -> &IntstatusEn {
        &self.intstatus_en
    }
    #[doc = "0x88..0x90 - Channel Interrupt Status Register"]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        &self.intstatus
    }
    #[doc = "0x90..0x98 - Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn intsignal_en(&self) -> &IntsignalEn {
        &self.intsignal_en
    }
    #[doc = "0x98..0xa0 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn intclear(&self) -> &Intclear {
        &self.intclear
    }
}
#[doc = "sar (rw) register accessor: SAR Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`]
module"]
#[doc(alias = "sar")]
pub type Sar = crate::Reg<sar::SarSpec>;
#[doc = "SAR Address Register"]
pub mod sar;
#[doc = "dar (rw) register accessor: DAR Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar`]
module"]
#[doc(alias = "dar")]
pub type Dar = crate::Reg<dar::DarSpec>;
#[doc = "DAR Address Register"]
pub mod dar;
#[doc = "block_ts (rw) register accessor: Block Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`block_ts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_ts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_ts`]
module"]
#[doc(alias = "block_ts")]
pub type BlockTs = crate::Reg<block_ts::BlockTsSpec>;
#[doc = "Block Transfer Size Register"]
pub mod block_ts;
#[doc = "ctl (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "ctl")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control Register"]
pub mod ctl;
#[doc = "cfg (rw) register accessor: Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "cfg")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configure Register"]
pub mod cfg;
#[doc = "llp (rw) register accessor: Linked List Pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`llp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp`]
module"]
#[doc(alias = "llp")]
pub type Llp = crate::Reg<llp::LlpSpec>;
#[doc = "Linked List Pointer register"]
pub mod llp;
#[doc = "status (rw) register accessor: Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Channel Status Register"]
pub mod status;
#[doc = "swhssrc (rw) register accessor: Channel Software handshake Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swhssrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swhssrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swhssrc`]
module"]
#[doc(alias = "swhssrc")]
pub type Swhssrc = crate::Reg<swhssrc::SwhssrcSpec>;
#[doc = "Channel Software handshake Source Register"]
pub mod swhssrc;
#[doc = "swhsdst (rw) register accessor: Channel Software handshake Destination Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swhsdst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swhsdst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swhsdst`]
module"]
#[doc(alias = "swhsdst")]
pub type Swhsdst = crate::Reg<swhsdst::SwhsdstSpec>;
#[doc = "Channel Software handshake Destination Register"]
pub mod swhsdst;
#[doc = "blk_tfr (rw) register accessor: Channel Block Transfer Resume Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_tfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_tfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_tfr`]
module"]
#[doc(alias = "blk_tfr")]
pub type BlkTfr = crate::Reg<blk_tfr::BlkTfrSpec>;
#[doc = "Channel Block Transfer Resume Request Register"]
pub mod blk_tfr;
#[doc = "axi_id (rw) register accessor: Channel AXI ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_id`]
module"]
#[doc(alias = "axi_id")]
pub type AxiId = crate::Reg<axi_id::AxiIdSpec>;
#[doc = "Channel AXI ID Register"]
pub mod axi_id;
#[doc = "axi_qos (rw) register accessor: AXI QOS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_qos`]
module"]
#[doc(alias = "axi_qos")]
pub type AxiQos = crate::Reg<axi_qos::AxiQosSpec>;
#[doc = "AXI QOS Register"]
pub mod axi_qos;
#[doc = "intstatus_en (rw) register accessor: Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatus_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus_en`]
module"]
#[doc(alias = "intstatus_en")]
pub type IntstatusEn = crate::Reg<intstatus_en::IntstatusEnSpec>;
#[doc = "Interrupt Status Enable Register"]
pub mod intstatus_en;
#[doc = "intstatus (rw) register accessor: Channel Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
#[doc(alias = "intstatus")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "Channel Interrupt Status Register"]
pub mod intstatus;
#[doc = "intsignal_en (rw) register accessor: Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsignal_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsignal_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsignal_en`]
module"]
#[doc(alias = "intsignal_en")]
pub type IntsignalEn = crate::Reg<intsignal_en::IntsignalEnSpec>;
#[doc = "Interrupt Signal Enable Register"]
pub mod intsignal_en;
#[doc = "intclear (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclear`]
module"]
#[doc(alias = "intclear")]
pub type Intclear = crate::Reg<intclear::IntclearSpec>;
#[doc = "Interrupt Clear Register"]
pub mod intclear;
