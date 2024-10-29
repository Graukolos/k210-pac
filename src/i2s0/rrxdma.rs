#[doc = "Register `rrxdma` reader"]
pub type R = crate::R<RrxdmaSpec>;
#[doc = "Register `rrxdma` writer"]
pub type W = crate::W<RrxdmaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reset Receiver Block DMA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rrxdma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrxdma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrxdmaSpec;
impl crate::RegisterSpec for RrxdmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrxdma::R`](R) reader structure"]
impl crate::Readable for RrxdmaSpec {}
#[doc = "`write(|w| ..)` method takes [`rrxdma::W`](W) writer structure"]
impl crate::Writable for RrxdmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rrxdma to value 0"]
impl crate::Resettable for RrxdmaSpec {
    const RESET_VALUE: u32 = 0;
}
