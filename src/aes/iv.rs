#[doc = "Register `iv[%s]` reader"]
pub type R = crate::R<IvSpec>;
#[doc = "Register `iv[%s]` writer"]
pub type W = crate::W<IvSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Initialisation Vector (96 bit for GCM, 128 bit for CBC)\n\nYou can [`read`](crate::Reg::read) this register and get [`iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IvSpec;
impl crate::RegisterSpec for IvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv::R`](R) reader structure"]
impl crate::Readable for IvSpec {}
#[doc = "`write(|w| ..)` method takes [`iv::W`](W) writer structure"]
impl crate::Writable for IvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets iv[%s]
to value 0"]
impl crate::Resettable for IvSpec {
    const RESET_VALUE: u32 = 0;
}
