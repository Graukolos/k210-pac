#[doc = "Register `source` reader"]
pub type R = crate::R<SourceSpec>;
#[doc = "Register `source` writer"]
pub type W = crate::W<SourceSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data source registers\n\nYou can [`read`](crate::Reg::read) this register and get [`source::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`source::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SourceSpec;
impl crate::RegisterSpec for SourceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`source::R`](R) reader structure"]
impl crate::Readable for SourceSpec {}
#[doc = "`write(|w| ..)` method takes [`source::W`](W) writer structure"]
impl crate::Writable for SourceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets source to value 0"]
impl crate::Resettable for SourceSpec {
    const RESET_VALUE: u32 = 0;
}
