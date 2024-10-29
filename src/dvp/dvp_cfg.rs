#[doc = "Register `dvp_cfg` reader"]
pub type R = crate::R<DvpCfgSpec>;
#[doc = "Register `dvp_cfg` writer"]
pub type W = crate::W<DvpCfgSpec>;
#[doc = "Field `start_int_enable` reader - START_INT_ENABLE"]
pub type StartIntEnableR = crate::BitReader;
#[doc = "Field `start_int_enable` writer - START_INT_ENABLE"]
pub type StartIntEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `finish_int_enable` reader - FINISH_INT_ENABLE"]
pub type FinishIntEnableR = crate::BitReader;
#[doc = "Field `finish_int_enable` writer - FINISH_INT_ENABLE"]
pub type FinishIntEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ai_output_enable` reader - AI_OUTPUT_ENABLE"]
pub type AiOutputEnableR = crate::BitReader;
#[doc = "Field `ai_output_enable` writer - AI_OUTPUT_ENABLE"]
pub type AiOutputEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `display_output_enable` reader - DISPLAY_OUTPUT_ENABLE"]
pub type DisplayOutputEnableR = crate::BitReader;
#[doc = "Field `display_output_enable` writer - DISPLAY_OUTPUT_ENABLE"]
pub type DisplayOutputEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `auto_enable` reader - AUTO_ENABLE"]
pub type AutoEnableR = crate::BitReader;
#[doc = "Field `auto_enable` writer - AUTO_ENABLE"]
pub type AutoEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `burst_size_4beats` reader - BURST_SIZE_4BEATS"]
pub type BurstSize4beatsR = crate::BitReader;
#[doc = "Field `burst_size_4beats` writer - BURST_SIZE_4BEATS"]
pub type BurstSize4beatsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FORMAT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Format {
    #[doc = "0: RGB_FORMAT"]
    Rgb = 0,
    #[doc = "1: YUV_FORMAT"]
    Yuv = 1,
    #[doc = "3: Y_FORMAT"]
    Y = 3,
}
impl From<Format> for u8 {
    #[inline(always)]
    fn from(variant: Format) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Format {
    type Ux = u8;
}
impl crate::IsEnum for Format {}
#[doc = "Field `format` reader - FORMAT"]
pub type FormatR = crate::FieldReader<Format>;
impl FormatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Format> {
        match self.bits {
            0 => Some(Format::Rgb),
            1 => Some(Format::Yuv),
            3 => Some(Format::Y),
            _ => None,
        }
    }
    #[doc = "RGB_FORMAT"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == Format::Rgb
    }
    #[doc = "YUV_FORMAT"]
    #[inline(always)]
    pub fn is_yuv(&self) -> bool {
        *self == Format::Yuv
    }
    #[doc = "Y_FORMAT"]
    #[inline(always)]
    pub fn is_y(&self) -> bool {
        *self == Format::Y
    }
}
#[doc = "Field `format` writer - FORMAT"]
pub type FormatW<'a, REG> = crate::FieldWriter<'a, REG, 2, Format>;
impl<'a, REG> FormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RGB_FORMAT"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut crate::W<REG> {
        self.variant(Format::Rgb)
    }
    #[doc = "YUV_FORMAT"]
    #[inline(always)]
    pub fn yuv(self) -> &'a mut crate::W<REG> {
        self.variant(Format::Yuv)
    }
    #[doc = "Y_FORMAT"]
    #[inline(always)]
    pub fn y(self) -> &'a mut crate::W<REG> {
        self.variant(Format::Y)
    }
}
#[doc = "Field `href_burst_num` reader - HREF_BURST_NUM"]
pub type HrefBurstNumR = crate::FieldReader;
#[doc = "Field `href_burst_num` writer - HREF_BURST_NUM"]
pub type HrefBurstNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `line_num` reader - LINE_NUM"]
pub type LineNumR = crate::FieldReader<u16>;
#[doc = "Field `line_num` writer - LINE_NUM"]
pub type LineNumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - START_INT_ENABLE"]
    #[inline(always)]
    pub fn start_int_enable(&self) -> StartIntEnableR {
        StartIntEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FINISH_INT_ENABLE"]
    #[inline(always)]
    pub fn finish_int_enable(&self) -> FinishIntEnableR {
        FinishIntEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AI_OUTPUT_ENABLE"]
    #[inline(always)]
    pub fn ai_output_enable(&self) -> AiOutputEnableR {
        AiOutputEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DISPLAY_OUTPUT_ENABLE"]
    #[inline(always)]
    pub fn display_output_enable(&self) -> DisplayOutputEnableR {
        DisplayOutputEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUTO_ENABLE"]
    #[inline(always)]
    pub fn auto_enable(&self) -> AutoEnableR {
        AutoEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - BURST_SIZE_4BEATS"]
    #[inline(always)]
    pub fn burst_size_4beats(&self) -> BurstSize4beatsR {
        BurstSize4beatsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - FORMAT"]
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:19 - HREF_BURST_NUM"]
    #[inline(always)]
    pub fn href_burst_num(&self) -> HrefBurstNumR {
        HrefBurstNumR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:29 - LINE_NUM"]
    #[inline(always)]
    pub fn line_num(&self) -> LineNumR {
        LineNumR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - START_INT_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn start_int_enable(&mut self) -> StartIntEnableW<DvpCfgSpec> {
        StartIntEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - FINISH_INT_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn finish_int_enable(&mut self) -> FinishIntEnableW<DvpCfgSpec> {
        FinishIntEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - AI_OUTPUT_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn ai_output_enable(&mut self) -> AiOutputEnableW<DvpCfgSpec> {
        AiOutputEnableW::new(self, 2)
    }
    #[doc = "Bit 3 - DISPLAY_OUTPUT_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn display_output_enable(&mut self) -> DisplayOutputEnableW<DvpCfgSpec> {
        DisplayOutputEnableW::new(self, 3)
    }
    #[doc = "Bit 4 - AUTO_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn auto_enable(&mut self) -> AutoEnableW<DvpCfgSpec> {
        AutoEnableW::new(self, 4)
    }
    #[doc = "Bit 8 - BURST_SIZE_4BEATS"]
    #[inline(always)]
    #[must_use]
    pub fn burst_size_4beats(&mut self) -> BurstSize4beatsW<DvpCfgSpec> {
        BurstSize4beatsW::new(self, 8)
    }
    #[doc = "Bits 9:10 - FORMAT"]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FormatW<DvpCfgSpec> {
        FormatW::new(self, 9)
    }
    #[doc = "Bits 12:19 - HREF_BURST_NUM"]
    #[inline(always)]
    #[must_use]
    pub fn href_burst_num(&mut self) -> HrefBurstNumW<DvpCfgSpec> {
        HrefBurstNumW::new(self, 12)
    }
    #[doc = "Bits 20:29 - LINE_NUM"]
    #[inline(always)]
    #[must_use]
    pub fn line_num(&mut self) -> LineNumW<DvpCfgSpec> {
        LineNumW::new(self, 20)
    }
}
#[doc = "Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvpCfgSpec;
impl crate::RegisterSpec for DvpCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp_cfg::R`](R) reader structure"]
impl crate::Readable for DvpCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp_cfg::W`](W) writer structure"]
impl crate::Writable for DvpCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dvp_cfg to value 0"]
impl crate::Resettable for DvpCfgSpec {
    const RESET_VALUE: u32 = 0;
}
