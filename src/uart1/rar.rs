#[doc = "Register `rar` reader"]
pub type R = crate::R<RarSpec>;
#[doc = "Register `rar` writer"]
pub type W = crate::W<RarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Receive-Mode Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RarSpec;
impl crate::RegisterSpec for RarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rar::R`](R) reader structure"]
impl crate::Readable for RarSpec {}
#[doc = "`write(|w| ..)` method takes [`rar::W`](W) writer structure"]
impl crate::Writable for RarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rar to value 0"]
impl crate::Resettable for RarSpec {
    const RESET_VALUE: u32 = 0;
}
