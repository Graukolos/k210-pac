#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    priority: [Priority; 1024],
    pending: [Pending; 32],
    _reserved2: [u8; 0x0f80],
    target_enables: [TargetEnables; 4],
    _reserved3: [u8; 0x001f_de00],
    targets: [Targets; 4],
}
impl RegisterBlock {
    #[doc = "0x00..0x1000 - Interrupt Source Priority Register"]
    #[inline(always)]
    pub const fn priority(&self, n: usize) -> &Priority {
        &self.priority[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1000 - Interrupt Source Priority Register"]
    #[inline(always)]
    pub fn priority_iter(&self) -> impl Iterator<Item = &Priority> {
        self.priority.iter()
    }
    #[doc = "0x1000..0x1080 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn pending(&self, n: usize) -> &Pending {
        &self.pending[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1080 - Interrupt Pending Register"]
    #[inline(always)]
    pub fn pending_iter(&self) -> impl Iterator<Item = &Pending> {
        self.pending.iter()
    }
    #[doc = "0x2000..0x2200 - Target Interrupt Enables"]
    #[inline(always)]
    pub const fn target_enables(&self, n: usize) -> &TargetEnables {
        &self.target_enables[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2200 - Target Interrupt Enables"]
    #[inline(always)]
    pub fn target_enables_iter(&self) -> impl Iterator<Item = &TargetEnables> {
        self.target_enables.iter()
    }
    #[doc = "0x200000..0x204000 - Target Configuration"]
    #[inline(always)]
    pub const fn targets(&self, n: usize) -> &Targets {
        &self.targets[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200000..0x204000 - Target Configuration"]
    #[inline(always)]
    pub fn targets_iter(&self) -> impl Iterator<Item = &Targets> {
        self.targets.iter()
    }
}
#[doc = "priority (rw) register accessor: Interrupt Source Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`priority::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority`]
module"]
#[doc(alias = "priority")]
pub type Priority = crate::Reg<priority::PrioritySpec>;
#[doc = "Interrupt Source Priority Register"]
pub mod priority;
#[doc = "pending (rw) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pending::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pending::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending`]
module"]
#[doc(alias = "pending")]
pub type Pending = crate::Reg<pending::PendingSpec>;
#[doc = "Interrupt Pending Register"]
pub mod pending;
#[doc = "Target Interrupt Enables"]
pub use self::target_enables::TargetEnables;
#[doc = r"Cluster"]
#[doc = "Target Interrupt Enables"]
pub mod target_enables;
#[doc = "Target Configuration"]
pub use self::targets::Targets;
#[doc = r"Cluster"]
#[doc = "Target Configuration"]
pub mod targets;
