#[doc = "Register `date` reader"]
pub type R = crate::R<DateSpec>;
#[doc = "Register `date` writer"]
pub type W = crate::W<DateSpec>;
#[doc = "Field `week` reader - Week. Range \\[0,6\\]. 0 is Sunday."]
pub type WeekR = crate::FieldReader;
#[doc = "Field `week` writer - Week. Range \\[0,6\\]. 0 is Sunday."]
pub type WeekW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `day` reader - Day. Range \\[1,31\\]
or \\[1,30\\]
or \\[1,29\\]
or \\[1,28\\]"]
pub type DayR = crate::FieldReader;
#[doc = "Field `day` writer - Day. Range \\[1,31\\]
or \\[1,30\\]
or \\[1,29\\]
or \\[1,28\\]"]
pub type DayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `month` reader - Month. Range \\[1,12\\]"]
pub type MonthR = crate::FieldReader;
#[doc = "Field `month` writer - Month. Range \\[1,12\\]"]
pub type MonthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `year` reader - Year. Range \\[0,99\\]"]
pub type YearR = crate::FieldReader<u16>;
#[doc = "Field `year` writer - Year. Range \\[0,99\\]"]
pub type YearW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:2 - Week. Range \\[0,6\\]. 0 is Sunday."]
    #[inline(always)]
    pub fn week(&self) -> WeekR {
        WeekR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - Day. Range \\[1,31\\]
or \\[1,30\\]
or \\[1,29\\]
or \\[1,28\\]"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Month. Range \\[1,12\\]"]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Year. Range \\[0,99\\]"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Week. Range \\[0,6\\]. 0 is Sunday."]
    #[inline(always)]
    #[must_use]
    pub fn week(&mut self) -> WeekW<DateSpec> {
        WeekW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Day. Range \\[1,31\\]
or \\[1,30\\]
or \\[1,29\\]
or \\[1,28\\]"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DayW<DateSpec> {
        DayW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Month. Range \\[1,12\\]"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MonthW<DateSpec> {
        MonthW::new(self, 16)
    }
    #[doc = "Bits 20:31 - Year. Range \\[0,99\\]"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YearW<DateSpec> {
        YearW::new(self, 20)
    }
}
#[doc = "Timer date information\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DateSpec;
impl crate::RegisterSpec for DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DateSpec {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets date to value 0"]
impl crate::Resettable for DateSpec {
    const RESET_VALUE: u32 = 0;
}
