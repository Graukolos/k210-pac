#[doc = "Register `io[%s]` reader"]
pub type R = crate::R<IoSpec>;
#[doc = "Register `io[%s]` writer"]
pub type W = crate::W<IoSpec>;
#[doc = "Field `ch_sel` reader - Channel select from 256 input"]
pub type ChSelR = crate::FieldReader;
#[doc = "Field `ch_sel` writer - Channel select from 256 input"]
pub type ChSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ds` reader - Driving selector"]
pub type DsR = crate::FieldReader;
#[doc = "Field `ds` writer - Driving selector"]
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `oe_en` reader - Static output enable, will AND with OE_INV"]
pub type OeEnR = crate::BitReader;
#[doc = "Field `oe_en` writer - Static output enable, will AND with OE_INV"]
pub type OeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `oe_inv` reader - Invert output enable"]
pub type OeInvR = crate::BitReader;
#[doc = "Field `oe_inv` writer - Invert output enable"]
pub type OeInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `do_sel` reader - Data output select: 0 for DO, 1 for OE"]
pub type DoSelR = crate::BitReader;
#[doc = "Field `do_sel` writer - Data output select: 0 for DO, 1 for OE"]
pub type DoSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `do_inv` reader - Invert the result of data output select (DO_SEL)"]
pub type DoInvR = crate::BitReader;
#[doc = "Field `do_inv` writer - Invert the result of data output select (DO_SEL)"]
pub type DoInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu` reader - Pull up enable. 0 for nothing, 1 for pull up"]
pub type PuR = crate::BitReader;
#[doc = "Field `pu` writer - Pull up enable. 0 for nothing, 1 for pull up"]
pub type PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pd` reader - Pull down enable. 0 for nothing, 1 for pull down"]
pub type PdR = crate::BitReader;
#[doc = "Field `pd` writer - Pull down enable. 0 for nothing, 1 for pull down"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sl` reader - Slew rate control enable"]
pub type SlR = crate::BitReader;
#[doc = "Field `sl` writer - Slew rate control enable"]
pub type SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ie_en` reader - Static input enable, will AND with IE_INV"]
pub type IeEnR = crate::BitReader;
#[doc = "Field `ie_en` writer - Static input enable, will AND with IE_INV"]
pub type IeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ie_inv` reader - Invert input enable"]
pub type IeInvR = crate::BitReader;
#[doc = "Field `ie_inv` writer - Invert input enable"]
pub type IeInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `di_inv` reader - Invert Data input"]
pub type DiInvR = crate::BitReader;
#[doc = "Field `di_inv` writer - Invert Data input"]
pub type DiInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `st` reader - Schmitt trigger"]
pub type StR = crate::BitReader;
#[doc = "Field `st` writer - Schmitt trigger"]
pub type StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pad_di` reader - Read current IO's data input"]
pub type PadDiR = crate::BitReader;
#[doc = "Field `pad_di` writer - Read current IO's data input"]
pub type PadDiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Channel select from 256 input"]
    #[inline(always)]
    pub fn ch_sel(&self) -> ChSelR {
        ChSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Driving selector"]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Static output enable, will AND with OE_INV"]
    #[inline(always)]
    pub fn oe_en(&self) -> OeEnR {
        OeEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Invert output enable"]
    #[inline(always)]
    pub fn oe_inv(&self) -> OeInvR {
        OeInvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Data output select: 0 for DO, 1 for OE"]
    #[inline(always)]
    pub fn do_sel(&self) -> DoSelR {
        DoSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Invert the result of data output select (DO_SEL)"]
    #[inline(always)]
    pub fn do_inv(&self) -> DoInvR {
        DoInvR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pull up enable. 0 for nothing, 1 for pull up"]
    #[inline(always)]
    pub fn pu(&self) -> PuR {
        PuR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pull down enable. 0 for nothing, 1 for pull down"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Slew rate control enable"]
    #[inline(always)]
    pub fn sl(&self) -> SlR {
        SlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Static input enable, will AND with IE_INV"]
    #[inline(always)]
    pub fn ie_en(&self) -> IeEnR {
        IeEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Invert input enable"]
    #[inline(always)]
    pub fn ie_inv(&self) -> IeInvR {
        IeInvR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Invert Data input"]
    #[inline(always)]
    pub fn di_inv(&self) -> DiInvR {
        DiInvR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Schmitt trigger"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Read current IO's data input"]
    #[inline(always)]
    pub fn pad_di(&self) -> PadDiR {
        PadDiR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel select from 256 input"]
    #[inline(always)]
    #[must_use]
    pub fn ch_sel(&mut self) -> ChSelW<IoSpec> {
        ChSelW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Driving selector"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DsW<IoSpec> {
        DsW::new(self, 8)
    }
    #[doc = "Bit 12 - Static output enable, will AND with OE_INV"]
    #[inline(always)]
    #[must_use]
    pub fn oe_en(&mut self) -> OeEnW<IoSpec> {
        OeEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Invert output enable"]
    #[inline(always)]
    #[must_use]
    pub fn oe_inv(&mut self) -> OeInvW<IoSpec> {
        OeInvW::new(self, 13)
    }
    #[doc = "Bit 14 - Data output select: 0 for DO, 1 for OE"]
    #[inline(always)]
    #[must_use]
    pub fn do_sel(&mut self) -> DoSelW<IoSpec> {
        DoSelW::new(self, 14)
    }
    #[doc = "Bit 15 - Invert the result of data output select (DO_SEL)"]
    #[inline(always)]
    #[must_use]
    pub fn do_inv(&mut self) -> DoInvW<IoSpec> {
        DoInvW::new(self, 15)
    }
    #[doc = "Bit 16 - Pull up enable. 0 for nothing, 1 for pull up"]
    #[inline(always)]
    #[must_use]
    pub fn pu(&mut self) -> PuW<IoSpec> {
        PuW::new(self, 16)
    }
    #[doc = "Bit 17 - Pull down enable. 0 for nothing, 1 for pull down"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PdW<IoSpec> {
        PdW::new(self, 17)
    }
    #[doc = "Bit 19 - Slew rate control enable"]
    #[inline(always)]
    #[must_use]
    pub fn sl(&mut self) -> SlW<IoSpec> {
        SlW::new(self, 19)
    }
    #[doc = "Bit 20 - Static input enable, will AND with IE_INV"]
    #[inline(always)]
    #[must_use]
    pub fn ie_en(&mut self) -> IeEnW<IoSpec> {
        IeEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Invert input enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie_inv(&mut self) -> IeInvW<IoSpec> {
        IeInvW::new(self, 21)
    }
    #[doc = "Bit 22 - Invert Data input"]
    #[inline(always)]
    #[must_use]
    pub fn di_inv(&mut self) -> DiInvW<IoSpec> {
        DiInvW::new(self, 22)
    }
    #[doc = "Bit 23 - Schmitt trigger"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> StW<IoSpec> {
        StW::new(self, 23)
    }
    #[doc = "Bit 31 - Read current IO's data input"]
    #[inline(always)]
    #[must_use]
    pub fn pad_di(&mut self) -> PadDiW<IoSpec> {
        PadDiW::new(self, 31)
    }
}
#[doc = "FPIOA GPIO multiplexer io array\n\nYou can [`read`](crate::Reg::read) this register and get [`io::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoSpec;
impl crate::RegisterSpec for IoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io::R`](R) reader structure"]
impl crate::Readable for IoSpec {}
#[doc = "`write(|w| ..)` method takes [`io::W`](W) writer structure"]
impl crate::Writable for IoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets io[%s]
to value 0"]
impl crate::Resettable for IoSpec {
    const RESET_VALUE: u32 = 0;
}
