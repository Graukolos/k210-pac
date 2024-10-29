#[doc = "Register `dmardlr` reader"]
pub type R = crate::R<DmardlrSpec>;
#[doc = "Register `dmardlr` writer"]
pub type W = crate::W<DmardlrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Receive Data Level\n\nYou can [`read`](crate::Reg::read) this register and get [`dmardlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmardlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmardlrSpec;
impl crate::RegisterSpec for DmardlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmardlr::R`](R) reader structure"]
impl crate::Readable for DmardlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmardlr::W`](W) writer structure"]
impl crate::Writable for DmardlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dmardlr to value 0"]
impl crate::Resettable for DmardlrSpec {
    const RESET_VALUE: u32 = 0;
}
