#[doc = "Register `data_input` reader"]
pub type R = crate::R<DataInputSpec>;
#[doc = "Register `data_input` writer"]
pub type W = crate::W<DataInputSpec>;
#[doc = "Field `pin(0-7)` reader - "]
pub type PinR = crate::BitReader;
#[doc = "Field `pin(0-7)` writer - "]
pub type PinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `pin0` field.</div>"]
    #[inline(always)]
    pub fn pin(&self, n: u8) -> PinR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PinR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = PinR> + '_ {
        (0..8).map(move |n| PinR::new(((self.bits >> n) & 1) != 0))
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
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `pin0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self, n: u8) -> PinW<DataInputSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PinW::new(self, n)
    }
    #[doc = "Bit 0 - pin0"]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> PinW<DataInputSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bit 1 - pin1"]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> PinW<DataInputSpec> {
        PinW::new(self, 1)
    }
    #[doc = "Bit 2 - pin2"]
    #[inline(always)]
    #[must_use]
    pub fn pin2(&mut self) -> PinW<DataInputSpec> {
        PinW::new(self, 2)
    }
    #[doc = "Bit 3 - pin3"]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> PinW<DataInputSpec> {
        PinW::new(self, 3)
    }
    #[doc = "Bit 4 - pin4"]
    #[inline(always)]
    #[must_use]
    pub fn pin4(&mut self) -> PinW<DataInputSpec> {
        PinW::new(self, 4)
    }
    #[doc = "Bit 5 - pin5"]
    #[inline(always)]
    #[must_use]
    pub fn pin5(&mut self) -> PinW<DataInputSpec> {
        PinW::new(self, 5)
    }
    #[doc = "Bit 6 - pin6"]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> PinW<DataInputSpec> {
        PinW::new(self, 6)
    }
    #[doc = "Bit 7 - pin7"]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> PinW<DataInputSpec> {
        PinW::new(self, 7)
    }
}
#[doc = "External port (data input) registers\n\nYou can [`read`](crate::Reg::read) this register and get [`data_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataInputSpec;
impl crate::RegisterSpec for DataInputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_input::R`](R) reader structure"]
impl crate::Readable for DataInputSpec {}
#[doc = "`write(|w| ..)` method takes [`data_input::W`](W) writer structure"]
impl crate::Writable for DataInputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets data_input to value 0"]
impl crate::Resettable for DataInputSpec {
    const RESET_VALUE: u32 = 0;
}