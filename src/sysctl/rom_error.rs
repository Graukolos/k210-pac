#[doc = "Register `rom_error` reader"]
pub type R = crate::R<RomErrorSpec>;
#[doc = "Register `rom_error` writer"]
pub type W = crate::W<RomErrorSpec>;
#[doc = "Field `rom_mul_error` reader - "]
pub type RomMulErrorR = crate::BitReader;
#[doc = "Field `rom_mul_error` writer - "]
pub type RomMulErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rom_one_error` reader - "]
pub type RomOneErrorR = crate::BitReader;
#[doc = "Field `rom_one_error` writer - "]
pub type RomOneErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rom_mul_error(&self) -> RomMulErrorR {
        RomMulErrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rom_one_error(&self) -> RomOneErrorR {
        RomOneErrorR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rom_mul_error(&mut self) -> RomMulErrorW<RomErrorSpec> {
        RomMulErrorW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rom_one_error(&mut self) -> RomOneErrorW<RomErrorSpec> {
        RomOneErrorW::new(self, 1)
    }
}
#[doc = "AXI ROM detector\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_error::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_error::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomErrorSpec;
impl crate::RegisterSpec for RomErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_error::R`](R) reader structure"]
impl crate::Readable for RomErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`rom_error::W`](W) writer structure"]
impl crate::Writable for RomErrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rom_error to value 0"]
impl crate::Resettable for RomErrorSpec {
    const RESET_VALUE: u32 = 0;
}
