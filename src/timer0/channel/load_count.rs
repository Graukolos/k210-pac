#[doc = "Register `load_count` reader"]
pub type R = crate::R<LoadCountSpec>;
#[doc = "Register `load_count` writer"]
pub type W = crate::W<LoadCountSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Load Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`load_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoadCountSpec;
impl crate::RegisterSpec for LoadCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load_count::R`](R) reader structure"]
impl crate::Readable for LoadCountSpec {}
#[doc = "`write(|w| ..)` method takes [`load_count::W`](W) writer structure"]
impl crate::Writable for LoadCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets load_count to value 0"]
impl crate::Resettable for LoadCountSpec {
    const RESET_VALUE: u32 = 0;
}
