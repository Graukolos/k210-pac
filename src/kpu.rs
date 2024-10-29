#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    layer_argument_fifo: LayerArgumentFifo,
    interrupt_status: InterruptStatus,
    interrupt_raw: InterruptRaw,
    interrupt_mask: InterruptMask,
    interrupt_clear: InterruptClear,
    fifo_threshold: FifoThreshold,
    fifo_data_out: FifoDataOut,
    fifo_ctrl: FifoCtrl,
    eight_bit_mode: EightBitMode,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Layer arguments FIFO: each layer is defined by writing 12 successive argument values to this register"]
    #[inline(always)]
    pub const fn layer_argument_fifo(&self) -> &LayerArgumentFifo {
        &self.layer_argument_fifo
    }
    #[doc = "0x08..0x10 - Interrupt status"]
    #[inline(always)]
    pub const fn interrupt_status(&self) -> &InterruptStatus {
        &self.interrupt_status
    }
    #[doc = "0x10..0x18 - Interrupt raw"]
    #[inline(always)]
    pub const fn interrupt_raw(&self) -> &InterruptRaw {
        &self.interrupt_raw
    }
    #[doc = "0x18..0x20 - Interrupt mask: 0 enables the interrupt, 1 masks the interrupt"]
    #[inline(always)]
    pub const fn interrupt_mask(&self) -> &InterruptMask {
        &self.interrupt_mask
    }
    #[doc = "0x20..0x28 - Interrupt clear: write 1 to a bit to clear interrupt"]
    #[inline(always)]
    pub const fn interrupt_clear(&self) -> &InterruptClear {
        &self.interrupt_clear
    }
    #[doc = "0x28..0x30 - FIFO threshold"]
    #[inline(always)]
    pub const fn fifo_threshold(&self) -> &FifoThreshold {
        &self.fifo_threshold
    }
    #[doc = "0x30..0x38 - FIFO data output"]
    #[inline(always)]
    pub const fn fifo_data_out(&self) -> &FifoDataOut {
        &self.fifo_data_out
    }
    #[doc = "0x38..0x40 - FIFO control"]
    #[inline(always)]
    pub const fn fifo_ctrl(&self) -> &FifoCtrl {
        &self.fifo_ctrl
    }
    #[doc = "0x40..0x48 - Eight bit mode"]
    #[inline(always)]
    pub const fn eight_bit_mode(&self) -> &EightBitMode {
        &self.eight_bit_mode
    }
}
#[doc = "layer_argument_fifo (rw) register accessor: Layer arguments FIFO: each layer is defined by writing 12 successive argument values to this register\n\nYou can [`read`](crate::Reg::read) this register and get [`layer_argument_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer_argument_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer_argument_fifo`]
module"]
#[doc(alias = "layer_argument_fifo")]
pub type LayerArgumentFifo = crate::Reg<layer_argument_fifo::LayerArgumentFifoSpec>;
#[doc = "Layer arguments FIFO: each layer is defined by writing 12 successive argument values to this register"]
pub mod layer_argument_fifo;
#[doc = "interrupt_status (rw) register accessor: Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_status`]
module"]
#[doc(alias = "interrupt_status")]
pub type InterruptStatus = crate::Reg<interrupt_status::InterruptStatusSpec>;
#[doc = "Interrupt status"]
pub mod interrupt_status;
pub use interrupt_status as interrupt_raw;
pub use interrupt_status as interrupt_mask;
pub use interrupt_status as interrupt_clear;
pub use InterruptStatus as InterruptRaw;
pub use InterruptStatus as InterruptMask;
pub use InterruptStatus as InterruptClear;
#[doc = "fifo_threshold (rw) register accessor: FIFO threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_threshold`]
module"]
#[doc(alias = "fifo_threshold")]
pub type FifoThreshold = crate::Reg<fifo_threshold::FifoThresholdSpec>;
#[doc = "FIFO threshold"]
pub mod fifo_threshold;
#[doc = "fifo_data_out (rw) register accessor: FIFO data output\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_data_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_data_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_data_out`]
module"]
#[doc(alias = "fifo_data_out")]
pub type FifoDataOut = crate::Reg<fifo_data_out::FifoDataOutSpec>;
#[doc = "FIFO data output"]
pub mod fifo_data_out;
#[doc = "fifo_ctrl (rw) register accessor: FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_ctrl`]
module"]
#[doc(alias = "fifo_ctrl")]
pub type FifoCtrl = crate::Reg<fifo_ctrl::FifoCtrlSpec>;
#[doc = "FIFO control"]
pub mod fifo_ctrl;
#[doc = "eight_bit_mode (rw) register accessor: Eight bit mode\n\nYou can [`read`](crate::Reg::read) this register and get [`eight_bit_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eight_bit_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eight_bit_mode`]
module"]
#[doc(alias = "eight_bit_mode")]
pub type EightBitMode = crate::Reg<eight_bit_mode::EightBitModeSpec>;
#[doc = "Eight bit mode"]
pub mod eight_bit_mode;
