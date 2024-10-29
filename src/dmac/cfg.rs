#[doc = "Register `cfg` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `cfg` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `dmac_en` reader - Enable DMAC"]
pub type DmacEnR = crate::BitReader;
#[doc = "Field `dmac_en` writer - Enable DMAC"]
pub type DmacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `int_en` reader - Globally enable interrupt generation"]
pub type IntEnR = crate::BitReader;
#[doc = "Field `int_en` writer - Globally enable interrupt generation"]
pub type IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable DMAC"]
    #[inline(always)]
    pub fn dmac_en(&self) -> DmacEnR {
        DmacEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Globally enable interrupt generation"]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMAC"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_en(&mut self) -> DmacEnW<CfgSpec> {
        DmacEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Globally enable interrupt generation"]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> IntEnW<CfgSpec> {
        IntEnW::new(self, 1)
    }
}
#[doc = "Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets cfg to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u64 = 0;
}
