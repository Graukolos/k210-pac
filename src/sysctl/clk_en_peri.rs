#[doc = "Register `clk_en_peri` reader"]
pub type R = crate::R<ClkEnPeriSpec>;
#[doc = "Register `clk_en_peri` writer"]
pub type W = crate::W<ClkEnPeriSpec>;
#[doc = "Field `rom_clk_en` reader - "]
pub type RomClkEnR = crate::BitReader;
#[doc = "Field `rom_clk_en` writer - "]
pub type RomClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_clk_en` reader - "]
pub type DmaClkEnR = crate::BitReader;
#[doc = "Field `dma_clk_en` writer - "]
pub type DmaClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ai_clk_en` reader - "]
pub type AiClkEnR = crate::BitReader;
#[doc = "Field `ai_clk_en` writer - "]
pub type AiClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dvp_clk_en` reader - "]
pub type DvpClkEnR = crate::BitReader;
#[doc = "Field `dvp_clk_en` writer - "]
pub type DvpClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fft_clk_en` reader - "]
pub type FftClkEnR = crate::BitReader;
#[doc = "Field `fft_clk_en` writer - "]
pub type FftClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_clk_en` reader - "]
pub type GpioClkEnR = crate::BitReader;
#[doc = "Field `gpio_clk_en` writer - "]
pub type GpioClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi0_clk_en` reader - "]
pub type Spi0ClkEnR = crate::BitReader;
#[doc = "Field `spi0_clk_en` writer - "]
pub type Spi0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi1_clk_en` reader - "]
pub type Spi1ClkEnR = crate::BitReader;
#[doc = "Field `spi1_clk_en` writer - "]
pub type Spi1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi2_clk_en` reader - "]
pub type Spi2ClkEnR = crate::BitReader;
#[doc = "Field `spi2_clk_en` writer - "]
pub type Spi2ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi3_clk_en` reader - "]
pub type Spi3ClkEnR = crate::BitReader;
#[doc = "Field `spi3_clk_en` writer - "]
pub type Spi3ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s0_clk_en` reader - "]
pub type I2s0ClkEnR = crate::BitReader;
#[doc = "Field `i2s0_clk_en` writer - "]
pub type I2s0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s1_clk_en` reader - "]
pub type I2s1ClkEnR = crate::BitReader;
#[doc = "Field `i2s1_clk_en` writer - "]
pub type I2s1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s2_clk_en` reader - "]
pub type I2s2ClkEnR = crate::BitReader;
#[doc = "Field `i2s2_clk_en` writer - "]
pub type I2s2ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_clk_en` reader - "]
pub type I2c0ClkEnR = crate::BitReader;
#[doc = "Field `i2c0_clk_en` writer - "]
pub type I2c0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_clk_en` reader - "]
pub type I2c1ClkEnR = crate::BitReader;
#[doc = "Field `i2c1_clk_en` writer - "]
pub type I2c1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_clk_en` reader - "]
pub type I2c2ClkEnR = crate::BitReader;
#[doc = "Field `i2c2_clk_en` writer - "]
pub type I2c2ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart1_clk_en` reader - "]
pub type Uart1ClkEnR = crate::BitReader;
#[doc = "Field `uart1_clk_en` writer - "]
pub type Uart1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart2_clk_en` reader - "]
pub type Uart2ClkEnR = crate::BitReader;
#[doc = "Field `uart2_clk_en` writer - "]
pub type Uart2ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart3_clk_en` reader - "]
pub type Uart3ClkEnR = crate::BitReader;
#[doc = "Field `uart3_clk_en` writer - "]
pub type Uart3ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aes_clk_en` reader - "]
pub type AesClkEnR = crate::BitReader;
#[doc = "Field `aes_clk_en` writer - "]
pub type AesClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fpioa_clk_en` reader - "]
pub type FpioaClkEnR = crate::BitReader;
#[doc = "Field `fpioa_clk_en` writer - "]
pub type FpioaClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer0_clk_en` reader - "]
pub type Timer0ClkEnR = crate::BitReader;
#[doc = "Field `timer0_clk_en` writer - "]
pub type Timer0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer1_clk_en` reader - "]
pub type Timer1ClkEnR = crate::BitReader;
#[doc = "Field `timer1_clk_en` writer - "]
pub type Timer1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer2_clk_en` reader - "]
pub type Timer2ClkEnR = crate::BitReader;
#[doc = "Field `timer2_clk_en` writer - "]
pub type Timer2ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdt0_clk_en` reader - "]
pub type Wdt0ClkEnR = crate::BitReader;
#[doc = "Field `wdt0_clk_en` writer - "]
pub type Wdt0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdt1_clk_en` reader - "]
pub type Wdt1ClkEnR = crate::BitReader;
#[doc = "Field `wdt1_clk_en` writer - "]
pub type Wdt1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sha_clk_en` reader - "]
pub type ShaClkEnR = crate::BitReader;
#[doc = "Field `sha_clk_en` writer - "]
pub type ShaClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `otp_clk_en` reader - "]
pub type OtpClkEnR = crate::BitReader;
#[doc = "Field `otp_clk_en` writer - "]
pub type OtpClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtc_clk_en` reader - "]
pub type RtcClkEnR = crate::BitReader;
#[doc = "Field `rtc_clk_en` writer - "]
pub type RtcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rom_clk_en(&self) -> RomClkEnR {
        RomClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DmaClkEnR {
        DmaClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ai_clk_en(&self) -> AiClkEnR {
        AiClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dvp_clk_en(&self) -> DvpClkEnR {
        DvpClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fft_clk_en(&self) -> FftClkEnR {
        FftClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_clk_en(&self) -> GpioClkEnR {
        GpioClkEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi0_clk_en(&self) -> Spi0ClkEnR {
        Spi0ClkEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi1_clk_en(&self) -> Spi1ClkEnR {
        Spi1ClkEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi2_clk_en(&self) -> Spi2ClkEnR {
        Spi2ClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi3_clk_en(&self) -> Spi3ClkEnR {
        Spi3ClkEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s0_clk_en(&self) -> I2s0ClkEnR {
        I2s0ClkEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s1_clk_en(&self) -> I2s1ClkEnR {
        I2s1ClkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s2_clk_en(&self) -> I2s2ClkEnR {
        I2s2ClkEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c0_clk_en(&self) -> I2c0ClkEnR {
        I2c0ClkEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2c1_clk_en(&self) -> I2c1ClkEnR {
        I2c1ClkEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2c2_clk_en(&self) -> I2c2ClkEnR {
        I2c2ClkEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> Uart1ClkEnR {
        Uart1ClkEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> Uart2ClkEnR {
        Uart2ClkEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart3_clk_en(&self) -> Uart3ClkEnR {
        Uart3ClkEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn aes_clk_en(&self) -> AesClkEnR {
        AesClkEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fpioa_clk_en(&self) -> FpioaClkEnR {
        FpioaClkEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn timer0_clk_en(&self) -> Timer0ClkEnR {
        Timer0ClkEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn timer1_clk_en(&self) -> Timer1ClkEnR {
        Timer1ClkEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn timer2_clk_en(&self) -> Timer2ClkEnR {
        Timer2ClkEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wdt0_clk_en(&self) -> Wdt0ClkEnR {
        Wdt0ClkEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wdt1_clk_en(&self) -> Wdt1ClkEnR {
        Wdt1ClkEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sha_clk_en(&self) -> ShaClkEnR {
        ShaClkEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn otp_clk_en(&self) -> OtpClkEnR {
        OtpClkEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_clk_en(&self) -> RtcClkEnR {
        RtcClkEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rom_clk_en(&mut self) -> RomClkEnW<ClkEnPeriSpec> {
        RomClkEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_clk_en(&mut self) -> DmaClkEnW<ClkEnPeriSpec> {
        DmaClkEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ai_clk_en(&mut self) -> AiClkEnW<ClkEnPeriSpec> {
        AiClkEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dvp_clk_en(&mut self) -> DvpClkEnW<ClkEnPeriSpec> {
        DvpClkEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fft_clk_en(&mut self) -> FftClkEnW<ClkEnPeriSpec> {
        FftClkEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_clk_en(&mut self) -> GpioClkEnW<ClkEnPeriSpec> {
        GpioClkEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn spi0_clk_en(&mut self) -> Spi0ClkEnW<ClkEnPeriSpec> {
        Spi0ClkEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_clk_en(&mut self) -> Spi1ClkEnW<ClkEnPeriSpec> {
        Spi1ClkEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_clk_en(&mut self) -> Spi2ClkEnW<ClkEnPeriSpec> {
        Spi2ClkEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_clk_en(&mut self) -> Spi3ClkEnW<ClkEnPeriSpec> {
        Spi3ClkEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_clk_en(&mut self) -> I2s0ClkEnW<ClkEnPeriSpec> {
        I2s0ClkEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_clk_en(&mut self) -> I2s1ClkEnW<ClkEnPeriSpec> {
        I2s1ClkEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2_clk_en(&mut self) -> I2s2ClkEnW<ClkEnPeriSpec> {
        I2s2ClkEnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_clk_en(&mut self) -> I2c0ClkEnW<ClkEnPeriSpec> {
        I2c0ClkEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_clk_en(&mut self) -> I2c1ClkEnW<ClkEnPeriSpec> {
        I2c1ClkEnW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_clk_en(&mut self) -> I2c2ClkEnW<ClkEnPeriSpec> {
        I2c2ClkEnW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_clk_en(&mut self) -> Uart1ClkEnW<ClkEnPeriSpec> {
        Uart1ClkEnW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_clk_en(&mut self) -> Uart2ClkEnW<ClkEnPeriSpec> {
        Uart2ClkEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn uart3_clk_en(&mut self) -> Uart3ClkEnW<ClkEnPeriSpec> {
        Uart3ClkEnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn aes_clk_en(&mut self) -> AesClkEnW<ClkEnPeriSpec> {
        AesClkEnW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn fpioa_clk_en(&mut self) -> FpioaClkEnW<ClkEnPeriSpec> {
        FpioaClkEnW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_clk_en(&mut self) -> Timer0ClkEnW<ClkEnPeriSpec> {
        Timer0ClkEnW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_clk_en(&mut self) -> Timer1ClkEnW<ClkEnPeriSpec> {
        Timer1ClkEnW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_clk_en(&mut self) -> Timer2ClkEnW<ClkEnPeriSpec> {
        Timer2ClkEnW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn wdt0_clk_en(&mut self) -> Wdt0ClkEnW<ClkEnPeriSpec> {
        Wdt0ClkEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn wdt1_clk_en(&mut self) -> Wdt1ClkEnW<ClkEnPeriSpec> {
        Wdt1ClkEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn sha_clk_en(&mut self) -> ShaClkEnW<ClkEnPeriSpec> {
        ShaClkEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn otp_clk_en(&mut self) -> OtpClkEnW<ClkEnPeriSpec> {
        OtpClkEnW::new(self, 27)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_clk_en(&mut self) -> RtcClkEnW<ClkEnPeriSpec> {
        RtcClkEnW::new(self, 29)
    }
}
#[doc = "Peripheral clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en_peri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en_peri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkEnPeriSpec;
impl crate::RegisterSpec for ClkEnPeriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_en_peri::R`](R) reader structure"]
impl crate::Readable for ClkEnPeriSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_en_peri::W`](W) writer structure"]
impl crate::Writable for ClkEnPeriSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_en_peri to value 0"]
impl crate::Resettable for ClkEnPeriSpec {
    const RESET_VALUE: u32 = 0;
}
