#[doc = "Register `aad_num` reader"]
pub type R = crate::R<AadNumSpec>;
#[doc = "Register `aad_num` writer"]
pub type W = crate::W<AadNumSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GCM additional authenticated data count in bytes, minus one\n\nYou can [`read`](crate::Reg::read) this register and get [`aad_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aad_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AadNumSpec;
impl crate::RegisterSpec for AadNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aad_num::R`](R) reader structure"]
impl crate::Readable for AadNumSpec {}
#[doc = "`write(|w| ..)` method takes [`aad_num::W`](W) writer structure"]
impl crate::Writable for AadNumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets aad_num to value 0"]
impl crate::Resettable for AadNumSpec {
    const RESET_VALUE: u32 = 0;
}
