#[doc = "Register `srr` reader"]
pub type R = crate::R<SrrSpec>;
#[doc = "Register `srr` writer"]
pub type W = crate::W<SrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Software Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrrSpec;
impl crate::RegisterSpec for SrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srr::R`](R) reader structure"]
impl crate::Readable for SrrSpec {}
#[doc = "`write(|w| ..)` method takes [`srr::W`](W) writer structure"]
impl crate::Writable for SrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets srr to value 0"]
impl crate::Resettable for SrrSpec {
    const RESET_VALUE: u32 = 0;
}
