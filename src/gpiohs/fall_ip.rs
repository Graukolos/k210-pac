#[doc = "Register `fall_ip` reader"]
pub type R = crate::R<FallIpSpec>;
#[doc = "Register `fall_ip` writer"]
pub type W = crate::W<FallIpSpec>;
#[doc = "Field `pin(0-31)` reader - "]
pub type PinR = crate::BitReader;
#[doc = "Field `pin(0-31)` writer - "]
pub type PinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `pin0` field.</div>"]
    #[inline(always)]
    pub fn pin(&self, n: u8) -> PinR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PinR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = PinR> + '_ {
        (0..32).map(move |n| PinR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - pin0"]
    #[inline(always)]
    pub fn pin0(&self) -> PinR {
        PinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pin1"]
    #[inline(always)]
    pub fn pin1(&self) -> PinR {
        PinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pin2"]
    #[inline(always)]
    pub fn pin2(&self) -> PinR {
        PinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pin3"]
    #[inline(always)]
    pub fn pin3(&self) -> PinR {
        PinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pin4"]
    #[inline(always)]
    pub fn pin4(&self) -> PinR {
        PinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - pin5"]
    #[inline(always)]
    pub fn pin5(&self) -> PinR {
        PinR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pin6"]
    #[inline(always)]
    pub fn pin6(&self) -> PinR {
        PinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pin7"]
    #[inline(always)]
    pub fn pin7(&self) -> PinR {
        PinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pin8"]
    #[inline(always)]
    pub fn pin8(&self) -> PinR {
        PinR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pin9"]
    #[inline(always)]
    pub fn pin9(&self) -> PinR {
        PinR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - pin10"]
    #[inline(always)]
    pub fn pin10(&self) -> PinR {
        PinR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pin11"]
    #[inline(always)]
    pub fn pin11(&self) -> PinR {
        PinR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - pin12"]
    #[inline(always)]
    pub fn pin12(&self) -> PinR {
        PinR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - pin13"]
    #[inline(always)]
    pub fn pin13(&self) -> PinR {
        PinR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - pin14"]
    #[inline(always)]
    pub fn pin14(&self) -> PinR {
        PinR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - pin15"]
    #[inline(always)]
    pub fn pin15(&self) -> PinR {
        PinR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - pin16"]
    #[inline(always)]
    pub fn pin16(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - pin17"]
    #[inline(always)]
    pub fn pin17(&self) -> PinR {
        PinR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - pin18"]
    #[inline(always)]
    pub fn pin18(&self) -> PinR {
        PinR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - pin19"]
    #[inline(always)]
    pub fn pin19(&self) -> PinR {
        PinR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - pin20"]
    #[inline(always)]
    pub fn pin20(&self) -> PinR {
        PinR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - pin21"]
    #[inline(always)]
    pub fn pin21(&self) -> PinR {
        PinR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - pin22"]
    #[inline(always)]
    pub fn pin22(&self) -> PinR {
        PinR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - pin23"]
    #[inline(always)]
    pub fn pin23(&self) -> PinR {
        PinR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - pin24"]
    #[inline(always)]
    pub fn pin24(&self) -> PinR {
        PinR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - pin25"]
    #[inline(always)]
    pub fn pin25(&self) -> PinR {
        PinR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - pin26"]
    #[inline(always)]
    pub fn pin26(&self) -> PinR {
        PinR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - pin27"]
    #[inline(always)]
    pub fn pin27(&self) -> PinR {
        PinR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - pin28"]
    #[inline(always)]
    pub fn pin28(&self) -> PinR {
        PinR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - pin29"]
    #[inline(always)]
    pub fn pin29(&self) -> PinR {
        PinR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - pin30"]
    #[inline(always)]
    pub fn pin30(&self) -> PinR {
        PinR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - pin31"]
    #[inline(always)]
    pub fn pin31(&self) -> PinR {
        PinR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `pin0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self, n: u8) -> PinW<FallIpSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        PinW::new(self, n)
    }
    #[doc = "Bit 0 - pin0"]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bit 1 - pin1"]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 1)
    }
    #[doc = "Bit 2 - pin2"]
    #[inline(always)]
    #[must_use]
    pub fn pin2(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 2)
    }
    #[doc = "Bit 3 - pin3"]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 3)
    }
    #[doc = "Bit 4 - pin4"]
    #[inline(always)]
    #[must_use]
    pub fn pin4(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 4)
    }
    #[doc = "Bit 5 - pin5"]
    #[inline(always)]
    #[must_use]
    pub fn pin5(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 5)
    }
    #[doc = "Bit 6 - pin6"]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 6)
    }
    #[doc = "Bit 7 - pin7"]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 7)
    }
    #[doc = "Bit 8 - pin8"]
    #[inline(always)]
    #[must_use]
    pub fn pin8(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 8)
    }
    #[doc = "Bit 9 - pin9"]
    #[inline(always)]
    #[must_use]
    pub fn pin9(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 9)
    }
    #[doc = "Bit 10 - pin10"]
    #[inline(always)]
    #[must_use]
    pub fn pin10(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 10)
    }
    #[doc = "Bit 11 - pin11"]
    #[inline(always)]
    #[must_use]
    pub fn pin11(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 11)
    }
    #[doc = "Bit 12 - pin12"]
    #[inline(always)]
    #[must_use]
    pub fn pin12(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 12)
    }
    #[doc = "Bit 13 - pin13"]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 13)
    }
    #[doc = "Bit 14 - pin14"]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 14)
    }
    #[doc = "Bit 15 - pin15"]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 15)
    }
    #[doc = "Bit 16 - pin16"]
    #[inline(always)]
    #[must_use]
    pub fn pin16(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 16)
    }
    #[doc = "Bit 17 - pin17"]
    #[inline(always)]
    #[must_use]
    pub fn pin17(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 17)
    }
    #[doc = "Bit 18 - pin18"]
    #[inline(always)]
    #[must_use]
    pub fn pin18(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 18)
    }
    #[doc = "Bit 19 - pin19"]
    #[inline(always)]
    #[must_use]
    pub fn pin19(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 19)
    }
    #[doc = "Bit 20 - pin20"]
    #[inline(always)]
    #[must_use]
    pub fn pin20(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 20)
    }
    #[doc = "Bit 21 - pin21"]
    #[inline(always)]
    #[must_use]
    pub fn pin21(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 21)
    }
    #[doc = "Bit 22 - pin22"]
    #[inline(always)]
    #[must_use]
    pub fn pin22(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 22)
    }
    #[doc = "Bit 23 - pin23"]
    #[inline(always)]
    #[must_use]
    pub fn pin23(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 23)
    }
    #[doc = "Bit 24 - pin24"]
    #[inline(always)]
    #[must_use]
    pub fn pin24(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 24)
    }
    #[doc = "Bit 25 - pin25"]
    #[inline(always)]
    #[must_use]
    pub fn pin25(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 25)
    }
    #[doc = "Bit 26 - pin26"]
    #[inline(always)]
    #[must_use]
    pub fn pin26(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 26)
    }
    #[doc = "Bit 27 - pin27"]
    #[inline(always)]
    #[must_use]
    pub fn pin27(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 27)
    }
    #[doc = "Bit 28 - pin28"]
    #[inline(always)]
    #[must_use]
    pub fn pin28(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 28)
    }
    #[doc = "Bit 29 - pin29"]
    #[inline(always)]
    #[must_use]
    pub fn pin29(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 29)
    }
    #[doc = "Bit 30 - pin30"]
    #[inline(always)]
    #[must_use]
    pub fn pin30(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 30)
    }
    #[doc = "Bit 31 - pin31"]
    #[inline(always)]
    #[must_use]
    pub fn pin31(&mut self) -> PinW<FallIpSpec> {
        PinW::new(self, 31)
    }
}
#[doc = "Fall Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fall_ip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fall_ip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FallIpSpec;
impl crate::RegisterSpec for FallIpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fall_ip::R`](R) reader structure"]
impl crate::Readable for FallIpSpec {}
#[doc = "`write(|w| ..)` method takes [`fall_ip::W`](W) writer structure"]
impl crate::Writable for FallIpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fall_ip to value 0"]
impl crate::Resettable for FallIpSpec {
    const RESET_VALUE: u32 = 0;
}
