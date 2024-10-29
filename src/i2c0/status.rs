#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `activity` reader - ACTIVITY"]
pub type ActivityR = crate::BitReader;
#[doc = "Field `tfnf` reader - TFNF"]
pub type TfnfR = crate::BitReader;
#[doc = "Field `tfe` reader - TFE"]
pub type TfeR = crate::BitReader;
#[doc = "Field `rfne` reader - RFNE"]
pub type RfneR = crate::BitReader;
#[doc = "Field `rff` reader - RFF"]
pub type RffR = crate::BitReader;
#[doc = "Field `mst_activity` reader - MST_ACTIVITY"]
pub type MstActivityR = crate::BitReader;
#[doc = "Field `slv_activity` reader - SLV_ACTIVITY"]
pub type SlvActivityR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ACTIVITY"]
    #[inline(always)]
    pub fn activity(&self) -> ActivityR {
        ActivityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TFNF"]
    #[inline(always)]
    pub fn tfnf(&self) -> TfnfR {
        TfnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TFE"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RFNE"]
    #[inline(always)]
    pub fn rfne(&self) -> RfneR {
        RfneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RFF"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MST_ACTIVITY"]
    #[inline(always)]
    pub fn mst_activity(&self) -> MstActivityR {
        MstActivityR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SLV_ACTIVITY"]
    #[inline(always)]
    pub fn slv_activity(&self) -> SlvActivityR {
        SlvActivityR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
