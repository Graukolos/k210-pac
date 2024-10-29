#[doc = "Register `id_code` reader"]
pub type R = crate::R<IdCodeSpec>;
#[doc = "Register `id_code` writer"]
pub type W = crate::W<IdCodeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ID code\n\nYou can [`read`](crate::Reg::read) this register and get [`id_code::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_code::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdCodeSpec;
impl crate::RegisterSpec for IdCodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_code::R`](R) reader structure"]
impl crate::Readable for IdCodeSpec {}
#[doc = "`write(|w| ..)` method takes [`id_code::W`](W) writer structure"]
impl crate::Writable for IdCodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets id_code to value 0"]
impl crate::Resettable for IdCodeSpec {
    const RESET_VALUE: u32 = 0;
}
