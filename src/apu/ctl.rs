#[doc = "Register `ctl` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `ctl` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `dir_search_en` reader - Sound direction searching enable bit"]
pub type DirSearchEnR = crate::BitReader;
#[doc = "Field `dir_search_en` writer - Sound direction searching enable bit"]
pub type DirSearchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `search_path_reset` reader - Reset all control logic on direction search processing path"]
pub type SearchPathResetR = crate::BitReader;
#[doc = "Field `search_path_reset` writer - Reset all control logic on direction search processing path"]
pub type SearchPathResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stream_gen_en` reader - Valid voice sample stream generation enable bit"]
pub type StreamGenEnR = crate::BitReader;
#[doc = "Field `stream_gen_en` writer - Valid voice sample stream generation enable bit"]
pub type StreamGenEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `voice_gen_path_reset` reader - Reset all control logic on voice stream generating path"]
pub type VoiceGenPathResetR = crate::BitReader;
#[doc = "Field `voice_gen_path_reset` writer - Reset all control logic on voice stream generating path"]
pub type VoiceGenPathResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `update_voice_dir` reader - Switch to a new voice source direction"]
pub type UpdateVoiceDirR = crate::BitReader;
#[doc = "Field `update_voice_dir` writer - Switch to a new voice source direction"]
pub type UpdateVoiceDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `we_dir_search_en` writer - Write enable for we_dir_search_en parameter"]
pub type WeDirSearchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `we_search_path_rst` writer - Write enable for we_search_path_rst parameter"]
pub type WeSearchPathRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `we_stream_gen` writer - Write enable for we_stream_gen parameter"]
pub type WeStreamGenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `we_voice_gen_path_rst` writer - Write enable for we_voice_gen_path_rst parameter"]
pub type WeVoiceGenPathRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `we_update_voice_dir` writer - Write enable for we_update_voice_dir parameter"]
pub type WeUpdateVoiceDirW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sound direction searching enable bit"]
    #[inline(always)]
    pub fn dir_search_en(&self) -> DirSearchEnR {
        DirSearchEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset all control logic on direction search processing path"]
    #[inline(always)]
    pub fn search_path_reset(&self) -> SearchPathResetR {
        SearchPathResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Valid voice sample stream generation enable bit"]
    #[inline(always)]
    pub fn stream_gen_en(&self) -> StreamGenEnR {
        StreamGenEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset all control logic on voice stream generating path"]
    #[inline(always)]
    pub fn voice_gen_path_reset(&self) -> VoiceGenPathResetR {
        VoiceGenPathResetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Switch to a new voice source direction"]
    #[inline(always)]
    pub fn update_voice_dir(&self) -> UpdateVoiceDirR {
        UpdateVoiceDirR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sound direction searching enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dir_search_en(&mut self) -> DirSearchEnW<CtlSpec> {
        DirSearchEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset all control logic on direction search processing path"]
    #[inline(always)]
    #[must_use]
    pub fn search_path_reset(&mut self) -> SearchPathResetW<CtlSpec> {
        SearchPathResetW::new(self, 1)
    }
    #[doc = "Bit 4 - Valid voice sample stream generation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn stream_gen_en(&mut self) -> StreamGenEnW<CtlSpec> {
        StreamGenEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset all control logic on voice stream generating path"]
    #[inline(always)]
    #[must_use]
    pub fn voice_gen_path_reset(&mut self) -> VoiceGenPathResetW<CtlSpec> {
        VoiceGenPathResetW::new(self, 5)
    }
    #[doc = "Bit 6 - Switch to a new voice source direction"]
    #[inline(always)]
    #[must_use]
    pub fn update_voice_dir(&mut self) -> UpdateVoiceDirW<CtlSpec> {
        UpdateVoiceDirW::new(self, 6)
    }
    #[doc = "Bit 8 - Write enable for we_dir_search_en parameter"]
    #[inline(always)]
    #[must_use]
    pub fn we_dir_search_en(&mut self) -> WeDirSearchEnW<CtlSpec> {
        WeDirSearchEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Write enable for we_search_path_rst parameter"]
    #[inline(always)]
    #[must_use]
    pub fn we_search_path_rst(&mut self) -> WeSearchPathRstW<CtlSpec> {
        WeSearchPathRstW::new(self, 9)
    }
    #[doc = "Bit 10 - Write enable for we_stream_gen parameter"]
    #[inline(always)]
    #[must_use]
    pub fn we_stream_gen(&mut self) -> WeStreamGenW<CtlSpec> {
        WeStreamGenW::new(self, 10)
    }
    #[doc = "Bit 11 - Write enable for we_voice_gen_path_rst parameter"]
    #[inline(always)]
    #[must_use]
    pub fn we_voice_gen_path_rst(&mut self) -> WeVoiceGenPathRstW<CtlSpec> {
        WeVoiceGenPathRstW::new(self, 11)
    }
    #[doc = "Bit 12 - Write enable for we_update_voice_dir parameter"]
    #[inline(always)]
    #[must_use]
    pub fn we_update_voice_dir(&mut self) -> WeUpdateVoiceDirW<CtlSpec> {
        WeUpdateVoiceDirW::new(self, 12)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctl to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
