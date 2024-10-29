#[doc = "Register `ch_cfg` reader"]
pub type R = crate::R<ChCfgSpec>;
#[doc = "Register `ch_cfg` writer"]
pub type W = crate::W<ChCfgSpec>;
#[doc = "Field `sound_ch_en` reader - BF unit sound channel enable control bits"]
pub type SoundChEnR = crate::FieldReader;
#[doc = "Field `sound_ch_en` writer - BF unit sound channel enable control bits"]
pub type SoundChEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `target_dir` reader - Target direction select for valid voice output"]
pub type TargetDirR = crate::FieldReader;
#[doc = "Field `target_dir` writer - Target direction select for valid voice output"]
pub type TargetDirW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `audio_gain` reader - Audio sample gain factor"]
pub type AudioGainR = crate::FieldReader<u16>;
#[doc = "Field `audio_gain` writer - Audio sample gain factor"]
pub type AudioGainW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `data_src_mode` reader - Audio data source configure parameter"]
pub type DataSrcModeR = crate::BitReader;
#[doc = "Field `data_src_mode` writer - Audio data source configure parameter"]
pub type DataSrcModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `we_sound_ch_en` writer - Write enable for sound_ch_en parameter"]
pub type WeSoundChEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `we_target_dir` writer - Write enable for target_dir parameter"]
pub type WeTargetDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `we_audio_gain` writer - Write enable for audio_gain parameter"]
pub type WeAudioGainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `we_data_src_mode` writer - Write enable for data_out_mode parameter"]
pub type WeDataSrcModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - BF unit sound channel enable control bits"]
    #[inline(always)]
    pub fn sound_ch_en(&self) -> SoundChEnR {
        SoundChEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Target direction select for valid voice output"]
    #[inline(always)]
    pub fn target_dir(&self) -> TargetDirR {
        TargetDirR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:22 - Audio sample gain factor"]
    #[inline(always)]
    pub fn audio_gain(&self) -> AudioGainR {
        AudioGainR::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bit 24 - Audio data source configure parameter"]
    #[inline(always)]
    pub fn data_src_mode(&self) -> DataSrcModeR {
        DataSrcModeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - BF unit sound channel enable control bits"]
    #[inline(always)]
    #[must_use]
    pub fn sound_ch_en(&mut self) -> SoundChEnW<ChCfgSpec> {
        SoundChEnW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Target direction select for valid voice output"]
    #[inline(always)]
    #[must_use]
    pub fn target_dir(&mut self) -> TargetDirW<ChCfgSpec> {
        TargetDirW::new(self, 8)
    }
    #[doc = "Bits 12:22 - Audio sample gain factor"]
    #[inline(always)]
    #[must_use]
    pub fn audio_gain(&mut self) -> AudioGainW<ChCfgSpec> {
        AudioGainW::new(self, 12)
    }
    #[doc = "Bit 24 - Audio data source configure parameter"]
    #[inline(always)]
    #[must_use]
    pub fn data_src_mode(&mut self) -> DataSrcModeW<ChCfgSpec> {
        DataSrcModeW::new(self, 24)
    }
    #[doc = "Bit 28 - Write enable for sound_ch_en parameter"]
    #[inline(always)]
    #[must_use]
    pub fn we_sound_ch_en(&mut self) -> WeSoundChEnW<ChCfgSpec> {
        WeSoundChEnW::new(self, 28)
    }
    #[doc = "Bit 29 - Write enable for target_dir parameter"]
    #[inline(always)]
    #[must_use]
    pub fn we_target_dir(&mut self) -> WeTargetDirW<ChCfgSpec> {
        WeTargetDirW::new(self, 29)
    }
    #[doc = "Bit 30 - Write enable for audio_gain parameter"]
    #[inline(always)]
    #[must_use]
    pub fn we_audio_gain(&mut self) -> WeAudioGainW<ChCfgSpec> {
        WeAudioGainW::new(self, 30)
    }
    #[doc = "Bit 31 - Write enable for data_out_mode parameter"]
    #[inline(always)]
    #[must_use]
    pub fn we_data_src_mode(&mut self) -> WeDataSrcModeW<ChCfgSpec> {
        WeDataSrcModeW::new(self, 31)
    }
}
#[doc = "Channel Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChCfgSpec;
impl crate::RegisterSpec for ChCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_cfg::R`](R) reader structure"]
impl crate::Readable for ChCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ch_cfg::W`](W) writer structure"]
impl crate::Writable for ChCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ch_cfg to value 0"]
impl crate::Resettable for ChCfgSpec {
    const RESET_VALUE: u32 = 0;
}
