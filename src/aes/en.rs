#[doc = "Register `en` reader"]
pub type R = crate::R<EnSpec>;
#[doc = "Register `en` writer"]
pub type W = crate::W<EnSpec>;
#[doc = "AES module enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable module"]
    Disable = 0,
    #[doc = "1: Enable module"]
    Enable = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `en` reader - AES module enable"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disable,
            true => En::Enable,
        }
    }
    #[doc = "Disable module"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == En::Disable
    }
    #[doc = "Enable module"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == En::Enable
    }
}
#[doc = "Field `en` writer - AES module enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable module"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disable)
    }
    #[doc = "Enable module"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - AES module enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES module enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<EnSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "AES module enable\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnSpec;
impl crate::RegisterSpec for EnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en::R`](R) reader structure"]
impl crate::Readable for EnSpec {}
#[doc = "`write(|w| ..)` method takes [`en::W`](W) writer structure"]
impl crate::Writable for EnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets en to value 0"]
impl crate::Resettable for EnSpec {
    const RESET_VALUE: u32 = 0;
}