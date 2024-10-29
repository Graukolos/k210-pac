#[doc = "Register `dar` reader"]
pub type R = crate::R<DarSpec>;
#[doc = "Register `dar` writer"]
pub type W = crate::W<DarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DAR Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DarSpec;
impl crate::RegisterSpec for DarSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`dar::R`](R) reader structure"]
impl crate::Readable for DarSpec {}
#[doc = "`write(|w| ..)` method takes [`dar::W`](W) writer structure"]
impl crate::Writable for DarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets dar to value 0"]
impl crate::Resettable for DarSpec {
    const RESET_VALUE: u64 = 0;
}
