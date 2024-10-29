#[doc = "Register `ter` reader"]
pub type R = crate::R<TerSpec>;
#[doc = "Register `ter` writer"]
pub type W = crate::W<TerSpec>;
#[doc = "Field `txchenx` reader - Transmit channel enable/disable"]
pub type TxchenxR = crate::BitReader;
#[doc = "Field `txchenx` writer - Transmit channel enable/disable"]
pub type TxchenxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit channel enable/disable"]
    #[inline(always)]
    pub fn txchenx(&self) -> TxchenxR {
        TxchenxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit channel enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn txchenx(&mut self) -> TxchenxW<TerSpec> {
        TxchenxW::new(self, 0)
    }
}
#[doc = "Transmit Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TerSpec;
impl crate::RegisterSpec for TerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ter::R`](R) reader structure"]
impl crate::Readable for TerSpec {}
#[doc = "`write(|w| ..)` method takes [`ter::W`](W) writer structure"]
impl crate::Writable for TerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ter to value 0"]
impl crate::Resettable for TerSpec {
    const RESET_VALUE: u32 = 0;
}
