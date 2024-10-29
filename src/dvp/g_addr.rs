#[doc = "Register `g_addr` reader"]
pub type R = crate::R<GAddrSpec>;
#[doc = "Register `g_addr` writer"]
pub type W = crate::W<GAddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "G_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`g_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`g_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAddrSpec;
impl crate::RegisterSpec for GAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`g_addr::R`](R) reader structure"]
impl crate::Readable for GAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`g_addr::W`](W) writer structure"]
impl crate::Writable for GAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets g_addr to value 0"]
impl crate::Resettable for GAddrSpec {
    const RESET_VALUE: u32 = 0;
}
