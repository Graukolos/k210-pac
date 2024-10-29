#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    git_id: GitId,
    clk_freq: ClkFreq,
    pll0: Pll0,
    pll1: Pll1,
    pll2: Pll2,
    _reserved5: [u8; 0x04],
    pll_lock: PllLock,
    rom_error: RomError,
    clk_sel0: ClkSel0,
    clk_sel1: ClkSel1,
    clk_en_cent: ClkEnCent,
    clk_en_peri: ClkEnPeri,
    soft_reset: SoftReset,
    peri_reset: PeriReset,
    clk_th0: ClkTh0,
    clk_th1: ClkTh1,
    clk_th2: ClkTh2,
    clk_th3: ClkTh3,
    clk_th4: ClkTh4,
    clk_th5: ClkTh5,
    clk_th6: ClkTh6,
    misc: Misc,
    peri: Peri,
    spi_sleep: SpiSleep,
    reset_status: ResetStatus,
    dma_sel0: DmaSel0,
    dma_sel1: DmaSel1,
    power_sel: PowerSel,
}
impl RegisterBlock {
    #[doc = "0x00 - Git short commit id"]
    #[inline(always)]
    pub const fn git_id(&self) -> &GitId {
        &self.git_id
    }
    #[doc = "0x04 - System clock base frequency"]
    #[inline(always)]
    pub const fn clk_freq(&self) -> &ClkFreq {
        &self.clk_freq
    }
    #[doc = "0x08 - PLL0 controller"]
    #[inline(always)]
    pub const fn pll0(&self) -> &Pll0 {
        &self.pll0
    }
    #[doc = "0x0c - PLL1 controller"]
    #[inline(always)]
    pub const fn pll1(&self) -> &Pll1 {
        &self.pll1
    }
    #[doc = "0x10 - PLL2 controller"]
    #[inline(always)]
    pub const fn pll2(&self) -> &Pll2 {
        &self.pll2
    }
    #[doc = "0x18 - PLL lock tester"]
    #[inline(always)]
    pub const fn pll_lock(&self) -> &PllLock {
        &self.pll_lock
    }
    #[doc = "0x1c - AXI ROM detector"]
    #[inline(always)]
    pub const fn rom_error(&self) -> &RomError {
        &self.rom_error
    }
    #[doc = "0x20 - Clock select controller 0"]
    #[inline(always)]
    pub const fn clk_sel0(&self) -> &ClkSel0 {
        &self.clk_sel0
    }
    #[doc = "0x24 - Clock select controller 1"]
    #[inline(always)]
    pub const fn clk_sel1(&self) -> &ClkSel1 {
        &self.clk_sel1
    }
    #[doc = "0x28 - Central clock enable"]
    #[inline(always)]
    pub const fn clk_en_cent(&self) -> &ClkEnCent {
        &self.clk_en_cent
    }
    #[doc = "0x2c - Peripheral clock enable"]
    #[inline(always)]
    pub const fn clk_en_peri(&self) -> &ClkEnPeri {
        &self.clk_en_peri
    }
    #[doc = "0x30 - Soft reset ctrl"]
    #[inline(always)]
    pub const fn soft_reset(&self) -> &SoftReset {
        &self.soft_reset
    }
    #[doc = "0x34 - Peripheral reset controller"]
    #[inline(always)]
    pub const fn peri_reset(&self) -> &PeriReset {
        &self.peri_reset
    }
    #[doc = "0x38 - Clock threshold controller 0"]
    #[inline(always)]
    pub const fn clk_th0(&self) -> &ClkTh0 {
        &self.clk_th0
    }
    #[doc = "0x3c - Clock threshold controller 1"]
    #[inline(always)]
    pub const fn clk_th1(&self) -> &ClkTh1 {
        &self.clk_th1
    }
    #[doc = "0x40 - Clock threshold controller 2"]
    #[inline(always)]
    pub const fn clk_th2(&self) -> &ClkTh2 {
        &self.clk_th2
    }
    #[doc = "0x44 - Clock threshold controller 3"]
    #[inline(always)]
    pub const fn clk_th3(&self) -> &ClkTh3 {
        &self.clk_th3
    }
    #[doc = "0x48 - Clock threshold controller 4"]
    #[inline(always)]
    pub const fn clk_th4(&self) -> &ClkTh4 {
        &self.clk_th4
    }
    #[doc = "0x4c - Clock threshold controller 5"]
    #[inline(always)]
    pub const fn clk_th5(&self) -> &ClkTh5 {
        &self.clk_th5
    }
    #[doc = "0x50 - Clock threshold controller 6"]
    #[inline(always)]
    pub const fn clk_th6(&self) -> &ClkTh6 {
        &self.clk_th6
    }
    #[doc = "0x54 - Miscellaneous controller"]
    #[inline(always)]
    pub const fn misc(&self) -> &Misc {
        &self.misc
    }
    #[doc = "0x58 - Peripheral controller"]
    #[inline(always)]
    pub const fn peri(&self) -> &Peri {
        &self.peri
    }
    #[doc = "0x5c - SPI sleep controller"]
    #[inline(always)]
    pub const fn spi_sleep(&self) -> &SpiSleep {
        &self.spi_sleep
    }
    #[doc = "0x60 - Reset source status"]
    #[inline(always)]
    pub const fn reset_status(&self) -> &ResetStatus {
        &self.reset_status
    }
    #[doc = "0x64 - DMA handshake selector"]
    #[inline(always)]
    pub const fn dma_sel0(&self) -> &DmaSel0 {
        &self.dma_sel0
    }
    #[doc = "0x68 - DMA handshake selector"]
    #[inline(always)]
    pub const fn dma_sel1(&self) -> &DmaSel1 {
        &self.dma_sel1
    }
    #[doc = "0x6c - IO Power Mode Select controller"]
    #[inline(always)]
    pub const fn power_sel(&self) -> &PowerSel {
        &self.power_sel
    }
}
#[doc = "git_id (rw) register accessor: Git short commit id\n\nYou can [`read`](crate::Reg::read) this register and get [`git_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`git_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@git_id`]
module"]
#[doc(alias = "git_id")]
pub type GitId = crate::Reg<git_id::GitIdSpec>;
#[doc = "Git short commit id"]
pub mod git_id;
#[doc = "clk_freq (rw) register accessor: System clock base frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_freq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_freq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_freq`]
module"]
#[doc(alias = "clk_freq")]
pub type ClkFreq = crate::Reg<clk_freq::ClkFreqSpec>;
#[doc = "System clock base frequency"]
pub mod clk_freq;
#[doc = "pll0 (rw) register accessor: PLL0 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0`]
module"]
#[doc(alias = "pll0")]
pub type Pll0 = crate::Reg<pll0::Pll0Spec>;
#[doc = "PLL0 controller"]
pub mod pll0;
#[doc = "pll1 (rw) register accessor: PLL1 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1`]
module"]
#[doc(alias = "pll1")]
pub type Pll1 = crate::Reg<pll1::Pll1Spec>;
#[doc = "PLL1 controller"]
pub mod pll1;
#[doc = "pll2 (rw) register accessor: PLL2 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`pll2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2`]
module"]
#[doc(alias = "pll2")]
pub type Pll2 = crate::Reg<pll2::Pll2Spec>;
#[doc = "PLL2 controller"]
pub mod pll2;
#[doc = "pll_lock (rw) register accessor: PLL lock tester\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_lock`]
module"]
#[doc(alias = "pll_lock")]
pub type PllLock = crate::Reg<pll_lock::PllLockSpec>;
#[doc = "PLL lock tester"]
pub mod pll_lock;
#[doc = "rom_error (rw) register accessor: AXI ROM detector\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_error`]
module"]
#[doc(alias = "rom_error")]
pub type RomError = crate::Reg<rom_error::RomErrorSpec>;
#[doc = "AXI ROM detector"]
pub mod rom_error;
#[doc = "clk_sel0 (rw) register accessor: Clock select controller 0\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_sel0`]
module"]
#[doc(alias = "clk_sel0")]
pub type ClkSel0 = crate::Reg<clk_sel0::ClkSel0Spec>;
#[doc = "Clock select controller 0"]
pub mod clk_sel0;
#[doc = "clk_sel1 (rw) register accessor: Clock select controller 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_sel1`]
module"]
#[doc(alias = "clk_sel1")]
pub type ClkSel1 = crate::Reg<clk_sel1::ClkSel1Spec>;
#[doc = "Clock select controller 1"]
pub mod clk_sel1;
#[doc = "clk_en_cent (rw) register accessor: Central clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en_cent::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en_cent::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en_cent`]
module"]
#[doc(alias = "clk_en_cent")]
pub type ClkEnCent = crate::Reg<clk_en_cent::ClkEnCentSpec>;
#[doc = "Central clock enable"]
pub mod clk_en_cent;
#[doc = "clk_en_peri (rw) register accessor: Peripheral clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en_peri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en_peri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en_peri`]
module"]
#[doc(alias = "clk_en_peri")]
pub type ClkEnPeri = crate::Reg<clk_en_peri::ClkEnPeriSpec>;
#[doc = "Peripheral clock enable"]
pub mod clk_en_peri;
#[doc = "soft_reset (rw) register accessor: Soft reset ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`soft_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_reset`]
module"]
#[doc(alias = "soft_reset")]
pub type SoftReset = crate::Reg<soft_reset::SoftResetSpec>;
#[doc = "Soft reset ctrl"]
pub mod soft_reset;
#[doc = "peri_reset (rw) register accessor: Peripheral reset controller\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_reset`]
module"]
#[doc(alias = "peri_reset")]
pub type PeriReset = crate::Reg<peri_reset::PeriResetSpec>;
#[doc = "Peripheral reset controller"]
pub mod peri_reset;
#[doc = "clk_th0 (rw) register accessor: Clock threshold controller 0\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_th0`]
module"]
#[doc(alias = "clk_th0")]
pub type ClkTh0 = crate::Reg<clk_th0::ClkTh0Spec>;
#[doc = "Clock threshold controller 0"]
pub mod clk_th0;
#[doc = "clk_th1 (rw) register accessor: Clock threshold controller 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_th1`]
module"]
#[doc(alias = "clk_th1")]
pub type ClkTh1 = crate::Reg<clk_th1::ClkTh1Spec>;
#[doc = "Clock threshold controller 1"]
pub mod clk_th1;
#[doc = "clk_th2 (rw) register accessor: Clock threshold controller 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_th2`]
module"]
#[doc(alias = "clk_th2")]
pub type ClkTh2 = crate::Reg<clk_th2::ClkTh2Spec>;
#[doc = "Clock threshold controller 2"]
pub mod clk_th2;
#[doc = "clk_th3 (rw) register accessor: Clock threshold controller 3\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_th3`]
module"]
#[doc(alias = "clk_th3")]
pub type ClkTh3 = crate::Reg<clk_th3::ClkTh3Spec>;
#[doc = "Clock threshold controller 3"]
pub mod clk_th3;
#[doc = "clk_th4 (rw) register accessor: Clock threshold controller 4\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_th4`]
module"]
#[doc(alias = "clk_th4")]
pub type ClkTh4 = crate::Reg<clk_th4::ClkTh4Spec>;
#[doc = "Clock threshold controller 4"]
pub mod clk_th4;
#[doc = "clk_th5 (rw) register accessor: Clock threshold controller 5\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_th5`]
module"]
#[doc(alias = "clk_th5")]
pub type ClkTh5 = crate::Reg<clk_th5::ClkTh5Spec>;
#[doc = "Clock threshold controller 5"]
pub mod clk_th5;
#[doc = "clk_th6 (rw) register accessor: Clock threshold controller 6\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_th6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_th6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_th6`]
module"]
#[doc(alias = "clk_th6")]
pub type ClkTh6 = crate::Reg<clk_th6::ClkTh6Spec>;
#[doc = "Clock threshold controller 6"]
pub mod clk_th6;
#[doc = "misc (rw) register accessor: Miscellaneous controller\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`]
module"]
#[doc(alias = "misc")]
pub type Misc = crate::Reg<misc::MiscSpec>;
#[doc = "Miscellaneous controller"]
pub mod misc;
#[doc = "peri (rw) register accessor: Peripheral controller\n\nYou can [`read`](crate::Reg::read) this register and get [`peri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri`]
module"]
#[doc(alias = "peri")]
pub type Peri = crate::Reg<peri::PeriSpec>;
#[doc = "Peripheral controller"]
pub mod peri;
#[doc = "spi_sleep (rw) register accessor: SPI sleep controller\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sleep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sleep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sleep`]
module"]
#[doc(alias = "spi_sleep")]
pub type SpiSleep = crate::Reg<spi_sleep::SpiSleepSpec>;
#[doc = "SPI sleep controller"]
pub mod spi_sleep;
#[doc = "reset_status (rw) register accessor: Reset source status\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_status`]
module"]
#[doc(alias = "reset_status")]
pub type ResetStatus = crate::Reg<reset_status::ResetStatusSpec>;
#[doc = "Reset source status"]
pub mod reset_status;
#[doc = "dma_sel0 (rw) register accessor: DMA handshake selector\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sel0`]
module"]
#[doc(alias = "dma_sel0")]
pub type DmaSel0 = crate::Reg<dma_sel0::DmaSel0Spec>;
#[doc = "DMA handshake selector"]
pub mod dma_sel0;
#[doc = "dma_sel1 (rw) register accessor: DMA handshake selector\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sel1`]
module"]
#[doc(alias = "dma_sel1")]
pub type DmaSel1 = crate::Reg<dma_sel1::DmaSel1Spec>;
#[doc = "DMA handshake selector"]
pub mod dma_sel1;
#[doc = "power_sel (rw) register accessor: IO Power Mode Select controller\n\nYou can [`read`](crate::Reg::read) this register and get [`power_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_sel`]
module"]
#[doc(alias = "power_sel")]
pub type PowerSel = crate::Reg<power_sel::PowerSelSpec>;
#[doc = "IO Power Mode Select controller"]
pub mod power_sel;
