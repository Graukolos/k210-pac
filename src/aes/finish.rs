#[doc = "Register `finish` reader"]
pub type R = crate::R<FinishSpec>;
#[doc = "Register `finish` writer"]
pub type W = crate::W<FinishSpec>;
#[doc = "AES operation finished status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Finish {
    #[doc = "0: Operation not finished"]
    NotFinished = 0,
    #[doc = "1: Operation finished"]
    Finished = 1,
}
impl From<Finish> for bool {
    #[inline(always)]
    fn from(variant: Finish) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `finish` reader - AES operation finished status"]
pub type FinishR = crate::BitReader<Finish>;
impl FinishR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Finish {
        match self.bits {
            false => Finish::NotFinished,
            true => Finish::Finished,
        }
    }
    #[doc = "Operation not finished"]
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == Finish::NotFinished
    }
    #[doc = "Operation finished"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == Finish::Finished
    }
}
#[doc = "Field `finish` writer - AES operation finished status"]
pub type FinishW<'a, REG> = crate::BitWriter<'a, REG, Finish>;
impl<'a, REG> FinishW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation not finished"]
    #[inline(always)]
    pub fn not_finished(self) -> &'a mut crate::W<REG> {
        self.variant(Finish::NotFinished)
    }
    #[doc = "Operation finished"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut crate::W<REG> {
        self.variant(Finish::Finished)
    }
}
impl R {
    #[doc = "Bit 0 - AES operation finished status"]
    #[inline(always)]
    pub fn finish(&self) -> FinishR {
        FinishR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES operation finished status"]
    #[inline(always)]
    #[must_use]
    pub fn finish(&mut self) -> FinishW<FinishSpec> {
        FinishW::new(self, 0)
    }
}
#[doc = "Finished status\n\nYou can [`read`](crate::Reg::read) this register and get [`finish::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`finish::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FinishSpec;
impl crate::RegisterSpec for FinishSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`finish::R`](R) reader structure"]
impl crate::Readable for FinishSpec {}
#[doc = "`write(|w| ..)` method takes [`finish::W`](W) writer structure"]
impl crate::Writable for FinishSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets finish to value 0"]
impl crate::Resettable for FinishSpec {
    const RESET_VALUE: u32 = 0;
}
