#[doc = "Register `com_intclear` reader"]
pub type R = crate::R<ComIntclearSpec>;
#[doc = "Register `com_intclear` writer"]
pub type W = crate::W<ComIntclearSpec>;
#[doc = "Field `slvif_dec_err` reader - Clear slvif_dec_err interrupt in com_intstatus"]
pub type SlvifDecErrR = crate::BitReader;
#[doc = "Field `slvif_dec_err` writer - Clear slvif_dec_err interrupt in com_intstatus"]
pub type SlvifDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_wr2ro_err` reader - Clear slvif_wr2ro_err interrupt in com_intstatus"]
pub type SlvifWr2roErrR = crate::BitReader;
#[doc = "Field `slvif_wr2ro_err` writer - Clear slvif_wr2ro_err interrupt in com_intstatus"]
pub type SlvifWr2roErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_rd2wo_err` reader - Clear slvif_rd2wo_err interrupt in com_intstatus"]
pub type SlvifRd2woErrR = crate::BitReader;
#[doc = "Field `slvif_rd2wo_err` writer - Clear slvif_rd2wo_err interrupt in com_intstatus"]
pub type SlvifRd2woErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_wronhold_err` reader - Clear slvif_wronhold_err interrupt in com_intstatus"]
pub type SlvifWronholdErrR = crate::BitReader;
#[doc = "Field `slvif_wronhold_err` writer - Clear slvif_wronhold_err interrupt in com_intstatus"]
pub type SlvifWronholdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_undefinedreg_dec_err` reader - Clear slvif_undefinedreg_dec_err in com_intstatus"]
pub type SlvifUndefinedregDecErrR = crate::BitReader;
#[doc = "Field `slvif_undefinedreg_dec_err` writer - Clear slvif_undefinedreg_dec_err in com_intstatus"]
pub type SlvifUndefinedregDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear slvif_dec_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_dec_err(&self) -> SlvifDecErrR {
        SlvifDecErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear slvif_wr2ro_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_wr2ro_err(&self) -> SlvifWr2roErrR {
        SlvifWr2roErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear slvif_rd2wo_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_rd2wo_err(&self) -> SlvifRd2woErrR {
        SlvifRd2woErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear slvif_wronhold_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_wronhold_err(&self) -> SlvifWronholdErrR {
        SlvifWronholdErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear slvif_undefinedreg_dec_err in com_intstatus"]
    #[inline(always)]
    pub fn slvif_undefinedreg_dec_err(&self) -> SlvifUndefinedregDecErrR {
        SlvifUndefinedregDecErrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear slvif_dec_err interrupt in com_intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_dec_err(&mut self) -> SlvifDecErrW<ComIntclearSpec> {
        SlvifDecErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear slvif_wr2ro_err interrupt in com_intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wr2ro_err(&mut self) -> SlvifWr2roErrW<ComIntclearSpec> {
        SlvifWr2roErrW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear slvif_rd2wo_err interrupt in com_intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_rd2wo_err(&mut self) -> SlvifRd2woErrW<ComIntclearSpec> {
        SlvifRd2woErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear slvif_wronhold_err interrupt in com_intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wronhold_err(&mut self) -> SlvifWronholdErrW<ComIntclearSpec> {
        SlvifWronholdErrW::new(self, 3)
    }
    #[doc = "Bit 8 - Clear slvif_undefinedreg_dec_err in com_intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_undefinedreg_dec_err(&mut self) -> SlvifUndefinedregDecErrW<ComIntclearSpec> {
        SlvifUndefinedregDecErrW::new(self, 8)
    }
}
#[doc = "Common Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`com_intclear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`com_intclear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ComIntclearSpec;
impl crate::RegisterSpec for ComIntclearSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`com_intclear::R`](R) reader structure"]
impl crate::Readable for ComIntclearSpec {}
#[doc = "`write(|w| ..)` method takes [`com_intclear::W`](W) writer structure"]
impl crate::Writable for ComIntclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets com_intclear to value 0"]
impl crate::Resettable for ComIntclearSpec {
    const RESET_VALUE: u64 = 0;
}
