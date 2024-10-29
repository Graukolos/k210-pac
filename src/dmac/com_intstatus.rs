#[doc = "Register `com_intstatus` reader"]
pub type R = crate::R<ComIntstatusSpec>;
#[doc = "Register `com_intstatus` writer"]
pub type W = crate::W<ComIntstatusSpec>;
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
    pub fn slvif_dec_err(&mut self) -> SlvifDecErrW<ComIntstatusSpec> {
        SlvifDecErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave Interface Common Register Write to Read only Error"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wr2ro_err(&mut self) -> SlvifWr2roErrW<ComIntstatusSpec> {
        SlvifWr2roErrW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Interface Common Register Read to Write-only Error"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_rd2wo_err(&mut self) -> SlvifRd2woErrW<ComIntstatusSpec> {
        SlvifRd2woErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Interface Common Register Write On Hold Error"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wronhold_err(&mut self) -> SlvifWronholdErrW<ComIntstatusSpec> {
        SlvifWronholdErrW::new(self, 3)
    }
    #[doc = "Bit 8 - Slave Interface Undefined Register Decode Error"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_undefinedreg_dec_err(&mut self) -> SlvifUndefinedregDecErrW<ComIntstatusSpec> {
        SlvifUndefinedregDecErrW::new(self, 8)
    }
}
#[doc = "Common Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`com_intstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`com_intstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ComIntstatusSpec;
impl crate::RegisterSpec for ComIntstatusSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`com_intstatus::R`](R) reader structure"]
impl crate::Readable for ComIntstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`com_intstatus::W`](W) writer structure"]
impl crate::Writable for ComIntstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets com_intstatus to value 0"]
impl crate::Resettable for ComIntstatusSpec {
    const RESET_VALUE: u64 = 0;
}
