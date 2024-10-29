#[doc = "Register `fcr_iir` reader"]
pub type R = crate::R<FcrIirSpec>;
#[doc = "Register `fcr_iir` writer"]
pub type W = crate::W<FcrIirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Control Register / Interrupt Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcr_iir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr_iir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrIirSpec;
impl crate::RegisterSpec for FcrIirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr_iir::R`](R) reader structure"]
impl crate::Readable for FcrIirSpec {}
#[doc = "`write(|w| ..)` method takes [`fcr_iir::W`](W) writer structure"]
impl crate::Writable for FcrIirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fcr_iir to value 0"]
impl crate::Resettable for FcrIirSpec {
    const RESET_VALUE: u32 = 0;
}
