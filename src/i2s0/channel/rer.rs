#[doc = "Register `rer` reader"]
pub type R = crate::R<RerSpec>;
#[doc = "Register `rer` writer"]
pub type W = crate::W<RerSpec>;
#[doc = "Field `rxchenx` reader - Receive channel enable/disable"]
pub type RxchenxR = crate::BitReader;
#[doc = "Field `rxchenx` writer - Receive channel enable/disable"]
pub type RxchenxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive channel enable/disable"]
    #[inline(always)]
    pub fn rxchenx(&self) -> RxchenxR {
        RxchenxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive channel enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxchenx(&mut self) -> RxchenxW<RerSpec> {
        RxchenxW::new(self, 0)
    }
}
#[doc = "Receive Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RerSpec;
impl crate::RegisterSpec for RerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rer::R`](R) reader structure"]
impl crate::Readable for RerSpec {}
#[doc = "`write(|w| ..)` method takes [`rer::W`](W) writer structure"]
impl crate::Writable for RerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rer to value 0"]
impl crate::Resettable for RerSpec {
    const RESET_VALUE: u32 = 0;
}
