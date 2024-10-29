#[doc = "Register `eight_bit_mode` reader"]
pub type R = crate::R<EightBitModeSpec>;
#[doc = "Register `eight_bit_mode` writer"]
pub type W = crate::W<EightBitModeSpec>;
#[doc = "Field `eight_bit_mode` reader - Use 8-bit instead of 16-bit precision if set"]
pub type EightBitModeR = crate::BitReader;
#[doc = "Field `eight_bit_mode` writer - Use 8-bit instead of 16-bit precision if set"]
pub type EightBitModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Use 8-bit instead of 16-bit precision if set"]
    #[inline(always)]
    pub fn eight_bit_mode(&self) -> EightBitModeR {
        EightBitModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use 8-bit instead of 16-bit precision if set"]
    #[inline(always)]
    #[must_use]
    pub fn eight_bit_mode(&mut self) -> EightBitModeW<EightBitModeSpec> {
        EightBitModeW::new(self, 0)
    }
}
#[doc = "Eight bit mode\n\nYou can [`read`](crate::Reg::read) this register and get [`eight_bit_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eight_bit_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EightBitModeSpec;
impl crate::RegisterSpec for EightBitModeSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`eight_bit_mode::R`](R) reader structure"]
impl crate::Readable for EightBitModeSpec {}
#[doc = "`write(|w| ..)` method takes [`eight_bit_mode::W`](W) writer structure"]
impl crate::Writable for EightBitModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets eight_bit_mode to value 0"]
impl crate::Resettable for EightBitModeSpec {
    const RESET_VALUE: u64 = 0;
}
