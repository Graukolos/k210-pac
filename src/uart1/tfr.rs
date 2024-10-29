#[doc = "Register `tfr` reader"]
pub type R = crate::R<TfrSpec>;
#[doc = "Register `tfr` writer"]
pub type W = crate::W<TfrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Transmit FIFO Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfrSpec;
impl crate::RegisterSpec for TfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfr::R`](R) reader structure"]
impl crate::Readable for TfrSpec {}
#[doc = "`write(|w| ..)` method takes [`tfr::W`](W) writer structure"]
impl crate::Writable for TfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tfr to value 0"]
impl crate::Resettable for TfrSpec {
    const RESET_VALUE: u32 = 0;
}
