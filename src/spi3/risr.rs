#[doc = "Register `risr` reader"]
pub type R = crate::R<RisrSpec>;
#[doc = "Register `risr` writer"]
pub type W = crate::W<RisrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`risr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisrSpec;
impl crate::RegisterSpec for RisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`risr::R`](R) reader structure"]
impl crate::Readable for RisrSpec {}
#[doc = "`write(|w| ..)` method takes [`risr::W`](W) writer structure"]
impl crate::Writable for RisrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets risr to value 0"]
impl crate::Resettable for RisrSpec {
    const RESET_VALUE: u32 = 0;
}
