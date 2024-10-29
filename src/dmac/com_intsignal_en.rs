#[doc = "Register `com_intsignal_en` reader"]
pub type R = crate::R<ComIntsignalEnSpec>;
#[doc = "Register `com_intsignal_en` writer"]
pub type W = crate::W<ComIntsignalEnSpec>;
#[doc = "Field `slvif_dec_err` reader - Slave Interface Common Register Decode Error"]
pub type SlvifDecErrR = crate::BitReader;
#[doc = "Field `slvif_dec_err` writer - Slave Interface Common Register Decode Error"]
pub type SlvifDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_wr2ro_err` reader - Slave Interface Common Register Write to Read only Error"]
pub type SlvifWr2roErrR = crate::BitReader;
#[doc = "Field `slvif_wr2ro_err` writer - Slave Interface Common Register Write to Read only Error"]
pub type SlvifWr2roErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_rd2wo_err` reader - Slave Interface Common Register Read to Write-only Error"]
pub type SlvifRd2woErrR = crate::BitReader;
#[doc = "Field `slvif_rd2wo_err` writer - Slave Interface Common Register Read to Write-only Error"]
pub type SlvifRd2woErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_wronhold_err` reader - Slave Interface Common Register Write On Hold Error"]
pub type SlvifWronholdErrR = crate::BitReader;
#[doc = "Field `slvif_wronhold_err` writer - Slave Interface Common Register Write On Hold Error"]
pub type SlvifWronholdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_undefinedreg_dec_err` reader - Slave Interface Undefined Register Decode Error"]
pub type SlvifUndefinedregDecErrR = crate::BitReader;
#[doc = "Field `slvif_undefinedreg_dec_err` writer - Slave Interface Undefined Register Decode Error"]
pub type SlvifUndefinedregDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Slave Interface Common Register Decode Error"]
    #[inline(always)]
    pub fn slvif_dec_err(&self) -> SlvifDecErrR {
        SlvifDecErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Interface Common Register Write to Read only Error"]
    #[inline(always)]
    pub fn slvif_wr2ro_err(&self) -> SlvifWr2roErrR {
        SlvifWr2roErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Interface Common Register Read to Write-only Error"]
    #[inline(always)]
    pub fn slvif_rd2wo_err(&self) -> SlvifRd2woErrR {
        SlvifRd2woErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Interface Common Register Write On Hold Error"]
    #[inline(always)]
    pub fn slvif_wronhold_err(&self) -> SlvifWronholdErrR {
        SlvifWronholdErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave Interface Undefined Register Decode Error"]
    #[inline(always)]
    pub fn slvif_undefinedreg_dec_err(&self) -> SlvifUndefinedregDecErrR {
        SlvifUndefinedregDecErrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Interface Common Register Decode Error"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_dec_err(&mut self) -> SlvifDecErrW<ComIntsignalEnSpec> {
        SlvifDecErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave Interface Common Register Write to Read only Error"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wr2ro_err(&mut self) -> SlvifWr2roErrW<ComIntsignalEnSpec> {
        SlvifWr2roErrW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Interface Common Register Read to Write-only Error"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_rd2wo_err(&mut self) -> SlvifRd2woErrW<ComIntsignalEnSpec> {
        SlvifRd2woErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Interface Common Register Write On Hold Error"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wronhold_err(&mut self) -> SlvifWronholdErrW<ComIntsignalEnSpec> {
        SlvifWronholdErrW::new(self, 3)
    }
    #[doc = "Bit 8 - Slave Interface Undefined Register Decode Error"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_undefinedreg_dec_err(&mut self) -> SlvifUndefinedregDecErrW<ComIntsignalEnSpec> {
        SlvifUndefinedregDecErrW::new(self, 8)
    }
}
#[doc = "Common Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`com_intsignal_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`com_intsignal_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ComIntsignalEnSpec;
impl crate::RegisterSpec for ComIntsignalEnSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`com_intsignal_en::R`](R) reader structure"]
impl crate::Readable for ComIntsignalEnSpec {}
#[doc = "`write(|w| ..)` method takes [`com_intsignal_en::W`](W) writer structure"]
impl crate::Writable for ComIntsignalEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets com_intsignal_en to value 0"]
impl crate::Resettable for ComIntsignalEnSpec {
    const RESET_VALUE: u64 = 0;
}
