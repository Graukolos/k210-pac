#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    torr: Torr,
    ccvr: Ccvr,
    crr: Crr,
    stat: Stat,
    eoi: Eoi,
    _reserved6: [u8; 0x04],
    prot_level: ProtLevel,
    _reserved7: [u8; 0xc4],
    comp_param_5: CompParam5,
    comp_param_4: CompParam4,
    comp_param_3: CompParam3,
    comp_param_2: CompParam2,
    comp_param_1: CompParam1,
    comp_version: CompVersion,
    comp_type: CompType,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Timeout Range Register"]
    #[inline(always)]
    pub const fn torr(&self) -> &Torr {
        &self.torr
    }
    #[doc = "0x08 - Current Counter Value Register"]
    #[inline(always)]
    pub const fn ccvr(&self) -> &Ccvr {
        &self.ccvr
    }
    #[doc = "0x0c - Counter Restart Register"]
    #[inline(always)]
    pub const fn crr(&self) -> &Crr {
        &self.crr
    }
    #[doc = "0x10 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x14 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn eoi(&self) -> &Eoi {
        &self.eoi
    }
    #[doc = "0x1c - Protection level Register"]
    #[inline(always)]
    pub const fn prot_level(&self) -> &ProtLevel {
        &self.prot_level
    }
    #[doc = "0xe4 - Component Parameters Register 5"]
    #[inline(always)]
    pub const fn comp_param_5(&self) -> &CompParam5 {
        &self.comp_param_5
    }
    #[doc = "0xe8 - Component Parameters Register 4"]
    #[inline(always)]
    pub const fn comp_param_4(&self) -> &CompParam4 {
        &self.comp_param_4
    }
    #[doc = "0xec - Component Parameters Register 3"]
    #[inline(always)]
    pub const fn comp_param_3(&self) -> &CompParam3 {
        &self.comp_param_3
    }
    #[doc = "0xf0 - Component Parameters Register 2"]
    #[inline(always)]
    pub const fn comp_param_2(&self) -> &CompParam2 {
        &self.comp_param_2
    }
    #[doc = "0xf4 - Component Parameters Register 1"]
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
#[doc = "cr (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "cr")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "torr (rw) register accessor: Timeout Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`torr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`torr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@torr`]
module"]
#[doc(alias = "torr")]
pub type Torr = crate::Reg<torr::TorrSpec>;
#[doc = "Timeout Range Register"]
pub mod torr;
#[doc = "ccvr (rw) register accessor: Current Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccvr`]
module"]
#[doc(alias = "ccvr")]
pub type Ccvr = crate::Reg<ccvr::CcvrSpec>;
#[doc = "Current Counter Value Register"]
pub mod ccvr;
#[doc = "crr (rw) register accessor: Counter Restart Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crr`]
module"]
#[doc(alias = "crr")]
pub type Crr = crate::Reg<crr::CrrSpec>;
#[doc = "Counter Restart Register"]
pub mod crr;
#[doc = "stat (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "stat")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Interrupt Status Register"]
pub mod stat;
#[doc = "eoi (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eoi`]
module"]
#[doc(alias = "eoi")]
pub type Eoi = crate::Reg<eoi::EoiSpec>;
#[doc = "Interrupt Clear Register"]
pub mod eoi;
#[doc = "prot_level (rw) register accessor: Protection level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`prot_level::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prot_level::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prot_level`]
module"]
#[doc(alias = "prot_level")]
pub type ProtLevel = crate::Reg<prot_level::ProtLevelSpec>;
#[doc = "Protection level Register"]
pub mod prot_level;
#[doc = "comp_param_5 (rw) register accessor: Component Parameters Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_param_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_param_5`]
module"]
#[doc(alias = "comp_param_5")]
pub type CompParam5 = crate::Reg<comp_param_5::CompParam5Spec>;
#[doc = "Component Parameters Register 5"]
pub mod comp_param_5;
#[doc = "comp_param_4 (rw) register accessor: Component Parameters Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_param_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_param_4`]
module"]
#[doc(alias = "comp_param_4")]
pub type CompParam4 = crate::Reg<comp_param_4::CompParam4Spec>;
#[doc = "Component Parameters Register 4"]
pub mod comp_param_4;
#[doc = "comp_param_3 (rw) register accessor: Component Parameters Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_param_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_param_3`]
module"]
#[doc(alias = "comp_param_3")]
pub type CompParam3 = crate::Reg<comp_param_3::CompParam3Spec>;
#[doc = "Component Parameters Register 3"]
pub mod comp_param_3;
#[doc = "comp_param_2 (rw) register accessor: Component Parameters Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_param_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_param_2`]
module"]
#[doc(alias = "comp_param_2")]
pub type CompParam2 = crate::Reg<comp_param_2::CompParam2Spec>;
#[doc = "Component Parameters Register 2"]
pub mod comp_param_2;
#[doc = "comp_param_1 (rw) register accessor: Component Parameters Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_param_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_param_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_param_1`]
module"]
#[doc(alias = "comp_param_1")]
pub type CompParam1 = crate::Reg<comp_param_1::CompParam1Spec>;
#[doc = "Component Parameters Register 1"]
pub mod comp_param_1;
#[doc = "comp_version (rw) register accessor: Component Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_version`]
module"]
#[doc(alias = "comp_version")]
pub type CompVersion = crate::Reg<comp_version::CompVersionSpec>;
#[doc = "Component Version Register"]
pub mod comp_version;
#[doc = "comp_type (rw) register accessor: Component Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_type`]
module"]
#[doc(alias = "comp_type")]
pub type CompType = crate::Reg<comp_type::CompTypeSpec>;
#[doc = "Component Type Register"]
pub mod comp_type;
