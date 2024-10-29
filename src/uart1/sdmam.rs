#[doc = "Register `sdmam` reader"]
pub type R = crate::R<SdmamSpec>;
#[doc = "Register `sdmam` writer"]
pub type W = crate::W<SdmamSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Shadow DMA Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmamSpec;
impl crate::RegisterSpec for SdmamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmam::R`](R) reader structure"]
impl crate::Readable for SdmamSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmam::W`](W) writer structure"]
impl crate::Writable for SdmamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sdmam to value 0"]
impl crate::Resettable for SdmamSpec {
    const RESET_VALUE: u32 = 0;
}
