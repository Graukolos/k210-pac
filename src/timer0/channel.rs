#[repr(C)]
#[doc = "Channel cluster: load_count, current_value, control, eoi and intr_stat registers"]
#[doc(alias = "channel")]
pub struct Channel {
    load_count: LoadCount,
    current_value: CurrentValue,
    control: Control,
    eoi: Eoi,
    intr_stat: IntrStat,
}
impl Channel {
    #[doc = "0x00 - Load Count Register"]
    #[inline(always)]
    pub const fn load_count(&self) -> &LoadCount {
        &self.load_count
    }
    #[doc = "0x04 - Current Value Register"]
    #[inline(always)]
    pub const fn current_value(&self) -> &CurrentValue {
        &self.current_value
    }
    #[doc = "0x08 - Control Register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x0c - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn eoi(&self) -> &Eoi {
        &self.eoi
    }
    #[doc = "0x10 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intr_stat(&self) -> &IntrStat {
        &self.intr_stat
    }
}
#[doc = "load_count (rw) register accessor: Load Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`load_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load_count`]
module"]
#[doc(alias = "load_count")]
pub type LoadCount = crate::Reg<load_count::LoadCountSpec>;
#[doc = "Load Count Register"]
pub mod load_count;
#[doc = "current_value (rw) register accessor: Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`current_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`current_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_value`]
module"]
#[doc(alias = "current_value")]
pub type CurrentValue = crate::Reg<current_value::CurrentValueSpec>;
#[doc = "Current Value Register"]
pub mod current_value;
#[doc = "control (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "control")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control Register"]
pub mod control;
#[doc = "eoi (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eoi`]
module"]
#[doc(alias = "eoi")]
pub type Eoi = crate::Reg<eoi::EoiSpec>;
#[doc = "Interrupt Clear Register"]
pub mod eoi;
#[doc = "intr_stat (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_stat`]
module"]
#[doc(alias = "intr_stat")]
pub type IntrStat = crate::Reg<intr_stat::IntrStatSpec>;
#[doc = "Interrupt Status Register"]
pub mod intr_stat;
