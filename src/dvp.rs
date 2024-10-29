#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dvp_cfg: DvpCfg,
    r_addr: RAddr,
    g_addr: GAddr,
    b_addr: BAddr,
    cmos_cfg: CmosCfg,
    sccb_cfg: SccbCfg,
    sccb_ctl: SccbCtl,
    axi: Axi,
    sts: Sts,
    reverse: Reverse,
    rgb_addr: RgbAddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Config Register"]
    #[inline(always)]
    pub const fn dvp_cfg(&self) -> &DvpCfg {
        &self.dvp_cfg
    }
    #[doc = "0x04 - R_ADDR"]
    #[inline(always)]
    pub const fn r_addr(&self) -> &RAddr {
        &self.r_addr
    }
    #[doc = "0x08 - G_ADDR"]
    #[inline(always)]
    pub const fn g_addr(&self) -> &GAddr {
        &self.g_addr
    }
    #[doc = "0x0c - B_ADDR"]
    #[inline(always)]
    pub const fn b_addr(&self) -> &BAddr {
        &self.b_addr
    }
    #[doc = "0x10 - CMOS Config Register"]
    #[inline(always)]
    pub const fn cmos_cfg(&self) -> &CmosCfg {
        &self.cmos_cfg
    }
    #[doc = "0x14 - SCCB Config Register"]
    #[inline(always)]
    pub const fn sccb_cfg(&self) -> &SccbCfg {
        &self.sccb_cfg
    }
    #[doc = "0x18 - SCCB Control Register"]
    #[inline(always)]
    pub const fn sccb_ctl(&self) -> &SccbCtl {
        &self.sccb_ctl
    }
    #[doc = "0x1c - AXI Register"]
    #[inline(always)]
    pub const fn axi(&self) -> &Axi {
        &self.axi
    }
    #[doc = "0x20 - STS Register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x24 - REVERSE"]
    #[inline(always)]
    pub const fn reverse(&self) -> &Reverse {
        &self.reverse
    }
    #[doc = "0x28 - RGB_ADDR"]
    #[inline(always)]
    pub const fn rgb_addr(&self) -> &RgbAddr {
        &self.rgb_addr
    }
}
#[doc = "dvp_cfg (rw) register accessor: Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp_cfg`]
module"]
#[doc(alias = "dvp_cfg")]
pub type DvpCfg = crate::Reg<dvp_cfg::DvpCfgSpec>;
#[doc = "Config Register"]
pub mod dvp_cfg;
#[doc = "r_addr (rw) register accessor: R_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`r_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_addr`]
module"]
#[doc(alias = "r_addr")]
pub type RAddr = crate::Reg<r_addr::RAddrSpec>;
#[doc = "R_ADDR"]
pub mod r_addr;
#[doc = "g_addr (rw) register accessor: G_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`g_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`g_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g_addr`]
module"]
#[doc(alias = "g_addr")]
pub type GAddr = crate::Reg<g_addr::GAddrSpec>;
#[doc = "G_ADDR"]
pub mod g_addr;
#[doc = "b_addr (rw) register accessor: B_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`b_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_addr`]
module"]
#[doc(alias = "b_addr")]
pub type BAddr = crate::Reg<b_addr::BAddrSpec>;
#[doc = "B_ADDR"]
pub mod b_addr;
#[doc = "cmos_cfg (rw) register accessor: CMOS Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmos_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmos_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmos_cfg`]
module"]
#[doc(alias = "cmos_cfg")]
pub type CmosCfg = crate::Reg<cmos_cfg::CmosCfgSpec>;
#[doc = "CMOS Config Register"]
pub mod cmos_cfg;
#[doc = "sccb_cfg (rw) register accessor: SCCB Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sccb_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccb_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccb_cfg`]
module"]
#[doc(alias = "sccb_cfg")]
pub type SccbCfg = crate::Reg<sccb_cfg::SccbCfgSpec>;
#[doc = "SCCB Config Register"]
pub mod sccb_cfg;
#[doc = "sccb_ctl (rw) register accessor: SCCB Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sccb_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccb_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccb_ctl`]
module"]
#[doc(alias = "sccb_ctl")]
pub type SccbCtl = crate::Reg<sccb_ctl::SccbCtlSpec>;
#[doc = "SCCB Control Register"]
pub mod sccb_ctl;
#[doc = "axi (rw) register accessor: AXI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi`]
module"]
#[doc(alias = "axi")]
pub type Axi = crate::Reg<axi::AxiSpec>;
#[doc = "AXI Register"]
pub mod axi;
#[doc = "sts (rw) register accessor: STS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "sts")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "STS Register"]
pub mod sts;
#[doc = "reverse (rw) register accessor: REVERSE\n\nYou can [`read`](crate::Reg::read) this register and get [`reverse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reverse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reverse`]
module"]
#[doc(alias = "reverse")]
pub type Reverse = crate::Reg<reverse::ReverseSpec>;
#[doc = "REVERSE"]
pub mod reverse;
#[doc = "rgb_addr (rw) register accessor: RGB_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`rgb_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgb_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgb_addr`]
module"]
#[doc(alias = "rgb_addr")]
pub type RgbAddr = crate::Reg<rgb_addr::RgbAddrSpec>;
#[doc = "RGB_ADDR"]
pub mod rgb_addr;
