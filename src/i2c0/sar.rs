#[doc = "Register `sar` reader"]
pub type R = crate::R<SarSpec>;
#[doc = "Register `sar` writer"]
pub type W = crate::W<SarSpec>;
#[doc = "Field `address` reader - Slave Address"]
pub type AddressR = crate::FieldReader<u16>;
#[doc = "Field `address` writer - Slave Address"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Slave Address"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<SarSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SarSpec;
impl crate::RegisterSpec for SarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar::R`](R) reader structure"]
impl crate::Readable for SarSpec {}
#[doc = "`write(|w| ..)` method takes [`sar::W`](W) writer structure"]
impl crate::Writable for SarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sar to value 0"]
impl crate::Resettable for SarSpec {
    const RESET_VALUE: u32 = 0;
}
