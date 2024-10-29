#[doc = "Register `intsignal_en` reader"]
pub type R = crate::R<IntsignalEnSpec>;
#[doc = "Register `intsignal_en` writer"]
pub type W = crate::W<IntsignalEnSpec>;
#[doc = "Field `block_tfr_done` reader - Block transfer done"]
pub type BlockTfrDoneR = crate::BitReader;
#[doc = "Field `block_tfr_done` writer - Block transfer done"]
pub type BlockTfrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tfr_done` reader - Transfer done"]
pub type TfrDoneR = crate::BitReader;
#[doc = "Field `tfr_done` writer - Transfer done"]
pub type TfrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `src_transcomp` reader - Source transaction complete"]
pub type SrcTranscompR = crate::BitReader;
#[doc = "Field `src_transcomp` writer - Source transaction complete"]
pub type SrcTranscompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dst_transcomp` reader - Destination transaction complete"]
pub type DstTranscompR = crate::BitReader;
#[doc = "Field `dst_transcomp` writer - Destination transaction complete"]
pub type DstTranscompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `src_dec_err` reader - Source Decode Error"]
pub type SrcDecErrR = crate::BitReader;
#[doc = "Field `src_dec_err` writer - Source Decode Error"]
pub type SrcDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dst_dec_err` reader - Destination Decode Error"]
pub type DstDecErrR = crate::BitReader;
#[doc = "Field `dst_dec_err` writer - Destination Decode Error"]
pub type DstDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `src_slv_err` reader - Source Slave Error"]
pub type SrcSlvErrR = crate::BitReader;
#[doc = "Field `src_slv_err` writer - Source Slave Error"]
pub type SrcSlvErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dst_slv_err` reader - Destination Slave Error"]
pub type DstSlvErrR = crate::BitReader;
#[doc = "Field `dst_slv_err` writer - Destination Slave Error"]
pub type DstSlvErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lli_rd_dec_err` reader - LLI Read Decode Error Status Enable"]
pub type LliRdDecErrR = crate::BitReader;
#[doc = "Field `lli_rd_dec_err` writer - LLI Read Decode Error Status Enable"]
pub type LliRdDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lli_wr_dec_err` reader - LLI WRITE Decode Error"]
pub type LliWrDecErrR = crate::BitReader;
#[doc = "Field `lli_wr_dec_err` writer - LLI WRITE Decode Error"]
pub type LliWrDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lli_rd_slv_err` reader - LLI Read Slave Error"]
pub type LliRdSlvErrR = crate::BitReader;
#[doc = "Field `lli_rd_slv_err` writer - LLI Read Slave Error"]
pub type LliRdSlvErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lli_wr_slv_err` reader - LLI WRITE Slave Error"]
pub type LliWrSlvErrR = crate::BitReader;
#[doc = "Field `lli_wr_slv_err` writer - LLI WRITE Slave Error"]
pub type LliWrSlvErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Block transfer done"]
    #[inline(always)]
    pub fn block_tfr_done(&self) -> BlockTfrDoneR {
        BlockTfrDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer done"]
    #[inline(always)]
    pub fn tfr_done(&self) -> TfrDoneR {
        TfrDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Source transaction complete"]
    #[inline(always)]
    pub fn src_transcomp(&self) -> SrcTranscompR {
        SrcTranscompR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Destination transaction complete"]
    #[inline(always)]
    pub fn dst_transcomp(&self) -> DstTranscompR {
        DstTranscompR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Source Decode Error"]
    #[inline(always)]
    pub fn src_dec_err(&self) -> SrcDecErrR {
        SrcDecErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Destination Decode Error"]
    #[inline(always)]
    pub fn dst_dec_err(&self) -> DstDecErrR {
        DstDecErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source Slave Error"]
    #[inline(always)]
    pub fn src_slv_err(&self) -> SrcSlvErrR {
        SrcSlvErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Destination Slave Error"]
    #[inline(always)]
    pub fn dst_slv_err(&self) -> DstSlvErrR {
        DstSlvErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LLI Read Decode Error Status Enable"]
    #[inline(always)]
    pub fn lli_rd_dec_err(&self) -> LliRdDecErrR {
        LliRdDecErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LLI WRITE Decode Error"]
    #[inline(always)]
    pub fn lli_wr_dec_err(&self) -> LliWrDecErrR {
        LliWrDecErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LLI Read Slave Error"]
    #[inline(always)]
    pub fn lli_rd_slv_err(&self) -> LliRdSlvErrR {
        LliRdSlvErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LLI WRITE Slave Error"]
    #[inline(always)]
    pub fn lli_wr_slv_err(&self) -> LliWrSlvErrR {
        LliWrSlvErrR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block transfer done"]
    #[inline(always)]
    #[must_use]
    pub fn block_tfr_done(&mut self) -> BlockTfrDoneW<IntsignalEnSpec> {
        BlockTfrDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer done"]
    #[inline(always)]
    #[must_use]
    pub fn tfr_done(&mut self) -> TfrDoneW<IntsignalEnSpec> {
        TfrDoneW::new(self, 1)
    }
    #[doc = "Bit 3 - Source transaction complete"]
    #[inline(always)]
    #[must_use]
    pub fn src_transcomp(&mut self) -> SrcTranscompW<IntsignalEnSpec> {
        SrcTranscompW::new(self, 3)
    }
    #[doc = "Bit 4 - Destination transaction complete"]
    #[inline(always)]
    #[must_use]
    pub fn dst_transcomp(&mut self) -> DstTranscompW<IntsignalEnSpec> {
        DstTranscompW::new(self, 4)
    }
    #[doc = "Bit 5 - Source Decode Error"]
    #[inline(always)]
    #[must_use]
    pub fn src_dec_err(&mut self) -> SrcDecErrW<IntsignalEnSpec> {
        SrcDecErrW::new(self, 5)
    }
    #[doc = "Bit 6 - Destination Decode Error"]
    #[inline(always)]
    #[must_use]
    pub fn dst_dec_err(&mut self) -> DstDecErrW<IntsignalEnSpec> {
        DstDecErrW::new(self, 6)
    }
    #[doc = "Bit 7 - Source Slave Error"]
    #[inline(always)]
    #[must_use]
    pub fn src_slv_err(&mut self) -> SrcSlvErrW<IntsignalEnSpec> {
        SrcSlvErrW::new(self, 7)
    }
    #[doc = "Bit 8 - Destination Slave Error"]
    #[inline(always)]
    #[must_use]
    pub fn dst_slv_err(&mut self) -> DstSlvErrW<IntsignalEnSpec> {
        DstSlvErrW::new(self, 8)
    }
    #[doc = "Bit 9 - LLI Read Decode Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lli_rd_dec_err(&mut self) -> LliRdDecErrW<IntsignalEnSpec> {
        LliRdDecErrW::new(self, 9)
    }
    #[doc = "Bit 10 - LLI WRITE Decode Error"]
    #[inline(always)]
    #[must_use]
    pub fn lli_wr_dec_err(&mut self) -> LliWrDecErrW<IntsignalEnSpec> {
        LliWrDecErrW::new(self, 10)
    }
    #[doc = "Bit 11 - LLI Read Slave Error"]
    #[inline(always)]
    #[must_use]
    pub fn lli_rd_slv_err(&mut self) -> LliRdSlvErrW<IntsignalEnSpec> {
        LliRdSlvErrW::new(self, 11)
    }
    #[doc = "Bit 12 - LLI WRITE Slave Error"]
    #[inline(always)]
    #[must_use]
    pub fn lli_wr_slv_err(&mut self) -> LliWrSlvErrW<IntsignalEnSpec> {
        LliWrSlvErrW::new(self, 12)
    }
}
#[doc = "Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsignal_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsignal_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsignalEnSpec;
impl crate::RegisterSpec for IntsignalEnSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intsignal_en::R`](R) reader structure"]
impl crate::Readable for IntsignalEnSpec {}
#[doc = "`write(|w| ..)` method takes [`intsignal_en::W`](W) writer structure"]
impl crate::Writable for IntsignalEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets intsignal_en to value 0"]
impl crate::Resettable for IntsignalEnSpec {
    const RESET_VALUE: u64 = 0;
}
