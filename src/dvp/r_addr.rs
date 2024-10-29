#[doc = "Register `r_addr` reader"]
pub type R = crate::R<RAddrSpec>;
#[doc = "Register `r_addr` writer"]
pub type W = crate::W<RAddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "R_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`r_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAddrSpec;
impl crate::RegisterSpec for RAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_addr::R`](R) reader structure"]
impl crate::Readable for RAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`r_addr::W`](W) writer structure"]
impl crate::Writable for RAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets r_addr to value 0"]
impl crate::Resettable for RAddrSpec {
    const RESET_VALUE: u32 = 0;
}
