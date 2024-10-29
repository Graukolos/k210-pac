#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rbr_dll_thr: RbrDllThr,
    dlh_ier: DlhIer,
    fcr_iir: FcrIir,
    lcr: Lcr,
    mcr: Mcr,
    lsr: Lsr,
    msr: Msr,
    scr: Scr,
    lpdll: Lpdll,
    lpdlh: Lpdlh,
    _reserved10: [u8; 0x08],
    srbr_sthr: [SrbrSthr; 16],
    far: Far,
    tfr: Tfr,
    rfw: Rfw,
    usr: Usr,
    tfl: Tfl,
    rfl: Rfl,
    srr: Srr,
    srts: Srts,
    sbcr: Sbcr,
    sdmam: Sdmam,
    sfe: Sfe,
    srt: Srt,
    stet: Stet,
    htx: Htx,
    dmasa: Dmasa,
    tcr: Tcr,
    de_en: DeEn,
    re_en: ReEn,
    det: Det,
    tat: Tat,
    dlf: Dlf,
    rar: Rar,
    tar: Tar,
    lcr_ext: LcrExt,
    _reserved35: [u8; 0x24],
    cpr: Cpr,
    ucv: Ucv,
    ctr: Ctr,
}
impl RegisterBlock {
    #[doc = "0x00 - Receive Buffer Register / Divisor Latch (Low) / Transmit Holding Register (depending on context and R/W)"]
    #[inline(always)]
    pub const fn rbr_dll_thr(&self) -> &RbrDllThr {
        &self.rbr_dll_thr
    }
    #[doc = "0x04 - Divisor Latch (High) / Interrupt Enable Register"]
    #[inline(always)]
    pub const fn dlh_ier(&self) -> &DlhIer {
        &self.dlh_ier
    }
    #[doc = "0x08 - FIFO Control Register / Interrupt Identification Register"]
    #[inline(always)]
    pub const fn fcr_iir(&self) -> &FcrIir {
        &self.fcr_iir
    }
    #[doc = "0x0c - Line Control Register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x10 - Modem Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x14 - Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0x18 - Modem Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x1c - Scratchpad Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x20 - Low Power Divisor Latch (Low) Register"]
    #[inline(always)]
    pub const fn lpdll(&self) -> &Lpdll {
        &self.lpdll
    }
    #[doc = "0x24 - Low Power Divisor Latch (High) Register"]
    #[inline(always)]
    pub const fn lpdlh(&self) -> &Lpdlh {
        &self.lpdlh
    }
    #[doc = "0x30..0x70 - Shadow Receive Buffer Register / Shadow Transmit Holding Register (depending on R/W)"]
    #[inline(always)]
    pub const fn srbr_sthr(&self, n: usize) -> &SrbrSthr {
        &self.srbr_sthr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x70 - Shadow Receive Buffer Register / Shadow Transmit Holding Register (depending on R/W)"]
    #[inline(always)]
    pub fn srbr_sthr_iter(&self) -> impl Iterator<Item = &SrbrSthr> {
        self.srbr_sthr.iter()
    }
    #[doc = "0x70 - FIFO Access Register"]
    #[inline(always)]
    pub const fn far(&self) -> &Far {
        &self.far
    }
    #[doc = "0x74 - Transmit FIFO Read Register"]
    #[inline(always)]
    pub const fn tfr(&self) -> &Tfr {
        &self.tfr
    }
    #[doc = "0x78 - Receive FIFO Write Register"]
    #[inline(always)]
    pub const fn rfw(&self) -> &Rfw {
        &self.rfw
    }
    #[doc = "0x7c - UART Status Register"]
    #[inline(always)]
    pub const fn usr(&self) -> &Usr {
        &self.usr
    }
    #[doc = "0x80 - Transmit FIFO Level"]
    #[inline(always)]
    pub const fn tfl(&self) -> &Tfl {
        &self.tfl
    }
    #[doc = "0x84 - Receive FIFO Level"]
    #[inline(always)]
    pub const fn rfl(&self) -> &Rfl {
        &self.rfl
    }
    #[doc = "0x88 - Software Reset Register"]
    #[inline(always)]
    pub const fn srr(&self) -> &Srr {
        &self.srr
    }
    #[doc = "0x8c - Shadow Request to Send Register"]
    #[inline(always)]
    pub const fn srts(&self) -> &Srts {
        &self.srts
    }
    #[doc = "0x90 - Shadow Break Control Register"]
    #[inline(always)]
    pub const fn sbcr(&self) -> &Sbcr {
        &self.sbcr
    }
    #[doc = "0x94 - Shadow DMA Mode"]
    #[inline(always)]
    pub const fn sdmam(&self) -> &Sdmam {
        &self.sdmam
    }
    #[doc = "0x98 - Shadow FIFO Enable"]
    #[inline(always)]
    pub const fn sfe(&self) -> &Sfe {
        &self.sfe
    }
    #[doc = "0x9c - Shadow RCVR Trigger Register"]
    #[inline(always)]
    pub const fn srt(&self) -> &Srt {
        &self.srt
    }
    #[doc = "0xa0 - Shadow TX Empty Trigger Register"]
    #[inline(always)]
    pub const fn stet(&self) -> &Stet {
        &self.stet
    }
    #[doc = "0xa4 - Halt TX Regster"]
    #[inline(always)]
    pub const fn htx(&self) -> &Htx {
        &self.htx
    }
    #[doc = "0xa8 - DMA Software Acknowledge Register"]
    #[inline(always)]
    pub const fn dmasa(&self) -> &Dmasa {
        &self.dmasa
    }
    #[doc = "0xac - Transfer Control Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0xb0 - DE Enable Register"]
    #[inline(always)]
    pub const fn de_en(&self) -> &DeEn {
        &self.de_en
    }
    #[doc = "0xb4 - RE Enable Register"]
    #[inline(always)]
    pub const fn re_en(&self) -> &ReEn {
        &self.re_en
    }
    #[doc = "0xb8 - DE Assertion Time Register"]
    #[inline(always)]
    pub const fn det(&self) -> &Det {
        &self.det
    }
    #[doc = "0xbc - Turn-Around Time Register"]
    #[inline(always)]
    pub const fn tat(&self) -> &Tat {
        &self.tat
    }
    #[doc = "0xc0 - Divisor Latch (Fractional) Register"]
    #[inline(always)]
    pub const fn dlf(&self) -> &Dlf {
        &self.dlf
    }
    #[doc = "0xc4 - Receive-Mode Address Register"]
    #[inline(always)]
    pub const fn rar(&self) -> &Rar {
        &self.rar
    }
    #[doc = "0xc8 - Transmit-Mode Address Register"]
    #[inline(always)]
    pub const fn tar(&self) -> &Tar {
        &self.tar
    }
    #[doc = "0xcc - Line Control Register (Extended)"]
    #[inline(always)]
    pub const fn lcr_ext(&self) -> &LcrExt {
        &self.lcr_ext
    }
    #[doc = "0xf4 - Component Parameter Register"]
    #[inline(always)]
    pub const fn cpr(&self) -> &Cpr {
        &self.cpr
    }
    #[doc = "0xf8 - UART Component Version"]
    #[inline(always)]
    pub const fn ucv(&self) -> &Ucv {
        &self.ucv
    }
    #[doc = "0xfc - Component Type Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
}
#[doc = "rbr_dll_thr (rw) register accessor: Receive Buffer Register / Divisor Latch (Low) / Transmit Holding Register (depending on context and R/W)\n\nYou can [`read`](crate::Reg::read) this register and get [`rbr_dll_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbr_dll_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr_dll_thr`]
module"]
#[doc(alias = "rbr_dll_thr")]
pub type RbrDllThr = crate::Reg<rbr_dll_thr::RbrDllThrSpec>;
#[doc = "Receive Buffer Register / Divisor Latch (Low) / Transmit Holding Register (depending on context and R/W)"]
pub mod rbr_dll_thr;
#[doc = "dlh_ier (rw) register accessor: Divisor Latch (High) / Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dlh_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlh_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlh_ier`]
module"]
#[doc(alias = "dlh_ier")]
pub type DlhIer = crate::Reg<dlh_ier::DlhIerSpec>;
#[doc = "Divisor Latch (High) / Interrupt Enable Register"]
pub mod dlh_ier;
#[doc = "fcr_iir (rw) register accessor: FIFO Control Register / Interrupt Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcr_iir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr_iir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr_iir`]
module"]
#[doc(alias = "fcr_iir")]
pub type FcrIir = crate::Reg<fcr_iir::FcrIirSpec>;
#[doc = "FIFO Control Register / Interrupt Identification Register"]
pub mod fcr_iir;
#[doc = "lcr (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`]
module"]
#[doc(alias = "lcr")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "mcr (rw) register accessor: Modem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
#[doc(alias = "mcr")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Modem Control Register"]
pub mod mcr;
#[doc = "lsr (rw) register accessor: Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
#[doc(alias = "lsr")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "msr (rw) register accessor: Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
#[doc(alias = "msr")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Modem Status Register"]
pub mod msr;
#[doc = "scr (rw) register accessor: Scratchpad Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "scr")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Scratchpad Register"]
pub mod scr;
#[doc = "lpdll (rw) register accessor: Low Power Divisor Latch (Low) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpdll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdll`]
module"]
#[doc(alias = "lpdll")]
pub type Lpdll = crate::Reg<lpdll::LpdllSpec>;
#[doc = "Low Power Divisor Latch (Low) Register"]
pub mod lpdll;
#[doc = "lpdlh (rw) register accessor: Low Power Divisor Latch (High) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpdlh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdlh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdlh`]
module"]
#[doc(alias = "lpdlh")]
pub type Lpdlh = crate::Reg<lpdlh::LpdlhSpec>;
#[doc = "Low Power Divisor Latch (High) Register"]
pub mod lpdlh;
#[doc = "srbr_sthr (rw) register accessor: Shadow Receive Buffer Register / Shadow Transmit Holding Register (depending on R/W)\n\nYou can [`read`](crate::Reg::read) this register and get [`srbr_sthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srbr_sthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr_sthr`]
module"]
#[doc(alias = "srbr_sthr")]
pub type SrbrSthr = crate::Reg<srbr_sthr::SrbrSthrSpec>;
#[doc = "Shadow Receive Buffer Register / Shadow Transmit Holding Register (depending on R/W)"]
pub mod srbr_sthr;
#[doc = "far (rw) register accessor: FIFO Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@far`]
module"]
#[doc(alias = "far")]
pub type Far = crate::Reg<far::FarSpec>;
#[doc = "FIFO Access Register"]
pub mod far;
#[doc = "tfr (rw) register accessor: Transmit FIFO Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfr`]
module"]
#[doc(alias = "tfr")]
pub type Tfr = crate::Reg<tfr::TfrSpec>;
#[doc = "Transmit FIFO Read Register"]
pub mod tfr;
#[doc = "rfw (rw) register accessor: Receive FIFO Write Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfw`]
module"]
#[doc(alias = "rfw")]
pub type Rfw = crate::Reg<rfw::RfwSpec>;
#[doc = "Receive FIFO Write Register"]
pub mod rfw;
#[doc = "usr (rw) register accessor: UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usr`]
module"]
#[doc(alias = "usr")]
pub type Usr = crate::Reg<usr::UsrSpec>;
#[doc = "UART Status Register"]
pub mod usr;
#[doc = "tfl (rw) register accessor: Transmit FIFO Level\n\nYou can [`read`](crate::Reg::read) this register and get [`tfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfl`]
module"]
#[doc(alias = "tfl")]
pub type Tfl = crate::Reg<tfl::TflSpec>;
#[doc = "Transmit FIFO Level"]
pub mod tfl;
#[doc = "rfl (rw) register accessor: Receive FIFO Level\n\nYou can [`read`](crate::Reg::read) this register and get [`rfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfl`]
module"]
#[doc(alias = "rfl")]
pub type Rfl = crate::Reg<rfl::RflSpec>;
#[doc = "Receive FIFO Level"]
pub mod rfl;
#[doc = "srr (rw) register accessor: Software Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srr`]
module"]
#[doc(alias = "srr")]
pub type Srr = crate::Reg<srr::SrrSpec>;
#[doc = "Software Reset Register"]
pub mod srr;
#[doc = "srts (rw) register accessor: Shadow Request to Send Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srts`]
module"]
#[doc(alias = "srts")]
pub type Srts = crate::Reg<srts::SrtsSpec>;
#[doc = "Shadow Request to Send Register"]
pub mod srts;
#[doc = "sbcr (rw) register accessor: Shadow Break Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbcr`]
module"]
#[doc(alias = "sbcr")]
pub type Sbcr = crate::Reg<sbcr::SbcrSpec>;
#[doc = "Shadow Break Control Register"]
pub mod sbcr;
#[doc = "sdmam (rw) register accessor: Shadow DMA Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmam`]
module"]
#[doc(alias = "sdmam")]
pub type Sdmam = crate::Reg<sdmam::SdmamSpec>;
#[doc = "Shadow DMA Mode"]
pub mod sdmam;
#[doc = "sfe (rw) register accessor: Shadow FIFO Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`sfe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfe`]
module"]
#[doc(alias = "sfe")]
pub type Sfe = crate::Reg<sfe::SfeSpec>;
#[doc = "Shadow FIFO Enable"]
pub mod sfe;
#[doc = "srt (rw) register accessor: Shadow RCVR Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srt`]
module"]
#[doc(alias = "srt")]
pub type Srt = crate::Reg<srt::SrtSpec>;
#[doc = "Shadow RCVR Trigger Register"]
pub mod srt;
#[doc = "stet (rw) register accessor: Shadow TX Empty Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stet::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stet::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stet`]
module"]
#[doc(alias = "stet")]
pub type Stet = crate::Reg<stet::StetSpec>;
#[doc = "Shadow TX Empty Trigger Register"]
pub mod stet;
#[doc = "htx (rw) register accessor: Halt TX Regster\n\nYou can [`read`](crate::Reg::read) this register and get [`htx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htx`]
module"]
#[doc(alias = "htx")]
pub type Htx = crate::Reg<htx::HtxSpec>;
#[doc = "Halt TX Regster"]
pub mod htx;
#[doc = "dmasa (rw) register accessor: DMA Software Acknowledge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasa`]
module"]
#[doc(alias = "dmasa")]
pub type Dmasa = crate::Reg<dmasa::DmasaSpec>;
#[doc = "DMA Software Acknowledge Register"]
pub mod dmasa;
#[doc = "tcr (rw) register accessor: Transfer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
#[doc(alias = "tcr")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Transfer Control Register"]
pub mod tcr;
#[doc = "de_en (rw) register accessor: DE Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`de_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`de_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@de_en`]
module"]
#[doc(alias = "de_en")]
pub type DeEn = crate::Reg<de_en::DeEnSpec>;
#[doc = "DE Enable Register"]
pub mod de_en;
#[doc = "re_en (rw) register accessor: RE Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`re_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`re_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@re_en`]
module"]
#[doc(alias = "re_en")]
pub type ReEn = crate::Reg<re_en::ReEnSpec>;
#[doc = "RE Enable Register"]
pub mod re_en;
#[doc = "det (rw) register accessor: DE Assertion Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`det::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`det::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@det`]
module"]
#[doc(alias = "det")]
pub type Det = crate::Reg<det::DetSpec>;
#[doc = "DE Assertion Time Register"]
pub mod det;
#[doc = "tat (rw) register accessor: Turn-Around Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tat`]
module"]
#[doc(alias = "tat")]
pub type Tat = crate::Reg<tat::TatSpec>;
#[doc = "Turn-Around Time Register"]
pub mod tat;
#[doc = "dlf (rw) register accessor: Divisor Latch (Fractional) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dlf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlf`]
module"]
#[doc(alias = "dlf")]
pub type Dlf = crate::Reg<dlf::DlfSpec>;
#[doc = "Divisor Latch (Fractional) Register"]
pub mod dlf;
#[doc = "rar (rw) register accessor: Receive-Mode Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rar`]
module"]
#[doc(alias = "rar")]
pub type Rar = crate::Reg<rar::RarSpec>;
#[doc = "Receive-Mode Address Register"]
pub mod rar;
#[doc = "tar (rw) register accessor: Transmit-Mode Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar`]
module"]
#[doc(alias = "tar")]
pub type Tar = crate::Reg<tar::TarSpec>;
#[doc = "Transmit-Mode Address Register"]
pub mod tar;
#[doc = "lcr_ext (rw) register accessor: Line Control Register (Extended)\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr_ext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr_ext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr_ext`]
module"]
#[doc(alias = "lcr_ext")]
pub type LcrExt = crate::Reg<lcr_ext::LcrExtSpec>;
#[doc = "Line Control Register (Extended)"]
pub mod lcr_ext;
#[doc = "cpr (rw) register accessor: Component Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpr`]
module"]
#[doc(alias = "cpr")]
pub type Cpr = crate::Reg<cpr::CprSpec>;
#[doc = "Component Parameter Register"]
pub mod cpr;
#[doc = "ucv (rw) register accessor: UART Component Version\n\nYou can [`read`](crate::Reg::read) this register and get [`ucv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucv`]
module"]
#[doc(alias = "ucv")]
pub type Ucv = crate::Reg<ucv::UcvSpec>;
#[doc = "UART Component Version"]
pub mod ucv;
#[doc = "ctr (rw) register accessor: Component Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
#[doc(alias = "ctr")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "Component Type Register"]
pub mod ctr;
