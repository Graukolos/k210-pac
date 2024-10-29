#[doc = "Register `endian` reader"]
pub type R = crate::R<EndianSpec>;
#[doc = "Register `endian` writer"]
pub type W = crate::W<EndianSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ENDIAN\n\nYou can [`read`](crate::Reg::read) this register and get [`endian::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endian::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndianSpec;
impl crate::RegisterSpec for EndianSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endian::R`](R) reader structure"]
impl crate::Readable for EndianSpec {}
#[doc = "`write(|w| ..)` method takes [`endian::W`](W) writer structure"]
impl crate::Writable for EndianSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets endian to value 0"]
impl crate::Resettable for EndianSpec {
    const RESET_VALUE: u32 = 0;
}
