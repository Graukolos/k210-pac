#[doc = "Register `intstatus` reader"]
pub type R = crate::R<IntstatusSpec>;
#[doc = "Register `intstatus` writer"]
pub type W = crate::W<IntstatusSpec>;
#[doc = "Field `ch_intstat(1-6)` reader - Channel %s interrupt bit"]
pub type ChIntstatR = crate::BitReader;
#[doc = "Field `ch_intstat(1-6)` writer - Channel %s interrupt bit"]
pub type ChIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `commonreg_intstat` reader - Common register status bit"]
pub type CommonregIntstatR = crate::BitReader;
#[doc = "Field `commonreg_intstat` writer - Common register status bit"]
pub type CommonregIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Channel (1-6) interrupt bit"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_intstat` field.</div>"]
    #[inline(always)]
    pub fn ch_intstat(&self, n: u8) -> ChIntstatR {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChIntstatR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-6) interrupt bit"]
    #[inline(always)]
    pub fn ch_intstat_iter(&self) -> impl Iterator<Item = ChIntstatR> + '_ {
        (0..6).map(move |n| ChIntstatR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Channel 1 interrupt bit"]
    #[inline(always)]
    pub fn ch1_intstat(&self) -> ChIntstatR {
        ChIntstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 2 interrupt bit"]
    #[inline(always)]
    pub fn ch2_intstat(&self) -> ChIntstatR {
        ChIntstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 3 interrupt bit"]
    #[inline(always)]
    pub fn ch3_intstat(&self) -> ChIntstatR {
        ChIntstatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 4 interrupt bit"]
    #[inline(always)]
    pub fn ch4_intstat(&self) -> ChIntstatR {
        ChIntstatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 5 interrupt bit"]
    #[inline(always)]
    pub fn ch5_intstat(&self) -> ChIntstatR {
        ChIntstatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 6 interrupt bit"]
    #[inline(always)]
    pub fn ch6_intstat(&self) -> ChIntstatR {
        ChIntstatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Common register status bit"]
    #[inline(always)]
    pub fn commonreg_intstat(&self) -> CommonregIntstatR {
        CommonregIntstatR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Channel (1-6) interrupt bit"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_intstat` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn ch_intstat(&mut self, n: u8) -> ChIntstatW<IntstatusSpec> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChIntstatW::new(self, n)
    }
    #[doc = "Bit 0 - Channel 1 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_intstat(&mut self) -> ChIntstatW<IntstatusSpec> {
        ChIntstatW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 2 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_intstat(&mut self) -> ChIntstatW<IntstatusSpec> {
        ChIntstatW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 3 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_intstat(&mut self) -> ChIntstatW<IntstatusSpec> {
        ChIntstatW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 4 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_intstat(&mut self) -> ChIntstatW<IntstatusSpec> {
        ChIntstatW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 5 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_intstat(&mut self) -> ChIntstatW<IntstatusSpec> {
        ChIntstatW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 6 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_intstat(&mut self) -> ChIntstatW<IntstatusSpec> {
        ChIntstatW::new(self, 5)
    }
    #[doc = "Bit 16 - Common register status bit"]
    #[inline(always)]
    #[must_use]
    pub fn commonreg_intstat(&mut self) -> CommonregIntstatW<IntstatusSpec> {
        CommonregIntstatW::new(self, 16)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatusSpec;
impl crate::RegisterSpec for IntstatusSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intstatus::R`](R) reader structure"]
impl crate::Readable for IntstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`intstatus::W`](W) writer structure"]
impl crate::Writable for IntstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets intstatus to value 0"]
impl crate::Resettable for IntstatusSpec {
    const RESET_VALUE: u64 = 0;
}
