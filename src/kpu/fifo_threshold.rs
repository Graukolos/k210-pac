#[doc = "Register `fifo_threshold` reader"]
pub type R = crate::R<FifoThresholdSpec>;
#[doc = "Register `fifo_threshold` writer"]
pub type W = crate::W<FifoThresholdSpec>;
#[doc = "Field `full_threshold` reader - FIFO full threshold"]
pub type FullThresholdR = crate::FieldReader;
#[doc = "Field `full_threshold` writer - FIFO full threshold"]
pub type FullThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `empty_threshold` reader - FIFO empty threshold"]
pub type EmptyThresholdR = crate::FieldReader;
#[doc = "Field `empty_threshold` writer - FIFO empty threshold"]
pub type EmptyThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - FIFO full threshold"]
    #[inline(always)]
    pub fn full_threshold(&self) -> FullThresholdR {
        FullThresholdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FIFO empty threshold"]
    #[inline(always)]
    pub fn empty_threshold(&self) -> EmptyThresholdR {
        EmptyThresholdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - FIFO full threshold"]
    #[inline(always)]
    #[must_use]
    pub fn full_threshold(&mut self) -> FullThresholdW<FifoThresholdSpec> {
        FullThresholdW::new(self, 0)
    }
    #[doc = "Bits 4:7 - FIFO empty threshold"]
    #[inline(always)]
    #[must_use]
    pub fn empty_threshold(&mut self) -> EmptyThresholdW<FifoThresholdSpec> {
        EmptyThresholdW::new(self, 4)
    }
}
#[doc = "FIFO threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoThresholdSpec;
impl crate::RegisterSpec for FifoThresholdSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`fifo_threshold::R`](R) reader structure"]
impl crate::Readable for FifoThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_threshold::W`](W) writer structure"]
impl crate::Writable for FifoThresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets fifo_threshold to value 0"]
impl crate::Resettable for FifoThresholdSpec {
    const RESET_VALUE: u64 = 0;
}
