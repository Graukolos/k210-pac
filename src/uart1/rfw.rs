#[doc = "Register `rfw` reader"]
pub type R = crate::R<RfwSpec>;
#[doc = "Register `rfw` writer"]
pub type W = crate::W<RfwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Receive FIFO Write Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfwSpec;
impl crate::RegisterSpec for RfwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfw::R`](R) reader structure"]
impl crate::Readable for RfwSpec {}
#[doc = "`write(|w| ..)` method takes [`rfw::W`](W) writer structure"]
impl crate::Writable for RfwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rfw to value 0"]
impl crate::Resettable for RfwSpec {
    const RESET_VALUE: u32 = 0;
}
