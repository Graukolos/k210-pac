#[doc = "Register `cr` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `cr` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `enable` reader - enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "rmod\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmod {
    #[doc = "0: RESET"]
    Reset = 0,
    #[doc = "1: INTERRUPT"]
    Interrupt = 1,
}
impl From<Rmod> for bool {
    #[inline(always)]
    fn from(variant: Rmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rmod` reader - rmod"]
pub type RmodR = crate::BitReader<Rmod>;
impl RmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rmod {
        match self.bits {
            false => Rmod::Reset,
            true => Rmod::Interrupt,
        }
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Rmod::Reset
    }
    #[doc = "INTERRUPT"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Rmod::Interrupt
    }
}
#[doc = "Field `rmod` writer - rmod"]
pub type RmodW<'a, REG> = crate::BitWriter<'a, REG, Rmod>;
impl<'a, REG> RmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RESET"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rmod::Reset)
    }
    #[doc = "INTERRUPT"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Rmod::Interrupt)
    }
}
#[doc = "Field `rpl` reader - rpl"]
pub type RplR = crate::FieldReader;
#[doc = "Field `rpl` writer - rpl"]
pub type RplW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rmod"]
    #[inline(always)]
    pub fn rmod(&self) -> RmodR {
        RmodR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - rpl"]
    #[inline(always)]
    pub fn rpl(&self) -> RplR {
        RplR::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - rmod"]
    #[inline(always)]
    #[must_use]
    pub fn rmod(&mut self) -> RmodW<CrSpec> {
        RmodW::new(self, 1)
    }
    #[doc = "Bits 2:4 - rpl"]
    #[inline(always)]
    #[must_use]
    pub fn rpl(&mut self) -> RplW<CrSpec> {
        RplW::new(self, 2)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cr to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
