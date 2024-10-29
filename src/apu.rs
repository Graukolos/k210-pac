#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch_cfg: ChCfg,
    ctl: Ctl,
    dir_bidx: [DirBidx; 32],
    pre_fir0_coef: [PreFir0Coef; 9],
    post_fir0_coef: [PostFir0Coef; 9],
    pre_fir1_coef: [PreFir1Coef; 9],
    post_fir1_coef: [PostFir1Coef; 9],
    dwsz_cfg: DwszCfg,
    fft_cfg: FftCfg,
    sobuf_dma_rdata: SobufDmaRdata,
    vobuf_dma_rdata: VobufDmaRdata,
    int_stat: IntStat,
    int_mask: IntMask,
    sat_counter: SatCounter,
    sat_limits: SatLimits,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel Config Register"]
    #[inline(always)]
    pub const fn ch_cfg(&self) -> &ChCfg {
        &self.ch_cfg
    }
    #[doc = "0x04 - Control Register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x08..0x88 - Direction Sample Buffer Read Index Configure Register (16 directions * 2 values * 4 indices)"]
    #[inline(always)]
    pub const fn dir_bidx(&self, n: usize) -> &DirBidx {
        &self.dir_bidx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x88 - Direction Sample Buffer Read Index Configure Register (16 directions * 2 values * 4 indices)"]
    #[inline(always)]
    pub fn dir_bidx_iter(&self) -> impl Iterator<Item = &DirBidx> {
        self.dir_bidx.iter()
    }
    #[doc = "0x88..0xac - FIR0 pre-filter coefficients"]
    #[inline(always)]
    pub const fn pre_fir0_coef(&self, n: usize) -> &PreFir0Coef {
        &self.pre_fir0_coef[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88..0xac - FIR0 pre-filter coefficients"]
    #[inline(always)]
    pub fn pre_fir0_coef_iter(&self) -> impl Iterator<Item = &PreFir0Coef> {
        self.pre_fir0_coef.iter()
    }
    #[doc = "0xac..0xd0 - FIR0 post-filter coefficients"]
    #[inline(always)]
    pub const fn post_fir0_coef(&self, n: usize) -> &PostFir0Coef {
        &self.post_fir0_coef[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xac..0xd0 - FIR0 post-filter coefficients"]
    #[inline(always)]
    pub fn post_fir0_coef_iter(&self) -> impl Iterator<Item = &PostFir0Coef> {
        self.post_fir0_coef.iter()
    }
    #[doc = "0xd0..0xf4 - FIR1 pre-filter coeffecients"]
    #[inline(always)]
    pub const fn pre_fir1_coef(&self, n: usize) -> &PreFir1Coef {
        &self.pre_fir1_coef[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd0..0xf4 - FIR1 pre-filter coeffecients"]
    #[inline(always)]
    pub fn pre_fir1_coef_iter(&self) -> impl Iterator<Item = &PreFir1Coef> {
        self.pre_fir1_coef.iter()
    }
    #[doc = "0xf4..0x118 - FIR1 post-filter coefficients"]
    #[inline(always)]
    pub const fn post_fir1_coef(&self, n: usize) -> &PostFir1Coef {
        &self.post_fir1_coef[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf4..0x118 - FIR1 post-filter coefficients"]
    #[inline(always)]
    pub fn post_fir1_coef_iter(&self) -> impl Iterator<Item = &PostFir1Coef> {
        self.post_fir1_coef.iter()
    }
    #[doc = "0x118 - Downsize Config Register"]
    #[inline(always)]
    pub const fn dwsz_cfg(&self) -> &DwszCfg {
        &self.dwsz_cfg
    }
    #[doc = "0x11c - FFT Config Register"]
    #[inline(always)]
    pub const fn fft_cfg(&self) -> &FftCfg {
        &self.fft_cfg
    }
    #[doc = "0x120 - Read register for DMA to sample-out buffers"]
    #[inline(always)]
    pub const fn sobuf_dma_rdata(&self) -> &SobufDmaRdata {
        &self.sobuf_dma_rdata
    }
    #[doc = "0x124 - Read register for DMA to voice-out buffers"]
    #[inline(always)]
    pub const fn vobuf_dma_rdata(&self) -> &VobufDmaRdata {
        &self.vobuf_dma_rdata
    }
    #[doc = "0x128 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_stat(&self) -> &IntStat {
        &self.int_stat
    }
    #[doc = "0x12c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    #[doc = "0x130 - Saturation Counter"]
    #[inline(always)]
    pub const fn sat_counter(&self) -> &SatCounter {
        &self.sat_counter
    }
    #[doc = "0x134 - Saturation Limits"]
    #[inline(always)]
    pub const fn sat_limits(&self) -> &SatLimits {
        &self.sat_limits
    }
}
#[doc = "ch_cfg (rw) register accessor: Channel Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfg`]
module"]
#[doc(alias = "ch_cfg")]
pub type ChCfg = crate::Reg<ch_cfg::ChCfgSpec>;
#[doc = "Channel Config Register"]
pub mod ch_cfg;
#[doc = "ctl (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "ctl")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control Register"]
pub mod ctl;
#[doc = "dir_bidx (rw) register accessor: Direction Sample Buffer Read Index Configure Register (16 directions * 2 values * 4 indices)\n\nYou can [`read`](crate::Reg::read) this register and get [`dir_bidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir_bidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir_bidx`]
module"]
#[doc(alias = "dir_bidx")]
pub type DirBidx = crate::Reg<dir_bidx::DirBidxSpec>;
#[doc = "Direction Sample Buffer Read Index Configure Register (16 directions * 2 values * 4 indices)"]
pub mod dir_bidx;
#[doc = "pre_fir0_coef (rw) register accessor: FIR0 pre-filter coefficients\n\nYou can [`read`](crate::Reg::read) this register and get [`pre_fir0_coef::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pre_fir0_coef::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pre_fir0_coef`]
module"]
#[doc(alias = "pre_fir0_coef")]
pub type PreFir0Coef = crate::Reg<pre_fir0_coef::PreFir0CoefSpec>;
#[doc = "FIR0 pre-filter coefficients"]
pub mod pre_fir0_coef;
#[doc = "post_fir0_coef (rw) register accessor: FIR0 post-filter coefficients\n\nYou can [`read`](crate::Reg::read) this register and get [`post_fir0_coef::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`post_fir0_coef::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@post_fir0_coef`]
module"]
#[doc(alias = "post_fir0_coef")]
pub type PostFir0Coef = crate::Reg<post_fir0_coef::PostFir0CoefSpec>;
#[doc = "FIR0 post-filter coefficients"]
pub mod post_fir0_coef;
#[doc = "pre_fir1_coef (rw) register accessor: FIR1 pre-filter coeffecients\n\nYou can [`read`](crate::Reg::read) this register and get [`pre_fir1_coef::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pre_fir1_coef::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pre_fir1_coef`]
module"]
#[doc(alias = "pre_fir1_coef")]
pub type PreFir1Coef = crate::Reg<pre_fir1_coef::PreFir1CoefSpec>;
#[doc = "FIR1 pre-filter coeffecients"]
pub mod pre_fir1_coef;
#[doc = "post_fir1_coef (rw) register accessor: FIR1 post-filter coefficients\n\nYou can [`read`](crate::Reg::read) this register and get [`post_fir1_coef::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`post_fir1_coef::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@post_fir1_coef`]
module"]
#[doc(alias = "post_fir1_coef")]
pub type PostFir1Coef = crate::Reg<post_fir1_coef::PostFir1CoefSpec>;
#[doc = "FIR1 post-filter coefficients"]
pub mod post_fir1_coef;
#[doc = "dwsz_cfg (rw) register accessor: Downsize Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dwsz_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwsz_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dwsz_cfg`]
module"]
#[doc(alias = "dwsz_cfg")]
pub type DwszCfg = crate::Reg<dwsz_cfg::DwszCfgSpec>;
#[doc = "Downsize Config Register"]
pub mod dwsz_cfg;
#[doc = "fft_cfg (rw) register accessor: FFT Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fft_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fft_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fft_cfg`]
module"]
#[doc(alias = "fft_cfg")]
pub type FftCfg = crate::Reg<fft_cfg::FftCfgSpec>;
#[doc = "FFT Config Register"]
pub mod fft_cfg;
#[doc = "sobuf_dma_rdata (rw) register accessor: Read register for DMA to sample-out buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`sobuf_dma_rdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sobuf_dma_rdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sobuf_dma_rdata`]
module"]
#[doc(alias = "sobuf_dma_rdata")]
pub type SobufDmaRdata = crate::Reg<sobuf_dma_rdata::SobufDmaRdataSpec>;
#[doc = "Read register for DMA to sample-out buffers"]
pub mod sobuf_dma_rdata;
#[doc = "vobuf_dma_rdata (rw) register accessor: Read register for DMA to voice-out buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`vobuf_dma_rdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vobuf_dma_rdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vobuf_dma_rdata`]
module"]
#[doc(alias = "vobuf_dma_rdata")]
pub type VobufDmaRdata = crate::Reg<vobuf_dma_rdata::VobufDmaRdataSpec>;
#[doc = "Read register for DMA to voice-out buffers"]
pub mod vobuf_dma_rdata;
#[doc = "int_stat (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_stat`]
module"]
#[doc(alias = "int_stat")]
pub type IntStat = crate::Reg<int_stat::IntStatSpec>;
#[doc = "Interrupt Status Register"]
pub mod int_stat;
#[doc = "int_mask (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask`]
module"]
#[doc(alias = "int_mask")]
pub type IntMask = crate::Reg<int_mask::IntMaskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod int_mask;
#[doc = "sat_counter (rw) register accessor: Saturation Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`sat_counter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sat_counter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sat_counter`]
module"]
#[doc(alias = "sat_counter")]
pub type SatCounter = crate::Reg<sat_counter::SatCounterSpec>;
#[doc = "Saturation Counter"]
pub mod sat_counter;
#[doc = "sat_limits (rw) register accessor: Saturation Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`sat_limits::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sat_limits::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sat_limits`]
module"]
#[doc(alias = "sat_limits")]
pub type SatLimits = crate::Reg<sat_limits::SatLimitsSpec>;
#[doc = "Saturation Limits"]
pub mod sat_limits;
