#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    input_fifo: InputFifo,
    ctrl: Ctrl,
    fifo_ctrl: FifoCtrl,
    intr_mask: IntrMask,
    intr_clear: IntrClear,
    status: Status,
    status_raw: StatusRaw,
    output_fifo: OutputFifo,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - FFT input data fifo"]
    #[inline(always)]
    pub const fn input_fifo(&self) -> &InputFifo {
        &self.input_fifo
    }
    #[doc = "0x08..0x10 - FFT control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x10..0x18 - FIFO control"]
    #[inline(always)]
    pub const fn fifo_ctrl(&self) -> &FifoCtrl {
        &self.fifo_ctrl
    }
    #[doc = "0x18..0x20 - interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x20..0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn intr_clear(&self) -> &IntrClear {
        &self.intr_clear
    }
    #[doc = "0x28..0x30 - FFT status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x30..0x38 - FFT status raw"]
    #[inline(always)]
    pub const fn status_raw(&self) -> &StatusRaw {
        &self.status_raw
    }
    #[doc = "0x38..0x40 - FFT output FIFO"]
    #[inline(always)]
    pub const fn output_fifo(&self) -> &OutputFifo {
        &self.output_fifo
    }
}
#[doc = "input_fifo (rw) register accessor: FFT input data fifo\n\nYou can [`read`](crate::Reg::read) this register and get [`input_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input_fifo`]
module"]
#[doc(alias = "input_fifo")]
pub type InputFifo = crate::Reg<input_fifo::InputFifoSpec>;
#[doc = "FFT input data fifo"]
pub mod input_fifo;
#[doc = "ctrl (rw) register accessor: FFT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "ctrl")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "FFT control register"]
pub mod ctrl;
#[doc = "fifo_ctrl (rw) register accessor: FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_ctrl`]
module"]
#[doc(alias = "fifo_ctrl")]
pub type FifoCtrl = crate::Reg<fifo_ctrl::FifoCtrlSpec>;
#[doc = "FIFO control"]
pub mod fifo_ctrl;
#[doc = "intr_mask (rw) register accessor: interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "intr_mask")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "interrupt mask"]
pub mod intr_mask;
#[doc = "intr_clear (rw) register accessor: Interrupt clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_clear`]
module"]
#[doc(alias = "intr_clear")]
pub type IntrClear = crate::Reg<intr_clear::IntrClearSpec>;
#[doc = "Interrupt clear"]
pub mod intr_clear;
#[doc = "status (rw) register accessor: FFT status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "FFT status register"]
pub mod status;
#[doc = "status_raw (rw) register accessor: FFT status raw\n\nYou can [`read`](crate::Reg::read) this register and get [`status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_raw`]
module"]
#[doc(alias = "status_raw")]
pub type StatusRaw = crate::Reg<status_raw::StatusRawSpec>;
#[doc = "FFT status raw"]
pub mod status_raw;
#[doc = "output_fifo (rw) register accessor: FFT output FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`output_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output_fifo`]
module"]
#[doc(alias = "output_fifo")]
pub type OutputFifo = crate::Reg<output_fifo::OutputFifoSpec>;
#[doc = "FFT output FIFO"]
pub mod output_fifo;
