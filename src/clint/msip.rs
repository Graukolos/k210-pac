#[doc = "Register `msip[%s]` reader"]
pub type R = crate::R<MsipSpec>;
#[doc = "Register `msip[%s]` writer"]
pub type W = crate::W<MsipSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Hart software interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`msip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsipSpec;
impl crate::RegisterSpec for MsipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msip::R`](R) reader structure"]
impl crate::Readable for MsipSpec {}
#[doc = "`write(|w| ..)` method takes [`msip::W`](W) writer structure"]
impl crate::Writable for MsipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets msip[%s]
to value 0"]
impl crate::Resettable for MsipSpec {
    const RESET_VALUE: u32 = 0;
}
