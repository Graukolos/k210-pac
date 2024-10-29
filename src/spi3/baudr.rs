#[doc = "Register `baudr` reader"]
pub type R = crate::R<BaudrSpec>;
#[doc = "Register `baudr` writer"]
pub type W = crate::W<BaudrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Baud Rate Select\n\nYou can [`read`](crate::Reg::read) this register and get [`baudr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudrSpec;
impl crate::RegisterSpec for BaudrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baudr::R`](R) reader structure"]
impl crate::Readable for BaudrSpec {}
#[doc = "`write(|w| ..)` method takes [`baudr::W`](W) writer structure"]
impl crate::Writable for BaudrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets baudr to value 0"]
impl crate::Resettable for BaudrSpec {
    const RESET_VALUE: u32 = 0;
}
