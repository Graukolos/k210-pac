#[doc = "Register `peri_reset` reader"]
pub type R = crate::R<PeriResetSpec>;
#[doc = "Register `peri_reset` writer"]
pub type W = crate::W<PeriResetSpec>;
#[doc = "Field `rom_reset` reader - "]
pub type RomResetR = crate::BitReader;
#[doc = "Field `rom_reset` writer - "]
pub type RomResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_reset` reader - "]
pub type DmaResetR = crate::BitReader;
#[doc = "Field `dma_reset` writer - "]
pub type DmaResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ai_reset` reader - "]
pub type AiResetR = crate::BitReader;
#[doc = "Field `ai_reset` writer - "]
pub type AiResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dvp_reset` reader - "]
pub type DvpResetR = crate::BitReader;
#[doc = "Field `dvp_reset` writer - "]
pub type DvpResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fft_reset` reader - "]
pub type FftResetR = crate::BitReader;
#[doc = "Field `fft_reset` writer - "]
pub type FftResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_reset` reader - "]
pub type GpioResetR = crate::BitReader;
#[doc = "Field `gpio_reset` writer - "]
pub type GpioResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi0_reset` reader - "]
pub type Spi0ResetR = crate::BitReader;
#[doc = "Field `spi0_reset` writer - "]
pub type Spi0ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi1_reset` reader - "]
pub type Spi1ResetR = crate::BitReader;
#[doc = "Field `spi1_reset` writer - "]
pub type Spi1ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi2_reset` reader - "]
pub type Spi2ResetR = crate::BitReader;
#[doc = "Field `spi2_reset` writer - "]
pub type Spi2ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi3_reset` reader - "]
pub type Spi3ResetR = crate::BitReader;
#[doc = "Field `spi3_reset` writer - "]
pub type Spi3ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s0_reset` reader - "]
pub type I2s0ResetR = crate::BitReader;
#[doc = "Field `i2s0_reset` writer - "]
pub type I2s0ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s1_reset` reader - "]
pub type I2s1ResetR = crate::BitReader;
#[doc = "Field `i2s1_reset` writer - "]
pub type I2s1ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s2_reset` reader - "]
pub type I2s2ResetR = crate::BitReader;
#[doc = "Field `i2s2_reset` writer - "]
pub type I2s2ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_reset` reader - "]
pub type I2c0ResetR = crate::BitReader;
#[doc = "Field `i2c0_reset` writer - "]
pub type I2c0ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_reset` reader - "]
pub type I2c1ResetR = crate::BitReader;
#[doc = "Field `i2c1_reset` writer - "]
pub type I2c1ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_reset` reader - "]
pub type I2c2ResetR = crate::BitReader;
#[doc = "Field `i2c2_reset` writer - "]
pub type I2c2ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart1_reset` reader - "]
pub type Uart1ResetR = crate::BitReader;
#[doc = "Field `uart1_reset` writer - "]
pub type Uart1ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart2_reset` reader - "]
pub type Uart2ResetR = crate::BitReader;
#[doc = "Field `uart2_reset` writer - "]
pub type Uart2ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart3_reset` reader - "]
pub type Uart3ResetR = crate::BitReader;
#[doc = "Field `uart3_reset` writer - "]
pub type Uart3ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aes_reset` reader - "]
pub type AesResetR = crate::BitReader;
#[doc = "Field `aes_reset` writer - "]
pub type AesResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fpioa_reset` reader - "]
pub type FpioaResetR = crate::BitReader;
#[doc = "Field `fpioa_reset` writer - "]
pub type FpioaResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer0_reset` reader - "]
pub type Timer0ResetR = crate::BitReader;
#[doc = "Field `timer0_reset` writer - "]
pub type Timer0ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer1_reset` reader - "]
pub type Timer1ResetR = crate::BitReader;
#[doc = "Field `timer1_reset` writer - "]
pub type Timer1ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer2_reset` reader - "]
pub type Timer2ResetR = crate::BitReader;
#[doc = "Field `timer2_reset` writer - "]
pub type Timer2ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdt0_reset` reader - "]
pub type Wdt0ResetR = crate::BitReader;
#[doc = "Field `wdt0_reset` writer - "]
pub type Wdt0ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdt1_reset` reader - "]
pub type Wdt1ResetR = crate::BitReader;
#[doc = "Field `wdt1_reset` writer - "]
pub type Wdt1ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sha_reset` reader - "]
pub type ShaResetR = crate::BitReader;
#[doc = "Field `sha_reset` writer - "]
pub type ShaResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtc_reset` reader - "]
pub type RtcResetR = crate::BitReader;
#[doc = "Field `rtc_reset` writer - "]
pub type RtcResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rom_reset(&self) -> RomResetR {
        RomResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_reset(&self) -> DmaResetR {
        DmaResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ai_reset(&self) -> AiResetR {
        AiResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dvp_reset(&self) -> DvpResetR {
        DvpResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fft_reset(&self) -> FftResetR {
        FftResetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_reset(&self) -> GpioResetR {
        GpioResetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi0_reset(&self) -> Spi0ResetR {
        Spi0ResetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi1_reset(&self) -> Spi1ResetR {
        Spi1ResetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi2_reset(&self) -> Spi2ResetR {
        Spi2ResetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi3_reset(&self) -> Spi3ResetR {
        Spi3ResetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s0_reset(&self) -> I2s0ResetR {
        I2s0ResetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s1_reset(&self) -> I2s1ResetR {
        I2s1ResetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s2_reset(&self) -> I2s2ResetR {
        I2s2ResetR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c0_reset(&self) -> I2c0ResetR {
        I2c0ResetR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2c1_reset(&self) -> I2c1ResetR {
        I2c1ResetR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2c2_reset(&self) -> I2c2ResetR {
        I2c2ResetR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uart1_reset(&self) -> Uart1ResetR {
        Uart1ResetR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn uart2_reset(&self) -> Uart2ResetR {
        Uart2ResetR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart3_reset(&self) -> Uart3ResetR {
        Uart3ResetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn aes_reset(&self) -> AesResetR {
        AesResetR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fpioa_reset(&self) -> FpioaResetR {
        FpioaResetR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn timer0_reset(&self) -> Timer0ResetR {
        Timer0ResetR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn timer1_reset(&self) -> Timer1ResetR {
        Timer1ResetR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn timer2_reset(&self) -> Timer2ResetR {
        Timer2ResetR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wdt0_reset(&self) -> Wdt0ResetR {
        Wdt0ResetR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wdt1_reset(&self) -> Wdt1ResetR {
        Wdt1ResetR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sha_reset(&self) -> ShaResetR {
        ShaResetR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_reset(&self) -> RtcResetR {
        RtcResetR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rom_reset(&mut self) -> RomResetW<PeriResetSpec> {
        RomResetW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_reset(&mut self) -> DmaResetW<PeriResetSpec> {
        DmaResetW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ai_reset(&mut self) -> AiResetW<PeriResetSpec> {
        AiResetW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dvp_reset(&mut self) -> DvpResetW<PeriResetSpec> {
        DvpResetW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fft_reset(&mut self) -> FftResetW<PeriResetSpec> {
        FftResetW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_reset(&mut self) -> GpioResetW<PeriResetSpec> {
        GpioResetW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn spi0_reset(&mut self) -> Spi0ResetW<PeriResetSpec> {
        Spi0ResetW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_reset(&mut self) -> Spi1ResetW<PeriResetSpec> {
        Spi1ResetW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_reset(&mut self) -> Spi2ResetW<PeriResetSpec> {
        Spi2ResetW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_reset(&mut self) -> Spi3ResetW<PeriResetSpec> {
        Spi3ResetW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_reset(&mut self) -> I2s0ResetW<PeriResetSpec> {
        I2s0ResetW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_reset(&mut self) -> I2s1ResetW<PeriResetSpec> {
        I2s1ResetW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2_reset(&mut self) -> I2s2ResetW<PeriResetSpec> {
        I2s2ResetW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_reset(&mut self) -> I2c0ResetW<PeriResetSpec> {
        I2c0ResetW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_reset(&mut self) -> I2c1ResetW<PeriResetSpec> {
        I2c1ResetW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_reset(&mut self) -> I2c2ResetW<PeriResetSpec> {
        I2c2ResetW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_reset(&mut self) -> Uart1ResetW<PeriResetSpec> {
        Uart1ResetW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_reset(&mut self) -> Uart2ResetW<PeriResetSpec> {
        Uart2ResetW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn uart3_reset(&mut self) -> Uart3ResetW<PeriResetSpec> {
        Uart3ResetW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn aes_reset(&mut self) -> AesResetW<PeriResetSpec> {
        AesResetW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn fpioa_reset(&mut self) -> FpioaResetW<PeriResetSpec> {
        FpioaResetW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_reset(&mut self) -> Timer0ResetW<PeriResetSpec> {
        Timer0ResetW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_reset(&mut self) -> Timer1ResetW<PeriResetSpec> {
        Timer1ResetW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_reset(&mut self) -> Timer2ResetW<PeriResetSpec> {
        Timer2ResetW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn wdt0_reset(&mut self) -> Wdt0ResetW<PeriResetSpec> {
        Wdt0ResetW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn wdt1_reset(&mut self) -> Wdt1ResetW<PeriResetSpec> {
        Wdt1ResetW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn sha_reset(&mut self) -> ShaResetW<PeriResetSpec> {
        ShaResetW::new(self, 26)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_reset(&mut self) -> RtcResetW<PeriResetSpec> {
        RtcResetW::new(self, 29)
    }
}
#[doc = "Peripheral reset controller\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriResetSpec;
impl crate::RegisterSpec for PeriResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_reset::R`](R) reader structure"]
impl crate::Readable for PeriResetSpec {}
#[doc = "`write(|w| ..)` method takes [`peri_reset::W`](W) writer structure"]
impl crate::Writable for PeriResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets peri_reset to value 0"]
impl crate::Resettable for PeriResetSpec {
    const RESET_VALUE: u32 = 0;
}
