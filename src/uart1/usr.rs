#[doc = "Register `usr` reader"]
pub type R = crate::R<UsrSpec>;
#[doc = "Register `usr` writer"]
pub type W = crate::W<UsrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsrSpec;
impl crate::RegisterSpec for UsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usr::R`](R) reader structure"]
impl crate::Readable for UsrSpec {}
#[doc = "`write(|w| ..)` method takes [`usr::W`](W) writer structure"]
impl crate::Writable for UsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets usr to value 0"]
impl crate::Resettable for UsrSpec {
    const RESET_VALUE: u32 = 0;
}
