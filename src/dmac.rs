#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: Id,
    compver: Compver,
    cfg: Cfg,
    chen: Chen,
    _reserved4: [u8; 0x10],
    intstatus: Intstatus,
    com_intclear: ComIntclear,
    com_intstatus_en: ComIntstatusEn,
    com_intsignal_en: ComIntsignalEn,
    com_intstatus: ComIntstatus,
    reset: Reset,
    _reserved10: [u8; 0xa0],
    channel: [Channel; 6],
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - ID Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x08..0x10 - COMPVER Register"]
    #[inline(always)]
    pub const fn compver(&self) -> &Compver {
        &self.compver
    }
    #[doc = "0x10..0x18 - Configure Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x18..0x20 - Channel Enable Register"]
    #[inline(always)]
    pub const fn chen(&self) -> &Chen {
        &self.chen
    }
    #[doc = "0x30..0x38 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        &self.intstatus
    }
    #[doc = "0x38..0x40 - Common Interrupt Clear Register"]
    #[inline(always)]
    pub const fn com_intclear(&self) -> &ComIntclear {
        &self.com_intclear
    }
    #[doc = "0x40..0x48 - Common Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn com_intstatus_en(&self) -> &ComIntstatusEn {
        &self.com_intstatus_en
    }
    #[doc = "0x48..0x50 - Common Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn com_intsignal_en(&self) -> &ComIntsignalEn {
        &self.com_intsignal_en
    }
    #[doc = "0x50..0x58 - Common Interrupt Status"]
    #[inline(always)]
    pub const fn com_intstatus(&self) -> &ComIntstatus {
        &self.com_intstatus
    }
    #[doc = "0x58..0x60 - Reset register"]
    #[inline(always)]
    pub const fn reset(&self) -> &Reset {
        &self.reset
    }
    #[doc = "0x100..0x700 - Channel configuration"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        &self.channel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x700 - Channel configuration"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        self.channel.iter()
    }
}
#[doc = "id (rw) register accessor: ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "id")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "ID Register"]
pub mod id;
#[doc = "compver (rw) register accessor: COMPVER Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compver::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compver::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compver`]
module"]
#[doc(alias = "compver")]
pub type Compver = crate::Reg<compver::CompverSpec>;
#[doc = "COMPVER Register"]
pub mod compver;
#[doc = "cfg (rw) register accessor: Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "cfg")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configure Register"]
pub mod cfg;
#[doc = "chen (rw) register accessor: Channel Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`]
module"]
#[doc(alias = "chen")]
pub type Chen = crate::Reg<chen::ChenSpec>;
#[doc = "Channel Enable Register"]
pub mod chen;
#[doc = "intstatus (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
#[doc(alias = "intstatus")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "Interrupt Status Register"]
pub mod intstatus;
#[doc = "com_intclear (rw) register accessor: Common Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`com_intclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`com_intclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@com_intclear`]
module"]
#[doc(alias = "com_intclear")]
pub type ComIntclear = crate::Reg<com_intclear::ComIntclearSpec>;
#[doc = "Common Interrupt Clear Register"]
pub mod com_intclear;
#[doc = "com_intstatus_en (rw) register accessor: Common Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`com_intstatus_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`com_intstatus_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@com_intstatus_en`]
module"]
#[doc(alias = "com_intstatus_en")]
pub type ComIntstatusEn = crate::Reg<com_intstatus_en::ComIntstatusEnSpec>;
#[doc = "Common Interrupt Status Enable Register"]
pub mod com_intstatus_en;
#[doc = "com_intsignal_en (rw) register accessor: Common Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`com_intsignal_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`com_intsignal_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@com_intsignal_en`]
module"]
#[doc(alias = "com_intsignal_en")]
pub type ComIntsignalEn = crate::Reg<com_intsignal_en::ComIntsignalEnSpec>;
#[doc = "Common Interrupt Signal Enable Register"]
pub mod com_intsignal_en;
#[doc = "com_intstatus (rw) register accessor: Common Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`com_intstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`com_intstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@com_intstatus`]
module"]
#[doc(alias = "com_intstatus")]
pub type ComIntstatus = crate::Reg<com_intstatus::ComIntstatusSpec>;
#[doc = "Common Interrupt Status"]
pub mod com_intstatus;
#[doc = "reset (rw) register accessor: Reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
#[doc(alias = "reset")]
pub type Reset = crate::Reg<reset::ResetSpec>;
#[doc = "Reset register"]
pub mod reset;
#[doc = "Channel configuration"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "Channel configuration"]
pub mod channel;
