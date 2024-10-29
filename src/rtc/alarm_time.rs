#[doc = "Register `alarm_time` reader"]
pub type R = crate::R<AlarmTimeSpec>;
#[doc = "Register `alarm_time` writer"]
pub type W = crate::W<AlarmTimeSpec>;
#[doc = "Field `second` reader - Second. Range \\[0,59\\]"]
pub type SecondR = crate::FieldReader;
#[doc = "Field `second` writer - Second. Range \\[0,59\\]"]
pub type SecondW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `minute` reader - Minute. Range \\[0,59\\]"]
pub type MinuteR = crate::FieldReader;
#[doc = "Field `minute` writer - Minute. Range \\[0,59\\]"]
pub type MinuteW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `hour` reader - Hour. Range \\[0,23\\]"]
pub type HourR = crate::FieldReader;
#[doc = "Field `hour` writer - Hour. Range \\[0,23\\]"]
pub type HourW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 10:15 - Second. Range \\[0,59\\]"]
    #[inline(always)]
    pub fn second(&self) -> SecondR {
        SecondR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Minute. Range \\[0,59\\]"]
    #[inline(always)]
    pub fn minute(&self) -> MinuteR {
        MinuteR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - Hour. Range \\[0,23\\]"]
    #[inline(always)]
    pub fn hour(&self) -> HourR {
        HourR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 10:15 - Second. Range \\[0,59\\]"]
    #[inline(always)]
    #[must_use]
    pub fn second(&mut self) -> SecondW<AlarmTimeSpec> {
        SecondW::new(self, 10)
    }
    #[doc = "Bits 16:21 - Minute. Range \\[0,59\\]"]
    #[inline(always)]
    #[must_use]
    pub fn minute(&mut self) -> MinuteW<AlarmTimeSpec> {
        MinuteW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Hour. Range \\[0,23\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HourW<AlarmTimeSpec> {
        HourW::new(self, 24)
    }
}
#[doc = "Alarm time information\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlarmTimeSpec;
impl crate::RegisterSpec for AlarmTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm_time::R`](R) reader structure"]
impl crate::Readable for AlarmTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`alarm_time::W`](W) writer structure"]
impl crate::Writable for AlarmTimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets alarm_time to value 0"]
impl crate::Resettable for AlarmTimeSpec {
    const RESET_VALUE: u32 = 0;
}
