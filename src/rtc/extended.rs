#[doc = "Register `extended` reader"]
pub type R = crate::R<ExtendedSpec>;
#[doc = "Register `extended` writer"]
pub type W = crate::W<ExtendedSpec>;
#[doc = "Field `century` reader - Century. Range \\[0,31\\]"]
pub type CenturyR = crate::FieldReader;
#[doc = "Field `century` writer - Century. Range \\[0,31\\]"]
pub type CenturyW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Is leap year. 1 is leap year, 0 is not leap year\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeapYear {
    #[doc = "0: 0 is not leap year"]
    NotLeap = 0,
    #[doc = "1: 1 is leap year"]
    Leap = 1,
}
impl From<LeapYear> for bool {
    #[inline(always)]
    fn from(variant: LeapYear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `leap_year` reader - Is leap year. 1 is leap year, 0 is not leap year"]
pub type LeapYearR = crate::BitReader<LeapYear>;
impl LeapYearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LeapYear {
        match self.bits {
            false => LeapYear::NotLeap,
            true => LeapYear::Leap,
        }
    }
    #[doc = "0 is not leap year"]
    #[inline(always)]
    pub fn is_not_leap(&self) -> bool {
        *self == LeapYear::NotLeap
    }
    #[doc = "1 is leap year"]
    #[inline(always)]
    pub fn is_leap(&self) -> bool {
        *self == LeapYear::Leap
    }
}
#[doc = "Field `leap_year` writer - Is leap year. 1 is leap year, 0 is not leap year"]
pub type LeapYearW<'a, REG> = crate::BitWriter<'a, REG, LeapYear>;
impl<'a, REG> LeapYearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0 is not leap year"]
    #[inline(always)]
    pub fn not_leap(self) -> &'a mut crate::W<REG> {
        self.variant(LeapYear::NotLeap)
    }
    #[doc = "1 is leap year"]
    #[inline(always)]
    pub fn leap(self) -> &'a mut crate::W<REG> {
        self.variant(LeapYear::Leap)
    }
}
impl R {
    #[doc = "Bits 0:4 - Century. Range \\[0,31\\]"]
    #[inline(always)]
    pub fn century(&self) -> CenturyR {
        CenturyR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Is leap year. 1 is leap year, 0 is not leap year"]
    #[inline(always)]
    pub fn leap_year(&self) -> LeapYearR {
        LeapYearR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Century. Range \\[0,31\\]"]
    #[inline(always)]
    #[must_use]
    pub fn century(&mut self) -> CenturyW<ExtendedSpec> {
        CenturyW::new(self, 0)
    }
    #[doc = "Bit 5 - Is leap year. 1 is leap year, 0 is not leap year"]
    #[inline(always)]
    #[must_use]
    pub fn leap_year(&mut self) -> LeapYearW<ExtendedSpec> {
        LeapYearW::new(self, 5)
    }
}
#[doc = "Timer extended information\n\nYou can [`read`](crate::Reg::read) this register and get [`extended::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extended::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtendedSpec;
impl crate::RegisterSpec for ExtendedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extended::R`](R) reader structure"]
impl crate::Readable for ExtendedSpec {}
#[doc = "`write(|w| ..)` method takes [`extended::W`](W) writer structure"]
impl crate::Writable for ExtendedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets extended to value 0"]
impl crate::Resettable for ExtendedSpec {
    const RESET_VALUE: u32 = 0;
}
