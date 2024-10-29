#[doc = "Register `dmasa` reader"]
pub type R = crate::R<DmasaSpec>;
#[doc = "Register `dmasa` writer"]
pub type W = crate::W<DmasaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Software Acknowledge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmasaSpec;
impl crate::RegisterSpec for DmasaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasa::R`](R) reader structure"]
impl crate::Readable for DmasaSpec {}
#[doc = "`write(|w| ..)` method takes [`dmasa::W`](W) writer structure"]
impl crate::Writable for DmasaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dmasa to value 0"]
impl crate::Resettable for DmasaSpec {
    const RESET_VALUE: u32 = 0;
}
