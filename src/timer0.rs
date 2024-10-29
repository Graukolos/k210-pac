#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    channel: [Channel; 4],
    _reserved1: [u8; 0x50],
    intr_stat: IntrStat,
    eoi: Eoi,
    raw_intr_stat: RawIntrStat,
    comp_version: CompVersion,
    load_count2: [LoadCount2; 4],
}
impl RegisterBlock {
    #[doc = "0x00..0x50 - Channel cluster: load_count, current_value, control, eoi and intr_stat registers"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        &self.channel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x50 - Channel cluster: load_count, current_value, control, eoi and intr_stat registers"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        self.channel.iter()
    }
    #[doc = "0xa0 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intr_stat(&self) -> &IntrStat {
        &self.intr_stat
    }
    #[doc = "0xa4 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn eoi(&self) -> &Eoi {
        &self.eoi
    }
    #[doc = "0xa8 - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn raw_intr_stat(&self) -> &RawIntrStat {
        &self.raw_intr_stat
    }
    #[doc = "0xac - Component Version Register"]
    #[inline(always)]
    pub const fn comp_version(&self) -> &CompVersion {
        &self.comp_version
    }
    #[doc = "0xb0..0xc0 - Load Count2 Register"]
    #[inline(always)]
    pub const fn load_count2(&self, n: usize) -> &LoadCount2 {
        &self.load_count2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb0..0xc0 - Load Count2 Register"]
    #[inline(always)]
    pub fn load_count2_iter(&self) -> impl Iterator<Item = &LoadCount2> {
        self.load_count2.iter()
    }
}
#[doc = "Channel cluster: load_count, current_value, control, eoi and intr_stat registers"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "Channel cluster: load_count, current_value, control, eoi and intr_stat registers"]
pub mod channel;
#[doc = "intr_stat (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_stat`]
module"]
#[doc(alias = "intr_stat")]
pub type IntrStat = crate::Reg<intr_stat::IntrStatSpec>;
#[doc = "Interrupt Status Register"]
pub mod intr_stat;
#[doc = "eoi (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eoi`]
module"]
#[doc(alias = "eoi")]
pub type Eoi = crate::Reg<eoi::EoiSpec>;
#[doc = "Interrupt Clear Register"]
pub mod eoi;
#[doc = "raw_intr_stat (rw) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_intr_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_intr_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_intr_stat`]
module"]
#[doc(alias = "raw_intr_stat")]
pub type RawIntrStat = crate::Reg<raw_intr_stat::RawIntrStatSpec>;
#[doc = "Raw Interrupt Status Register"]
pub mod raw_intr_stat;
#[doc = "comp_version (rw) register accessor: Component Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_version`]
module"]
#[doc(alias = "comp_version")]
pub type CompVersion = crate::Reg<comp_version::CompVersionSpec>;
#[doc = "Component Version Register"]
pub mod comp_version;
#[doc = "load_count2 (rw) register accessor: Load Count2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`load_count2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_count2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load_count2`]
module"]
#[doc(alias = "load_count2")]
pub type LoadCount2 = crate::Reg<load_count2::LoadCount2Spec>;
#[doc = "Load Count2 Register"]
pub mod load_count2;
