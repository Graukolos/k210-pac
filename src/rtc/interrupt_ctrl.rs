#[doc = "Register `interrupt_ctrl` reader"]
pub type R = crate::R<InterruptCtrlSpec>;
#[doc = "Register `interrupt_ctrl` writer"]
pub type W = crate::W<InterruptCtrlSpec>;
#[doc = "Field `tick_enable` reader - TICK_ENABLE"]
pub type TickEnableR = crate::BitReader;
#[doc = "Field `tick_enable` writer - TICK_ENABLE"]
pub type TickEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `alarm_enable` reader - Alarm interrupt enable"]
pub type AlarmEnableR = crate::BitReader;
#[doc = "Field `alarm_enable` writer - Alarm interrupt enable"]
pub type AlarmEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tick_int_mode` reader - Tick interrupt enable"]
pub type TickIntModeR = crate::FieldReader;
#[doc = "Field `tick_int_mode` writer - Tick interrupt enable"]
pub type TickIntModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `alarm_compare_mask` reader - Alarm compare mask for interrupt"]
pub type AlarmCompareMaskR = crate::FieldReader;
#[doc = "Field `alarm_compare_mask` writer - Alarm compare mask for interrupt"]
pub type AlarmCompareMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - TICK_ENABLE"]
    #[inline(always)]
    pub fn tick_enable(&self) -> TickEnableR {
        TickEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alarm_enable(&self) -> AlarmEnableR {
        AlarmEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Tick interrupt enable"]
    #[inline(always)]
    pub fn tick_int_mode(&self) -> TickIntModeR {
        TickIntModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Alarm compare mask for interrupt"]
    #[inline(always)]
    pub fn alarm_compare_mask(&self) -> AlarmCompareMaskR {
        AlarmCompareMaskR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TICK_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn tick_enable(&mut self) -> TickEnableW<InterruptCtrlSpec> {
        TickEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_enable(&mut self) -> AlarmEnableW<InterruptCtrlSpec> {
        AlarmEnableW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Tick interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tick_int_mode(&mut self) -> TickIntModeW<InterruptCtrlSpec> {
        TickIntModeW::new(self, 2)
    }
    #[doc = "Bits 24:31 - Alarm compare mask for interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_compare_mask(&mut self) -> AlarmCompareMaskW<InterruptCtrlSpec> {
        AlarmCompareMaskW::new(self, 24)
    }
}
#[doc = "RTC interrupt settings\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptCtrlSpec;
impl crate::RegisterSpec for InterruptCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_ctrl::R`](R) reader structure"]
impl crate::Readable for InterruptCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_ctrl::W`](W) writer structure"]
impl crate::Writable for InterruptCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets interrupt_ctrl to value 0"]
impl crate::Resettable for InterruptCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
