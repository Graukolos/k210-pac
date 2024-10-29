#[doc = "Register `ctrlr1` reader"]
pub type R = crate::R<Ctrlr1Spec>;
#[doc = "Register `ctrlr1` writer"]
pub type W = crate::W<Ctrlr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrlr1Spec;
impl crate::RegisterSpec for Ctrlr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlr1::R`](R) reader structure"]
impl crate::Readable for Ctrlr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrlr1::W`](W) writer structure"]
impl crate::Writable for Ctrlr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctrlr1 to value 0"]
impl crate::Resettable for Ctrlr1Spec {
    const RESET_VALUE: u32 = 0;
}