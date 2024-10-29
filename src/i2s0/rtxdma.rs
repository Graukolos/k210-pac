#[doc = "Register `rtxdma` reader"]
pub type R = crate::R<RtxdmaSpec>;
#[doc = "Register `rtxdma` writer"]
pub type W = crate::W<RtxdmaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reset Transmitter Block DMA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtxdma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtxdma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtxdmaSpec;
impl crate::RegisterSpec for RtxdmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtxdma::R`](R) reader structure"]
impl crate::Readable for RtxdmaSpec {}
#[doc = "`write(|w| ..)` method takes [`rtxdma::W`](W) writer structure"]
impl crate::Writable for RtxdmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rtxdma to value 0"]
impl crate::Resettable for RtxdmaSpec {
    const RESET_VALUE: u32 = 0;
}
