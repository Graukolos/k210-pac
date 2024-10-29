#[doc = "Register `pre_fir0_coef[%s]` reader"]
pub type R = crate::R<PreFir0CoefSpec>;
#[doc = "Register `pre_fir0_coef[%s]` writer"]
pub type W = crate::W<PreFir0CoefSpec>;
#[doc = "Field `tap0` reader - Tap 0"]
pub type Tap0R = crate::FieldReader<u16>;
#[doc = "Field `tap0` writer - Tap 0"]
pub type Tap0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tap1` reader - Tap 1"]
pub type Tap1R = crate::FieldReader<u16>;
#[doc = "Field `tap1` writer - Tap 1"]
pub type Tap1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Tap 0"]
    #[inline(always)]
    pub fn tap0(&self) -> Tap0R {
        Tap0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Tap 1"]
    #[inline(always)]
    pub fn tap1(&self) -> Tap1R {
        Tap1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Tap 0"]
    #[inline(always)]
    #[must_use]
    pub fn tap0(&mut self) -> Tap0W<PreFir0CoefSpec> {
        Tap0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Tap 1"]
    #[inline(always)]
    #[must_use]
    pub fn tap1(&mut self) -> Tap1W<PreFir0CoefSpec> {
        Tap1W::new(self, 16)
    }
}
#[doc = "FIR0 pre-filter coefficients\n\nYou can [`read`](crate::Reg::read) this register and get [`pre_fir0_coef::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pre_fir0_coef::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PreFir0CoefSpec;
impl crate::RegisterSpec for PreFir0CoefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pre_fir0_coef::R`](R) reader structure"]
impl crate::Readable for PreFir0CoefSpec {}
#[doc = "`write(|w| ..)` method takes [`pre_fir0_coef::W`](W) writer structure"]
impl crate::Writable for PreFir0CoefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pre_fir0_coef[%s]
to value 0"]
impl crate::Resettable for PreFir0CoefSpec {
    const RESET_VALUE: u32 = 0;
}
