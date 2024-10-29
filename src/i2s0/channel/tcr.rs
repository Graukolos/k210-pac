#[doc = "Register `tcr` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `tcr` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Desired data resolution of transmitter"]
pub use super::rcr::Wlen;
#[doc = "Field `wlen` reader - Desired data resolution of transmitter"]
pub use super::rcr::WlenR;
#[doc = "Field `wlen` writer - Desired data resolution of transmitter"]
pub use super::rcr::WlenW;
impl R {
    #[doc = "Bits 0:2 - Desired data resolution of transmitter"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Desired data resolution of transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn wlen(&mut self) -> WlenW<TcrSpec> {
        WlenW::new(self, 0)
    }
}
#[doc = "Transmit Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tcr to value 0"]
impl crate::Resettable for TcrSpec {
    const RESET_VALUE: u32 = 0;
}
