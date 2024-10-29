#[doc = "Register `chen` reader"]
pub type R = crate::R<ChenSpec>;
#[doc = "Register `chen` writer"]
pub type W = crate::W<ChenSpec>;
#[doc = "Field `ch_en(1-6)` reader - Enable channel %s"]
pub type ChEnR = crate::BitReader;
#[doc = "Field `ch_en(1-6)` writer - Enable channel %s"]
pub type ChEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_en_we(1-6)` reader - Write enable channel %s"]
pub type ChEnWeR = crate::BitReader;
#[doc = "Field `ch_en_we(1-6)` writer - Write enable channel %s"]
pub type ChEnWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_susp(1-6)` reader - Suspend request channel %s"]
pub type ChSuspR = crate::BitReader;
#[doc = "Field `ch_susp(1-6)` writer - Suspend request channel %s"]
pub type ChSuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_susp_we(1-6)` reader - Enable write to ch%s_susp bit"]
pub type ChSuspWeR = crate::BitReader;
#[doc = "Field `ch_susp_we(1-6)` writer - Enable write to ch%s_susp bit"]
pub type ChSuspWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_abort(1-6)` reader - Abort request channel %s"]
pub type ChAbortR = crate::BitReader;
#[doc = "Field `ch_abort(1-6)` writer - Abort request channel %s"]
pub type ChAbortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_abort_we(1-6)` reader - Enable write to ch%s_abort bit"]
pub type ChAbortWeR = crate::BitReader;
#[doc = "Field `ch_abort_we(1-6)` writer - Enable write to ch%s_abort bit"]
pub type ChAbortWeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Enable channel (1-6)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_en` field.</div>"]
    #[inline(always)]
    pub fn ch_en(&self, n: u8) -> ChEnR {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChEnR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable channel (1-6)"]
    #[inline(always)]
    pub fn ch_en_iter(&self) -> impl Iterator<Item = ChEnR> + '_ {
        (0..6).map(move |n| ChEnR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Enable channel 1"]
    #[inline(always)]
    pub fn ch1_en(&self) -> ChEnR {
        ChEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable channel 2"]
    #[inline(always)]
    pub fn ch2_en(&self) -> ChEnR {
        ChEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable channel 3"]
    #[inline(always)]
    pub fn ch3_en(&self) -> ChEnR {
        ChEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable channel 4"]
    #[inline(always)]
    pub fn ch4_en(&self) -> ChEnR {
        ChEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable channel 5"]
    #[inline(always)]
    pub fn ch5_en(&self) -> ChEnR {
        ChEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable channel 6"]
    #[inline(always)]
    pub fn ch6_en(&self) -> ChEnR {
        ChEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Write enable channel (1-6)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_en_we` field.</div>"]
    #[inline(always)]
    pub fn ch_en_we(&self, n: u8) -> ChEnWeR {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChEnWeR::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Write enable channel (1-6)"]
    #[inline(always)]
    pub fn ch_en_we_iter(&self) -> impl Iterator<Item = ChEnWeR> + '_ {
        (0..6).map(move |n| ChEnWeR::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - Write enable channel 1"]
    #[inline(always)]
    pub fn ch1_en_we(&self) -> ChEnWeR {
        ChEnWeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write enable channel 2"]
    #[inline(always)]
    pub fn ch2_en_we(&self) -> ChEnWeR {
        ChEnWeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write enable channel 3"]
    #[inline(always)]
    pub fn ch3_en_we(&self) -> ChEnWeR {
        ChEnWeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write enable channel 4"]
    #[inline(always)]
    pub fn ch4_en_we(&self) -> ChEnWeR {
        ChEnWeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write enable channel 5"]
    #[inline(always)]
    pub fn ch5_en_we(&self) -> ChEnWeR {
        ChEnWeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write enable channel 6"]
    #[inline(always)]
    pub fn ch6_en_we(&self) -> ChEnWeR {
        ChEnWeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Suspend request channel (1-6)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_susp` field.</div>"]
    #[inline(always)]
    pub fn ch_susp(&self, n: u8) -> ChSuspR {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChSuspR::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Suspend request channel (1-6)"]
    #[inline(always)]
    pub fn ch_susp_iter(&self) -> impl Iterator<Item = ChSuspR> + '_ {
        (0..6).map(move |n| ChSuspR::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - Suspend request channel 1"]
    #[inline(always)]
    pub fn ch1_susp(&self) -> ChSuspR {
        ChSuspR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Suspend request channel 2"]
    #[inline(always)]
    pub fn ch2_susp(&self) -> ChSuspR {
        ChSuspR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Suspend request channel 3"]
    #[inline(always)]
    pub fn ch3_susp(&self) -> ChSuspR {
        ChSuspR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Suspend request channel 4"]
    #[inline(always)]
    pub fn ch4_susp(&self) -> ChSuspR {
        ChSuspR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Suspend request channel 5"]
    #[inline(always)]
    pub fn ch5_susp(&self) -> ChSuspR {
        ChSuspR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Suspend request channel 6"]
    #[inline(always)]
    pub fn ch6_susp(&self) -> ChSuspR {
        ChSuspR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Enable write to ch(1-6)_susp bit"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_susp_we` field.</div>"]
    #[inline(always)]
    pub fn ch_susp_we(&self, n: u8) -> ChSuspWeR {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChSuspWeR::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable write to ch(1-6)_susp bit"]
    #[inline(always)]
    pub fn ch_susp_we_iter(&self) -> impl Iterator<Item = ChSuspWeR> + '_ {
        (0..6).map(move |n| ChSuspWeR::new(((self.bits >> (n + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - Enable write to ch1_susp bit"]
    #[inline(always)]
    pub fn ch1_susp_we(&self) -> ChSuspWeR {
        ChSuspWeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable write to ch2_susp bit"]
    #[inline(always)]
    pub fn ch2_susp_we(&self) -> ChSuspWeR {
        ChSuspWeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable write to ch3_susp bit"]
    #[inline(always)]
    pub fn ch3_susp_we(&self) -> ChSuspWeR {
        ChSuspWeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable write to ch4_susp bit"]
    #[inline(always)]
    pub fn ch4_susp_we(&self) -> ChSuspWeR {
        ChSuspWeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable write to ch5_susp bit"]
    #[inline(always)]
    pub fn ch5_susp_we(&self) -> ChSuspWeR {
        ChSuspWeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable write to ch6_susp bit"]
    #[inline(always)]
    pub fn ch6_susp_we(&self) -> ChSuspWeR {
        ChSuspWeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Abort request channel (1-6)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_abort` field.</div>"]
    #[inline(always)]
    pub fn ch_abort(&self, n: u8) -> ChAbortR {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChAbortR::new(((self.bits >> (n + 32)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Abort request channel (1-6)"]
    #[inline(always)]
    pub fn ch_abort_iter(&self) -> impl Iterator<Item = ChAbortR> + '_ {
        (0..6).map(move |n| ChAbortR::new(((self.bits >> (n + 32)) & 1) != 0))
    }
    #[doc = "Bit 32 - Abort request channel 1"]
    #[inline(always)]
    pub fn ch1_abort(&self) -> ChAbortR {
        ChAbortR::new(((self.bits >> 32) & 1) != 0)
    }
    #[doc = "Bit 33 - Abort request channel 2"]
    #[inline(always)]
    pub fn ch2_abort(&self) -> ChAbortR {
        ChAbortR::new(((self.bits >> 33) & 1) != 0)
    }
    #[doc = "Bit 34 - Abort request channel 3"]
    #[inline(always)]
    pub fn ch3_abort(&self) -> ChAbortR {
        ChAbortR::new(((self.bits >> 34) & 1) != 0)
    }
    #[doc = "Bit 35 - Abort request channel 4"]
    #[inline(always)]
    pub fn ch4_abort(&self) -> ChAbortR {
        ChAbortR::new(((self.bits >> 35) & 1) != 0)
    }
    #[doc = "Bit 36 - Abort request channel 5"]
    #[inline(always)]
    pub fn ch5_abort(&self) -> ChAbortR {
        ChAbortR::new(((self.bits >> 36) & 1) != 0)
    }
    #[doc = "Bit 37 - Abort request channel 6"]
    #[inline(always)]
    pub fn ch6_abort(&self) -> ChAbortR {
        ChAbortR::new(((self.bits >> 37) & 1) != 0)
    }
    #[doc = "Enable write to ch(1-6)_abort bit"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_abort_we` field.</div>"]
    #[inline(always)]
    pub fn ch_abort_we(&self, n: u8) -> ChAbortWeR {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChAbortWeR::new(((self.bits >> (n + 40)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable write to ch(1-6)_abort bit"]
    #[inline(always)]
    pub fn ch_abort_we_iter(&self) -> impl Iterator<Item = ChAbortWeR> + '_ {
        (0..6).map(move |n| ChAbortWeR::new(((self.bits >> (n + 40)) & 1) != 0))
    }
    #[doc = "Bit 40 - Enable write to ch1_abort bit"]
    #[inline(always)]
    pub fn ch1_abort_we(&self) -> ChAbortWeR {
        ChAbortWeR::new(((self.bits >> 40) & 1) != 0)
    }
    #[doc = "Bit 41 - Enable write to ch2_abort bit"]
    #[inline(always)]
    pub fn ch2_abort_we(&self) -> ChAbortWeR {
        ChAbortWeR::new(((self.bits >> 41) & 1) != 0)
    }
    #[doc = "Bit 42 - Enable write to ch3_abort bit"]
    #[inline(always)]
    pub fn ch3_abort_we(&self) -> ChAbortWeR {
        ChAbortWeR::new(((self.bits >> 42) & 1) != 0)
    }
    #[doc = "Bit 43 - Enable write to ch4_abort bit"]
    #[inline(always)]
    pub fn ch4_abort_we(&self) -> ChAbortWeR {
        ChAbortWeR::new(((self.bits >> 43) & 1) != 0)
    }
    #[doc = "Bit 44 - Enable write to ch5_abort bit"]
    #[inline(always)]
    pub fn ch5_abort_we(&self) -> ChAbortWeR {
        ChAbortWeR::new(((self.bits >> 44) & 1) != 0)
    }
    #[doc = "Bit 45 - Enable write to ch6_abort bit"]
    #[inline(always)]
    pub fn ch6_abort_we(&self) -> ChAbortWeR {
        ChAbortWeR::new(((self.bits >> 45) & 1) != 0)
    }
}
impl W {
    #[doc = "Enable channel (1-6)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_en` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn ch_en(&mut self, n: u8) -> ChEnW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChEnW::new(self, n)
    }
    #[doc = "Bit 0 - Enable channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_en(&mut self) -> ChEnW<ChenSpec> {
        ChEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_en(&mut self) -> ChEnW<ChenSpec> {
        ChEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_en(&mut self) -> ChEnW<ChenSpec> {
        ChEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_en(&mut self) -> ChEnW<ChenSpec> {
        ChEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_en(&mut self) -> ChEnW<ChenSpec> {
        ChEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_en(&mut self) -> ChEnW<ChenSpec> {
        ChEnW::new(self, 5)
    }
    #[doc = "Write enable channel (1-6)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_en_we` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn ch_en_we(&mut self, n: u8) -> ChEnWeW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChEnWeW::new(self, n + 8)
    }
    #[doc = "Bit 8 - Write enable channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_en_we(&mut self) -> ChEnWeW<ChenSpec> {
        ChEnWeW::new(self, 8)
    }
    #[doc = "Bit 9 - Write enable channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_en_we(&mut self) -> ChEnWeW<ChenSpec> {
        ChEnWeW::new(self, 9)
    }
    #[doc = "Bit 10 - Write enable channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_en_we(&mut self) -> ChEnWeW<ChenSpec> {
        ChEnWeW::new(self, 10)
    }
    #[doc = "Bit 11 - Write enable channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_en_we(&mut self) -> ChEnWeW<ChenSpec> {
        ChEnWeW::new(self, 11)
    }
    #[doc = "Bit 12 - Write enable channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_en_we(&mut self) -> ChEnWeW<ChenSpec> {
        ChEnWeW::new(self, 12)
    }
    #[doc = "Bit 13 - Write enable channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_en_we(&mut self) -> ChEnWeW<ChenSpec> {
        ChEnWeW::new(self, 13)
    }
    #[doc = "Suspend request channel (1-6)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_susp` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn ch_susp(&mut self, n: u8) -> ChSuspW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChSuspW::new(self, n + 16)
    }
    #[doc = "Bit 16 - Suspend request channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_susp(&mut self) -> ChSuspW<ChenSpec> {
        ChSuspW::new(self, 16)
    }
    #[doc = "Bit 17 - Suspend request channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_susp(&mut self) -> ChSuspW<ChenSpec> {
        ChSuspW::new(self, 17)
    }
    #[doc = "Bit 18 - Suspend request channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_susp(&mut self) -> ChSuspW<ChenSpec> {
        ChSuspW::new(self, 18)
    }
    #[doc = "Bit 19 - Suspend request channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_susp(&mut self) -> ChSuspW<ChenSpec> {
        ChSuspW::new(self, 19)
    }
    #[doc = "Bit 20 - Suspend request channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_susp(&mut self) -> ChSuspW<ChenSpec> {
        ChSuspW::new(self, 20)
    }
    #[doc = "Bit 21 - Suspend request channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_susp(&mut self) -> ChSuspW<ChenSpec> {
        ChSuspW::new(self, 21)
    }
    #[doc = "Enable write to ch(1-6)_susp bit"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_susp_we` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn ch_susp_we(&mut self, n: u8) -> ChSuspWeW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChSuspWeW::new(self, n + 24)
    }
    #[doc = "Bit 24 - Enable write to ch1_susp bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_susp_we(&mut self) -> ChSuspWeW<ChenSpec> {
        ChSuspWeW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable write to ch2_susp bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_susp_we(&mut self) -> ChSuspWeW<ChenSpec> {
        ChSuspWeW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable write to ch3_susp bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_susp_we(&mut self) -> ChSuspWeW<ChenSpec> {
        ChSuspWeW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable write to ch4_susp bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_susp_we(&mut self) -> ChSuspWeW<ChenSpec> {
        ChSuspWeW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable write to ch5_susp bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_susp_we(&mut self) -> ChSuspWeW<ChenSpec> {
        ChSuspWeW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable write to ch6_susp bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_susp_we(&mut self) -> ChSuspWeW<ChenSpec> {
        ChSuspWeW::new(self, 29)
    }
    #[doc = "Abort request channel (1-6)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_abort` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn ch_abort(&mut self, n: u8) -> ChAbortW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChAbortW::new(self, n + 32)
    }
    #[doc = "Bit 32 - Abort request channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_abort(&mut self) -> ChAbortW<ChenSpec> {
        ChAbortW::new(self, 32)
    }
    #[doc = "Bit 33 - Abort request channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_abort(&mut self) -> ChAbortW<ChenSpec> {
        ChAbortW::new(self, 33)
    }
    #[doc = "Bit 34 - Abort request channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_abort(&mut self) -> ChAbortW<ChenSpec> {
        ChAbortW::new(self, 34)
    }
    #[doc = "Bit 35 - Abort request channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_abort(&mut self) -> ChAbortW<ChenSpec> {
        ChAbortW::new(self, 35)
    }
    #[doc = "Bit 36 - Abort request channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_abort(&mut self) -> ChAbortW<ChenSpec> {
        ChAbortW::new(self, 36)
    }
    #[doc = "Bit 37 - Abort request channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_abort(&mut self) -> ChAbortW<ChenSpec> {
        ChAbortW::new(self, 37)
    }
    #[doc = "Enable write to ch(1-6)_abort bit"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ch1_abort_we` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn ch_abort_we(&mut self, n: u8) -> ChAbortWeW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ChAbortWeW::new(self, n + 40)
    }
    #[doc = "Bit 40 - Enable write to ch1_abort bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_abort_we(&mut self) -> ChAbortWeW<ChenSpec> {
        ChAbortWeW::new(self, 40)
    }
    #[doc = "Bit 41 - Enable write to ch2_abort bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_abort_we(&mut self) -> ChAbortWeW<ChenSpec> {
        ChAbortWeW::new(self, 41)
    }
    #[doc = "Bit 42 - Enable write to ch3_abort bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_abort_we(&mut self) -> ChAbortWeW<ChenSpec> {
        ChAbortWeW::new(self, 42)
    }
    #[doc = "Bit 43 - Enable write to ch4_abort bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_abort_we(&mut self) -> ChAbortWeW<ChenSpec> {
        ChAbortWeW::new(self, 43)
    }
    #[doc = "Bit 44 - Enable write to ch5_abort bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_abort_we(&mut self) -> ChAbortWeW<ChenSpec> {
        ChAbortWeW::new(self, 44)
    }
    #[doc = "Bit 45 - Enable write to ch6_abort bit"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_abort_we(&mut self) -> ChAbortWeW<ChenSpec> {
        ChAbortWeW::new(self, 45)
    }
}
#[doc = "Channel Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenSpec;
impl crate::RegisterSpec for ChenSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`chen::R`](R) reader structure"]
impl crate::Readable for ChenSpec {}
#[doc = "`write(|w| ..)` method takes [`chen::W`](W) writer structure"]
impl crate::Writable for ChenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets chen to value 0"]
impl crate::Resettable for ChenSpec {
    const RESET_VALUE: u64 = 0;
}
