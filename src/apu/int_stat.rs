#[doc = "Register `int_stat` reader"]
pub type R = crate::R<IntStatSpec>;
#[doc = "Register `int_stat` writer"]
pub type W = crate::W<IntStatSpec>;
#[doc = "Field `dir_search_data_rdy` reader - Sound direction searching data ready interrupt event"]
pub type DirSearchDataRdyR = crate::BitReader;
#[doc = "Field `dir_search_data_rdy` writer - Sound direction searching data ready interrupt event"]
pub type DirSearchDataRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `voc_buf_data_rdy` reader - Voice output stream buffer data ready interrupt event"]
pub type VocBufDataRdyR = crate::BitReader;
#[doc = "Field `voc_buf_data_rdy` writer - Voice output stream buffer data ready interrupt event"]
pub type VocBufDataRdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sound direction searching data ready interrupt event"]
    #[inline(always)]
    pub fn dir_search_data_rdy(&self) -> DirSearchDataRdyR {
        DirSearchDataRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voice output stream buffer data ready interrupt event"]
    #[inline(always)]
    pub fn voc_buf_data_rdy(&self) -> VocBufDataRdyR {
        VocBufDataRdyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sound direction searching data ready interrupt event"]
    #[inline(always)]
    #[must_use]
    pub fn dir_search_data_rdy(&mut self) -> DirSearchDataRdyW<IntStatSpec> {
        DirSearchDataRdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Voice output stream buffer data ready interrupt event"]
    #[inline(always)]
    #[must_use]
    pub fn voc_buf_data_rdy(&mut self) -> VocBufDataRdyW<IntStatSpec> {
        VocBufDataRdyW::new(self, 1)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatSpec;
impl crate::RegisterSpec for IntStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_stat::R`](R) reader structure"]
impl crate::Readable for IntStatSpec {}
#[doc = "`write(|w| ..)` method takes [`int_stat::W`](W) writer structure"]
impl crate::Writable for IntStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets int_stat to value 0"]
impl crate::Resettable for IntStatSpec {
    const RESET_VALUE: u32 = 0;
}
