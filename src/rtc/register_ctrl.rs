#[doc = "Register `register_ctrl` reader"]
pub type R = crate::R<RegisterCtrlSpec>;
#[doc = "Register `register_ctrl` writer"]
pub type W = crate::W<RegisterCtrlSpec>;
#[doc = "Field `read_enable` reader - RTC timer read enable"]
pub type ReadEnableR = crate::BitReader;
#[doc = "Field `read_enable` writer - RTC timer read enable"]
pub type ReadEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `write_enable` reader - RTC timer write enable"]
pub type WriteEnableR = crate::BitReader;
#[doc = "Field `write_enable` writer - RTC timer write enable"]
pub type WriteEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer_mask` reader - RTC timer mask"]
pub type TimerMaskR = crate::FieldReader;
#[doc = "Field `timer_mask` writer - RTC timer mask"]
pub type TimerMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `alarm_mask` reader - RTC alarm mask"]
pub type AlarmMaskR = crate::FieldReader;
#[doc = "Field `alarm_mask` writer - RTC alarm mask"]
pub type AlarmMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `initial_count_mask` reader - RTC counter initial count value mask"]
pub type InitialCountMaskR = crate::BitReader;
#[doc = "Field `initial_count_mask` writer - RTC counter initial count value mask"]
pub type InitialCountMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `interrupt_register_mask` reader - RTC interrupt register mask"]
pub type InterruptRegisterMaskR = crate::BitReader;
#[doc = "Field `interrupt_register_mask` writer - RTC interrupt register mask"]
pub type InterruptRegisterMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTC timer read enable"]
    #[inline(always)]
    pub fn read_enable(&self) -> ReadEnableR {
        ReadEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC timer write enable"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 13:20 - RTC timer mask"]
    #[inline(always)]
    pub fn timer_mask(&self) -> TimerMaskR {
        TimerMaskR::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:28 - RTC alarm mask"]
    #[inline(always)]
    pub fn alarm_mask(&self) -> AlarmMaskR {
        AlarmMaskR::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bit 29 - RTC counter initial count value mask"]
    #[inline(always)]
    pub fn initial_count_mask(&self) -> InitialCountMaskR {
        InitialCountMaskR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RTC interrupt register mask"]
    #[inline(always)]
    pub fn interrupt_register_mask(&self) -> InterruptRegisterMaskR {
        InterruptRegisterMaskR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC timer read enable"]
    #[inline(always)]
    #[must_use]
    pub fn read_enable(&mut self) -> ReadEnableW<RegisterCtrlSpec> {
        ReadEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - RTC timer write enable"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<RegisterCtrlSpec> {
        WriteEnableW::new(self, 1)
    }
    #[doc = "Bits 13:20 - RTC timer mask"]
    #[inline(always)]
    #[must_use]
    pub fn timer_mask(&mut self) -> TimerMaskW<RegisterCtrlSpec> {
        TimerMaskW::new(self, 13)
    }
    #[doc = "Bits 21:28 - RTC alarm mask"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_mask(&mut self) -> AlarmMaskW<RegisterCtrlSpec> {
        AlarmMaskW::new(self, 21)
    }
    #[doc = "Bit 29 - RTC counter initial count value mask"]
    #[inline(always)]
    #[must_use]
    pub fn initial_count_mask(&mut self) -> InitialCountMaskW<RegisterCtrlSpec> {
        InitialCountMaskW::new(self, 29)
    }
    #[doc = "Bit 30 - RTC interrupt register mask"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_register_mask(&mut self) -> InterruptRegisterMaskW<RegisterCtrlSpec> {
        InterruptRegisterMaskW::new(self, 30)
    }
}
#[doc = "RTC register settings\n\nYou can [`read`](crate::Reg::read) this register and get [`register_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegisterCtrlSpec;
impl crate::RegisterSpec for RegisterCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register_ctrl::R`](R) reader structure"]
impl crate::Readable for RegisterCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`register_ctrl::W`](W) writer structure"]
impl crate::Writable for RegisterCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets register_ctrl to value 0"]
impl crate::Resettable for RegisterCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
