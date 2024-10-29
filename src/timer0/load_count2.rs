#[doc = "Register `load_count2%s` reader"]
pub type R = crate::R<LoadCount2Spec>;
#[doc = "Register `load_count2%s` writer"]
pub type W = crate::W<LoadCount2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Load Count2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`load_count2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_count2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoadCount2Spec;
impl crate::RegisterSpec for LoadCount2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load_count2::R`](R) reader structure"]
impl crate::Readable for LoadCount2Spec {}
#[doc = "`write(|w| ..)` method takes [`load_count2::W`](W) writer structure"]
impl crate::Writable for LoadCount2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets load_count2%s to value 0"]
impl crate::Resettable for LoadCount2Spec {
    const RESET_VALUE: u32 = 0;
}
