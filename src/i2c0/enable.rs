#[doc = "Register `enable` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `enable` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Field `enable` reader - ENABLE"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - ENABLE"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort` reader - ABORT"]
pub type AbortR = crate::BitReader;
#[doc = "Field `abort` writer - ABORT"]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_cmd_block` reader - TX_CMD_BLOCK"]
pub type TxCmdBlockR = crate::BitReader;
#[doc = "Field `tx_cmd_block` writer - TX_CMD_BLOCK"]
pub type TxCmdBlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ABORT"]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX_CMD_BLOCK"]
    #[inline(always)]
    pub fn tx_cmd_block(&self) -> TxCmdBlockR {
        TxCmdBlockR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<EnableSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - ABORT"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> AbortW<EnableSpec> {
        AbortW::new(self, 1)
    }
    #[doc = "Bit 2 - TX_CMD_BLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn tx_cmd_block(&mut self) -> TxCmdBlockW<EnableSpec> {
        TxCmdBlockW::new(self, 2)
    }
}
#[doc = "Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets enable to value 0"]
impl crate::Resettable for EnableSpec {
    const RESET_VALUE: u32 = 0;
}
