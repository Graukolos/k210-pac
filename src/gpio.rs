#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    data_output: DataOutput,
    direction: Direction,
    source: Source,
    _reserved3: [u8; 0x24],
    interrupt_enable: InterruptEnable,
    interrupt_mask: InterruptMask,
    interrupt_level: InterruptLevel,
    interrupt_polarity: InterruptPolarity,
    interrupt_status: InterruptStatus,
    interrupt_status_raw: InterruptStatusRaw,
    interrupt_debounce: InterruptDebounce,
    interrupt_clear: InterruptClear,
    data_input: DataInput,
    _reserved12: [u8; 0x0c],
    sync_level: SyncLevel,
    id_code: IdCode,
    interrupt_bothedge: InterruptBothedge,
}
impl RegisterBlock {
    #[doc = "0x00 - Data (output) registers"]
    #[inline(always)]
    pub const fn data_output(&self) -> &DataOutput {
        &self.data_output
    }
    #[doc = "0x04 - Data direction registers"]
    #[inline(always)]
    pub const fn direction(&self) -> &Direction {
        &self.direction
    }
    #[doc = "0x08 - Data source registers"]
    #[inline(always)]
    pub const fn source(&self) -> &Source {
        &self.source
    }
    #[doc = "0x30 - Interrupt enable/disable registers"]
    #[inline(always)]
    pub const fn interrupt_enable(&self) -> &InterruptEnable {
        &self.interrupt_enable
    }
    #[doc = "0x34 - Interrupt mask registers"]
    #[inline(always)]
    pub const fn interrupt_mask(&self) -> &InterruptMask {
        &self.interrupt_mask
    }
    #[doc = "0x38 - Interrupt level registers"]
    #[inline(always)]
    pub const fn interrupt_level(&self) -> &InterruptLevel {
        &self.interrupt_level
    }
    #[doc = "0x3c - Interrupt polarity registers"]
    #[inline(always)]
    pub const fn interrupt_polarity(&self) -> &InterruptPolarity {
        &self.interrupt_polarity
    }
    #[doc = "0x40 - Interrupt status registers"]
    #[inline(always)]
    pub const fn interrupt_status(&self) -> &InterruptStatus {
        &self.interrupt_status
    }
    #[doc = "0x44 - Raw interrupt status registers"]
    #[inline(always)]
    pub const fn interrupt_status_raw(&self) -> &InterruptStatusRaw {
        &self.interrupt_status_raw
    }
    #[doc = "0x48 - Interrupt debounce registers"]
    #[inline(always)]
    pub const fn interrupt_debounce(&self) -> &InterruptDebounce {
        &self.interrupt_debounce
    }
    #[doc = "0x4c - Registers for clearing interrupts"]
    #[inline(always)]
    pub const fn interrupt_clear(&self) -> &InterruptClear {
        &self.interrupt_clear
    }
    #[doc = "0x50 - External port (data input) registers"]
    #[inline(always)]
    pub const fn data_input(&self) -> &DataInput {
        &self.data_input
    }
    #[doc = "0x60 - Sync level registers"]
    #[inline(always)]
    pub const fn sync_level(&self) -> &SyncLevel {
        &self.sync_level
    }
    #[doc = "0x64 - ID code"]
    #[inline(always)]
    pub const fn id_code(&self) -> &IdCode {
        &self.id_code
    }
    #[doc = "0x68 - Interrupt both edge type"]
    #[inline(always)]
    pub const fn interrupt_bothedge(&self) -> &InterruptBothedge {
        &self.interrupt_bothedge
    }
}
#[doc = "data_output (rw) register accessor: Data (output) registers\n\nYou can [`read`](crate::Reg::read) this register and get [`data_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_output`]
module"]
#[doc(alias = "data_output")]
pub type DataOutput = crate::Reg<data_output::DataOutputSpec>;
#[doc = "Data (output) registers"]
pub mod data_output;
#[doc = "direction (rw) register accessor: Data direction registers\n\nYou can [`read`](crate::Reg::read) this register and get [`direction::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direction::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@direction`]
module"]
#[doc(alias = "direction")]
pub type Direction = crate::Reg<direction::DirectionSpec>;
#[doc = "Data direction registers"]
pub mod direction;
#[doc = "source (rw) register accessor: Data source registers\n\nYou can [`read`](crate::Reg::read) this register and get [`source::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`source::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source`]
module"]
#[doc(alias = "source")]
pub type Source = crate::Reg<source::SourceSpec>;
#[doc = "Data source registers"]
pub mod source;
#[doc = "interrupt_enable (rw) register accessor: Interrupt enable/disable registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_enable`]
module"]
#[doc(alias = "interrupt_enable")]
pub type InterruptEnable = crate::Reg<interrupt_enable::InterruptEnableSpec>;
#[doc = "Interrupt enable/disable registers"]
pub mod interrupt_enable;
#[doc = "interrupt_mask (rw) register accessor: Interrupt mask registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_mask`]
module"]
#[doc(alias = "interrupt_mask")]
pub type InterruptMask = crate::Reg<interrupt_mask::InterruptMaskSpec>;
#[doc = "Interrupt mask registers"]
pub mod interrupt_mask;
#[doc = "interrupt_level (rw) register accessor: Interrupt level registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_level::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_level::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_level`]
module"]
#[doc(alias = "interrupt_level")]
pub type InterruptLevel = crate::Reg<interrupt_level::InterruptLevelSpec>;
#[doc = "Interrupt level registers"]
pub mod interrupt_level;
#[doc = "interrupt_polarity (rw) register accessor: Interrupt polarity registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_polarity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_polarity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_polarity`]
module"]
#[doc(alias = "interrupt_polarity")]
pub type InterruptPolarity = crate::Reg<interrupt_polarity::InterruptPolaritySpec>;
#[doc = "Interrupt polarity registers"]
pub mod interrupt_polarity;
#[doc = "interrupt_status (rw) register accessor: Interrupt status registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_status`]
module"]
#[doc(alias = "interrupt_status")]
pub type InterruptStatus = crate::Reg<interrupt_status::InterruptStatusSpec>;
#[doc = "Interrupt status registers"]
pub mod interrupt_status;
#[doc = "interrupt_status_raw (rw) register accessor: Raw interrupt status registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_status_raw`]
module"]
#[doc(alias = "interrupt_status_raw")]
pub type InterruptStatusRaw = crate::Reg<interrupt_status_raw::InterruptStatusRawSpec>;
#[doc = "Raw interrupt status registers"]
pub mod interrupt_status_raw;
#[doc = "interrupt_debounce (rw) register accessor: Interrupt debounce registers\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_debounce::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_debounce::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_debounce`]
module"]
#[doc(alias = "interrupt_debounce")]
pub type InterruptDebounce = crate::Reg<interrupt_debounce::InterruptDebounceSpec>;
#[doc = "Interrupt debounce registers"]
pub mod interrupt_debounce;
#[doc = "interrupt_clear (rw) register accessor: Registers for clearing interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_clear`]
module"]
#[doc(alias = "interrupt_clear")]
pub type InterruptClear = crate::Reg<interrupt_clear::InterruptClearSpec>;
#[doc = "Registers for clearing interrupts"]
pub mod interrupt_clear;
#[doc = "data_input (rw) register accessor: External port (data input) registers\n\nYou can [`read`](crate::Reg::read) this register and get [`data_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_input`]
module"]
#[doc(alias = "data_input")]
pub type DataInput = crate::Reg<data_input::DataInputSpec>;
#[doc = "External port (data input) registers"]
pub mod data_input;
#[doc = "sync_level (rw) register accessor: Sync level registers\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_level::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_level::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_level`]
module"]
#[doc(alias = "sync_level")]
pub type SyncLevel = crate::Reg<sync_level::SyncLevelSpec>;
#[doc = "Sync level registers"]
pub mod sync_level;
#[doc = "id_code (rw) register accessor: ID code\n\nYou can [`read`](crate::Reg::read) this register and get [`id_code::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_code::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_code`]
module"]
#[doc(alias = "id_code")]
pub type IdCode = crate::Reg<id_code::IdCodeSpec>;
#[doc = "ID code"]
pub mod id_code;
#[doc = "interrupt_bothedge (rw) register accessor: Interrupt both edge type\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_bothedge::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_bothedge::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_bothedge`]
module"]
#[doc(alias = "interrupt_bothedge")]
pub type InterruptBothedge = crate::Reg<interrupt_bothedge::InterruptBothedgeSpec>;
#[doc = "Interrupt both edge type"]
pub mod interrupt_bothedge;
