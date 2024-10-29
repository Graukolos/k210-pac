#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    io: [Io; 48],
    tie_en: [TieEn; 8],
    tie_val: [TieVal; 8],
}
impl RegisterBlock {
    #[doc = "0x00..0xc0 - FPIOA GPIO multiplexer io array"]
    #[inline(always)]
    pub const fn io(&self, n: usize) -> &Io {
        &self.io[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0xc0 - FPIOA GPIO multiplexer io array"]
    #[inline(always)]
    pub fn io_iter(&self) -> impl Iterator<Item = &Io> {
        self.io.iter()
    }
    #[doc = "0xc0..0xe0 - FPIOA GPIO multiplexer tie enable array"]
    #[inline(always)]
    pub const fn tie_en(&self, n: usize) -> &TieEn {
        &self.tie_en[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xe0 - FPIOA GPIO multiplexer tie enable array"]
    #[inline(always)]
    pub fn tie_en_iter(&self) -> impl Iterator<Item = &TieEn> {
        self.tie_en.iter()
    }
    #[doc = "0xe0..0x100 - FPIOA GPIO multiplexer tie value array"]
    #[inline(always)]
    pub const fn tie_val(&self, n: usize) -> &TieVal {
        &self.tie_val[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe0..0x100 - FPIOA GPIO multiplexer tie value array"]
    #[inline(always)]
    pub fn tie_val_iter(&self) -> impl Iterator<Item = &TieVal> {
        self.tie_val.iter()
    }
}
#[doc = "io (rw) register accessor: FPIOA GPIO multiplexer io array\n\nYou can [`read`](crate::Reg::read) this register and get [`io::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io`]
module"]
#[doc(alias = "io")]
pub type Io = crate::Reg<io::IoSpec>;
#[doc = "FPIOA GPIO multiplexer io array"]
pub mod io;
#[doc = "tie_en (rw) register accessor: FPIOA GPIO multiplexer tie enable array\n\nYou can [`read`](crate::Reg::read) this register and get [`tie_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tie_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tie_en`]
module"]
#[doc(alias = "tie_en")]
pub type TieEn = crate::Reg<tie_en::TieEnSpec>;
#[doc = "FPIOA GPIO multiplexer tie enable array"]
pub mod tie_en;
#[doc = "tie_val (rw) register accessor: FPIOA GPIO multiplexer tie value array\n\nYou can [`read`](crate::Reg::read) this register and get [`tie_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tie_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tie_val`]
module"]
#[doc(alias = "tie_val")]
pub type TieVal = crate::Reg<tie_val::TieValSpec>;
#[doc = "FPIOA GPIO multiplexer tie value array"]
pub mod tie_val;
