#[doc = "Register `txdma` reader"]
pub type R = crate::R<TxdmaSpec>;
#[doc = "Register `txdma` writer"]
pub type W = crate::W<TxdmaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Transmitter Block DMA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdmaSpec;
impl crate::RegisterSpec for TxdmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdma::R`](R) reader structure"]
impl crate::Readable for TxdmaSpec {}
#[doc = "`write(|w| ..)` method takes [`txdma::W`](W) writer structure"]
impl crate::Writable for TxdmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets txdma to value 0"]
impl crate::Resettable for TxdmaSpec {
    const RESET_VALUE: u32 = 0;
}
