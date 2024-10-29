#[doc = "Register `tat` reader"]
pub type R = crate::R<TatSpec>;
#[doc = "Register `tat` writer"]
pub type W = crate::W<TatSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Turn-Around Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TatSpec;
impl crate::RegisterSpec for TatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tat::R`](R) reader structure"]
impl crate::Readable for TatSpec {}
#[doc = "`write(|w| ..)` method takes [`tat::W`](W) writer structure"]
impl crate::Writable for TatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tat to value 0"]
impl crate::Resettable for TatSpec {
    const RESET_VALUE: u32 = 0;
}
