#[doc = "Register `dmatdlr` reader"]
pub type R = crate::R<DmatdlrSpec>;
#[doc = "Register `dmatdlr` writer"]
pub type W = crate::W<DmatdlrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Transmit Data Level\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatdlrSpec;
impl crate::RegisterSpec for DmatdlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatdlr::R`](R) reader structure"]
impl crate::Readable for DmatdlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatdlr::W`](W) writer structure"]
impl crate::Writable for DmatdlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dmatdlr to value 0"]
impl crate::Resettable for DmatdlrSpec {
    const RESET_VALUE: u32 = 0;
}
