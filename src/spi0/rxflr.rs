#[doc = "Register `rxflr` reader"]
pub type R = crate::R<RxflrSpec>;
#[doc = "Register `rxflr` writer"]
pub type W = crate::W<RxflrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Receive FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxflrSpec;
impl crate::RegisterSpec for RxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxflr::R`](R) reader structure"]
impl crate::Readable for RxflrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxflr::W`](W) writer structure"]
impl crate::Writable for RxflrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rxflr to value 0"]
impl crate::Resettable for RxflrSpec {
    const RESET_VALUE: u32 = 0;
}
