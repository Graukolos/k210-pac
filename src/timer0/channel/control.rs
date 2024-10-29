#[doc = "Register `control` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `control` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `enable` reader - ENABLE"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - ENABLE"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: FREE_MODE"]
    Free = 0,
    #[doc = "1: USER_MODE"]
    User = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `mode` reader - MODE"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Free,
            true => Mode::User,
        }
    }
    #[doc = "FREE_MODE"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == Mode::Free
    }
    #[doc = "USER_MODE"]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == Mode::User
    }
}
#[doc = "Field `mode` writer - MODE"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FREE_MODE"]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Free)
    }
    #[doc = "USER_MODE"]
    #[inline(always)]
    pub fn user(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::User)
    }
}
#[doc = "Field `interrupt` reader - INTERRUPT_MASK"]
pub type InterruptR = crate::BitReader;
#[doc = "Field `interrupt` writer - INTERRUPT_MASK"]
pub type InterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_enable` reader - PWM_ENABLE"]
pub type PwmEnableR = crate::BitReader;
#[doc = "Field `pwm_enable` writer - PWM_ENABLE"]
pub type PwmEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INTERRUPT_MASK"]
    #[inline(always)]
    pub fn interrupt(&self) -> InterruptR {
        InterruptR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM_ENABLE"]
    #[inline(always)]
    pub fn pwm_enable(&self) -> PwmEnableR {
        PwmEnableR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ControlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<ControlSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 2 - INTERRUPT_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt(&mut self) -> InterruptW<ControlSpec> {
        InterruptW::new(self, 2)
    }
    #[doc = "Bit 3 - PWM_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_enable(&mut self) -> PwmEnableW<ControlSpec> {
        PwmEnableW::new(self, 3)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
