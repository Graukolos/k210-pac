#[doc = "Register `sts` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `sts` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `frame_start` reader - FRAME_START"]
pub type FrameStartR = crate::BitReader;
#[doc = "Field `frame_start` writer - FRAME_START"]
pub type FrameStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frame_start_we` reader - FRAME_START_WE"]
pub type FrameStartWeR = crate::BitReader;
#[doc = "Field `frame_start_we` writer - FRAME_START_WE"]
pub type FrameStartWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frame_finish` reader - FRAME_FINISH"]
pub type FrameFinishR = crate::BitReader;
#[doc = "Field `frame_finish` writer - FRAME_FINISH"]
pub type FrameFinishW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frame_finish_we` reader - FRAME_FINISH_WE"]
pub type FrameFinishWeR = crate::BitReader;
#[doc = "Field `frame_finish_we` writer - FRAME_FINISH_WE"]
pub type FrameFinishWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dvp_en` reader - DVP_EN"]
pub type DvpEnR = crate::BitReader;
#[doc = "Field `dvp_en` writer - DVP_EN"]
pub type DvpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dvp_en_we` reader - DVP_EN_WE"]
pub type DvpEnWeR = crate::BitReader;
#[doc = "Field `dvp_en_we` writer - DVP_EN_WE"]
pub type DvpEnWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sccb_en` reader - SCCB_EN"]
pub type SccbEnR = crate::BitReader;
#[doc = "Field `sccb_en` writer - SCCB_EN"]
pub type SccbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sccb_en_we` reader - SCCB_EN_WE"]
pub type SccbEnWeR = crate::BitReader;
#[doc = "Field `sccb_en_we` writer - SCCB_EN_WE"]
pub type SccbEnWeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FRAME_START"]
    #[inline(always)]
    pub fn frame_start(&self) -> FrameStartR {
        FrameStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FRAME_START_WE"]
    #[inline(always)]
    pub fn frame_start_we(&self) -> FrameStartWeR {
        FrameStartWeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - FRAME_FINISH"]
    #[inline(always)]
    pub fn frame_finish(&self) -> FrameFinishR {
        FrameFinishR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FRAME_FINISH_WE"]
    #[inline(always)]
    pub fn frame_finish_we(&self) -> FrameFinishWeR {
        FrameFinishWeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - DVP_EN"]
    #[inline(always)]
    pub fn dvp_en(&self) -> DvpEnR {
        DvpEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DVP_EN_WE"]
    #[inline(always)]
    pub fn dvp_en_we(&self) -> DvpEnWeR {
        DvpEnWeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - SCCB_EN"]
    #[inline(always)]
    pub fn sccb_en(&self) -> SccbEnR {
        SccbEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCCB_EN_WE"]
    #[inline(always)]
    pub fn sccb_en_we(&self) -> SccbEnWeR {
        SccbEnWeR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FRAME_START"]
    #[inline(always)]
    #[must_use]
    pub fn frame_start(&mut self) -> FrameStartW<StsSpec> {
        FrameStartW::new(self, 0)
    }
    #[doc = "Bit 1 - FRAME_START_WE"]
    #[inline(always)]
    #[must_use]
    pub fn frame_start_we(&mut self) -> FrameStartWeW<StsSpec> {
        FrameStartWeW::new(self, 1)
    }
    #[doc = "Bit 8 - FRAME_FINISH"]
    #[inline(always)]
    #[must_use]
    pub fn frame_finish(&mut self) -> FrameFinishW<StsSpec> {
        FrameFinishW::new(self, 8)
    }
    #[doc = "Bit 9 - FRAME_FINISH_WE"]
    #[inline(always)]
    #[must_use]
    pub fn frame_finish_we(&mut self) -> FrameFinishWeW<StsSpec> {
        FrameFinishWeW::new(self, 9)
    }
    #[doc = "Bit 16 - DVP_EN"]
    #[inline(always)]
    #[must_use]
    pub fn dvp_en(&mut self) -> DvpEnW<StsSpec> {
        DvpEnW::new(self, 16)
    }
    #[doc = "Bit 17 - DVP_EN_WE"]
    #[inline(always)]
    #[must_use]
    pub fn dvp_en_we(&mut self) -> DvpEnWeW<StsSpec> {
        DvpEnWeW::new(self, 17)
    }
    #[doc = "Bit 24 - SCCB_EN"]
    #[inline(always)]
    #[must_use]
    pub fn sccb_en(&mut self) -> SccbEnW<StsSpec> {
        SccbEnW::new(self, 24)
    }
    #[doc = "Bit 25 - SCCB_EN_WE"]
    #[inline(always)]
    #[must_use]
    pub fn sccb_en_we(&mut self) -> SccbEnWeW<StsSpec> {
        SccbEnWeW::new(self, 25)
    }
}
#[doc = "STS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sts to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}
