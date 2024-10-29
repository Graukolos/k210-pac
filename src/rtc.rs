#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    date: Date,
    time: Time,
    alarm_date: AlarmDate,
    alarm_time: AlarmTime,
    initial_count: InitialCount,
    current_count: CurrentCount,
    interrupt_ctrl: InterruptCtrl,
    register_ctrl: RegisterCtrl,
    _reserved8: [u8; 0x08],
    extended: Extended,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer date information"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x04 - Timer time information"]
    #[inline(always)]
    pub const fn time(&self) -> &Time {
        &self.time
    }
    #[doc = "0x08 - Alarm date information"]
    #[inline(always)]
    pub const fn alarm_date(&self) -> &AlarmDate {
        &self.alarm_date
    }
    #[doc = "0x0c - Alarm time information"]
    #[inline(always)]
    pub const fn alarm_time(&self) -> &AlarmTime {
        &self.alarm_time
    }
    #[doc = "0x10 - Timer counter initial value"]
    #[inline(always)]
    pub const fn initial_count(&self) -> &InitialCount {
        &self.initial_count
    }
    #[doc = "0x14 - Timer counter current value"]
    #[inline(always)]
    pub const fn current_count(&self) -> &CurrentCount {
        &self.current_count
    }
    #[doc = "0x18 - RTC interrupt settings"]
    #[inline(always)]
    pub const fn interrupt_ctrl(&self) -> &InterruptCtrl {
        &self.interrupt_ctrl
    }
    #[doc = "0x1c - RTC register settings"]
    #[inline(always)]
    pub const fn register_ctrl(&self) -> &RegisterCtrl {
        &self.register_ctrl
    }
    #[doc = "0x28 - Timer extended information"]
    #[inline(always)]
    pub const fn extended(&self) -> &Extended {
        &self.extended
    }
}
#[doc = "date (rw) register accessor: Timer date information\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`]
module"]
#[doc(alias = "date")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Timer date information"]
pub mod date;
#[doc = "time (rw) register accessor: Timer time information\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`]
module"]
#[doc(alias = "time")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "Timer time information"]
pub mod time;
#[doc = "alarm_date (rw) register accessor: Alarm date information\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm_date`]
module"]
#[doc(alias = "alarm_date")]
pub type AlarmDate = crate::Reg<alarm_date::AlarmDateSpec>;
#[doc = "Alarm date information"]
pub mod alarm_date;
#[doc = "alarm_time (rw) register accessor: Alarm time information\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm_time`]
module"]
#[doc(alias = "alarm_time")]
pub type AlarmTime = crate::Reg<alarm_time::AlarmTimeSpec>;
#[doc = "Alarm time information"]
pub mod alarm_time;
#[doc = "initial_count (rw) register accessor: Timer counter initial value\n\nYou can [`read`](crate::Reg::read) this register and get [`initial_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initial_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initial_count`]
module"]
#[doc(alias = "initial_count")]
pub type InitialCount = crate::Reg<initial_count::InitialCountSpec>;
#[doc = "Timer counter initial value"]
pub mod initial_count;
#[doc = "current_count (rw) register accessor: Timer counter current value\n\nYou can [`read`](crate::Reg::read) this register and get [`current_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`current_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_count`]
module"]
#[doc(alias = "current_count")]
pub type CurrentCount = crate::Reg<current_count::CurrentCountSpec>;
#[doc = "Timer counter current value"]
pub mod current_count;
#[doc = "interrupt_ctrl (rw) register accessor: RTC interrupt settings\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_ctrl`]
module"]
#[doc(alias = "interrupt_ctrl")]
pub type InterruptCtrl = crate::Reg<interrupt_ctrl::InterruptCtrlSpec>;
#[doc = "RTC interrupt settings"]
pub mod interrupt_ctrl;
#[doc = "register_ctrl (rw) register accessor: RTC register settings\n\nYou can [`read`](crate::Reg::read) this register and get [`register_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register_ctrl`]
module"]
#[doc(alias = "register_ctrl")]
pub type RegisterCtrl = crate::Reg<register_ctrl::RegisterCtrlSpec>;
#[doc = "RTC register settings"]
pub mod register_ctrl;
#[doc = "extended (rw) register accessor: Timer extended information\n\nYou can [`read`](crate::Reg::read) this register and get [`extended::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extended::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extended`]
module"]
#[doc(alias = "extended")]
pub type Extended = crate::Reg<extended::ExtendedSpec>;
#[doc = "Timer extended information"]
pub mod extended;
