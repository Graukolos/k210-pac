#[doc = "Register `comp_version` reader"]
pub type R = crate::R<CompVersionSpec>;
#[doc = "Register `comp_version` writer"]
pub type W = crate::W<CompVersionSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Component Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompVersionSpec;
impl crate::RegisterSpec for CompVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_version::R`](R) reader structure"]
impl crate::Readable for CompVersionSpec {}
#[doc = "`write(|w| ..)` method takes [`comp_version::W`](W) writer structure"]
impl crate::Writable for CompVersionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets comp_version to value 0"]
impl crate::Resettable for CompVersionSpec {
    const RESET_VALUE: u32 = 0;
}
