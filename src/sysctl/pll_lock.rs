#[doc = "Register `pll_lock` reader"]
pub type R = crate::R<PllLockSpec>;
#[doc = "Register `pll_lock` writer"]
pub type W = crate::W<PllLockSpec>;
#[doc = "Field `pll_lock0` reader - "]
pub type PllLock0R = crate::FieldReader;
#[doc = "Field `pll_lock0` writer - "]
pub type PllLock0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll_slip_clear0` reader - "]
pub type PllSlipClear0R = crate::BitReader;
#[doc = "Field `pll_slip_clear0` writer - "]
pub type PllSlipClear0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_clk_out0` reader - "]
pub type TestClkOut0R = crate::BitReader;
#[doc = "Field `test_clk_out0` writer - "]
pub type TestClkOut0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll_lock1` reader - "]
pub type PllLock1R = crate::FieldReader;
#[doc = "Field `pll_lock1` writer - "]
pub type PllLock1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll_slip_clear1` reader - "]
pub type PllSlipClear1R = crate::BitReader;
#[doc = "Field `pll_slip_clear1` writer - "]
pub type PllSlipClear1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_clk_out1` reader - "]
pub type TestClkOut1R = crate::BitReader;
#[doc = "Field `test_clk_out1` writer - "]
pub type TestClkOut1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll_lock2` reader - "]
pub type PllLock2R = crate::FieldReader;
#[doc = "Field `pll_lock2` writer - "]
pub type PllLock2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll_slip_clear2` reader - "]
pub type PllSlipClear2R = crate::BitReader;
#[doc = "Field `pll_slip_clear2` writer - "]
pub type PllSlipClear2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_clk_out2` reader - "]
pub type TestClkOut2R = crate::BitReader;
#[doc = "Field `test_clk_out2` writer - "]
pub type TestClkOut2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pll_lock0(&self) -> PllLock0R {
        PllLock0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pll_slip_clear0(&self) -> PllSlipClear0R {
        PllSlipClear0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn test_clk_out0(&self) -> TestClkOut0R {
        TestClkOut0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pll_lock1(&self) -> PllLock1R {
        PllLock1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pll_slip_clear1(&self) -> PllSlipClear1R {
        PllSlipClear1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn test_clk_out1(&self) -> TestClkOut1R {
        TestClkOut1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pll_lock2(&self) -> PllLock2R {
        PllLock2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pll_slip_clear2(&self) -> PllSlipClear2R {
        PllSlipClear2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn test_clk_out2(&self) -> TestClkOut2R {
        TestClkOut2R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock0(&mut self) -> PllLock0W<PllLockSpec> {
        PllLock0W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pll_slip_clear0(&mut self) -> PllSlipClear0W<PllLockSpec> {
        PllSlipClear0W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn test_clk_out0(&mut self) -> TestClkOut0W<PllLockSpec> {
        TestClkOut0W::new(self, 3)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock1(&mut self) -> PllLock1W<PllLockSpec> {
        PllLock1W::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pll_slip_clear1(&mut self) -> PllSlipClear1W<PllLockSpec> {
        PllSlipClear1W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn test_clk_out1(&mut self) -> TestClkOut1W<PllLockSpec> {
        TestClkOut1W::new(self, 11)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock2(&mut self) -> PllLock2W<PllLockSpec> {
        PllLock2W::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pll_slip_clear2(&mut self) -> PllSlipClear2W<PllLockSpec> {
        PllSlipClear2W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn test_clk_out2(&mut self) -> TestClkOut2W<PllLockSpec> {
        TestClkOut2W::new(self, 19)
    }
}
#[doc = "PLL lock tester\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllLockSpec;
impl crate::RegisterSpec for PllLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_lock::R`](R) reader structure"]
impl crate::Readable for PllLockSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_lock::W`](W) writer structure"]
impl crate::Writable for PllLockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pll_lock to value 0"]
impl crate::Resettable for PllLockSpec {
    const RESET_VALUE: u32 = 0;
}
