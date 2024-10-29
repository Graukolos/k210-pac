#[doc = "Register `far` reader"]
pub type R = crate::R<FarSpec>;
#[doc = "Register `far` writer"]
pub type W = crate::W<FarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`far::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`far::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FarSpec;
impl crate::RegisterSpec for FarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`far::R`](R) reader structure"]
impl crate::Readable for FarSpec {}
#[doc = "`write(|w| ..)` method takes [`far::W`](W) writer structure"]
impl crate::Writable for FarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets far to value 0"]
impl crate::Resettable for FarSpec {
    const RESET_VALUE: u32 = 0;
}
