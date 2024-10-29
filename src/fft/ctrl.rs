#[doc = "Register `ctrl` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `ctrl` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "FFT calculation data length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Point {
    #[doc = "0: 512 point"]
    P512 = 0,
    #[doc = "1: 256 point"]
    P256 = 1,
    #[doc = "2: 128 point"]
    P128 = 2,
    #[doc = "3: 64 point"]
    P64 = 3,
}
impl From<Point> for u8 {
    #[inline(always)]
    fn from(variant: Point) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Point {
    type Ux = u8;
}
impl crate::IsEnum for Point {}
#[doc = "Field `point` reader - FFT calculation data length"]
pub type PointR = crate::FieldReader<Point>;
impl PointR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Point> {
        match self.bits {
            0 => Some(Point::P512),
            1 => Some(Point::P256),
            2 => Some(Point::P128),
            3 => Some(Point::P64),
            _ => None,
        }
    }
    #[doc = "512 point"]
    #[inline(always)]
    pub fn is_p512(&self) -> bool {
        *self == Point::P512
    }
    #[doc = "256 point"]
    #[inline(always)]
    pub fn is_p256(&self) -> bool {
        *self == Point::P256
    }
    #[doc = "128 point"]
    #[inline(always)]
    pub fn is_p128(&self) -> bool {
        *self == Point::P128
    }
    #[doc = "64 point"]
    #[inline(always)]
    pub fn is_p64(&self) -> bool {
        *self == Point::P64
    }
}
#[doc = "Field `point` writer - FFT calculation data length"]
pub type PointW<'a, REG> = crate::FieldWriter<'a, REG, 3, Point>;
impl<'a, REG> PointW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "512 point"]
    #[inline(always)]
    pub fn p512(self) -> &'a mut crate::W<REG> {
        self.variant(Point::P512)
    }
    #[doc = "256 point"]
    #[inline(always)]
    pub fn p256(self) -> &'a mut crate::W<REG> {
        self.variant(Point::P256)
    }
    #[doc = "128 point"]
    #[inline(always)]
    pub fn p128(self) -> &'a mut crate::W<REG> {
        self.variant(Point::P128)
    }
    #[doc = "64 point"]
    #[inline(always)]
    pub fn p64(self) -> &'a mut crate::W<REG> {
        self.variant(Point::P64)
    }
}
#[doc = "FFT mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: FFT mode"]
    Fft = 0,
    #[doc = "1: Inverse FFT mode"]
    Ifft = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `mode` reader - FFT mode"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Fft,
            true => Mode::Ifft,
        }
    }
    #[doc = "FFT mode"]
    #[inline(always)]
    pub fn is_fft(&self) -> bool {
        *self == Mode::Fft
    }
    #[doc = "Inverse FFT mode"]
    #[inline(always)]
    pub fn is_ifft(&self) -> bool {
        *self == Mode::Ifft
    }
}
#[doc = "Field `mode` writer - FFT mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FFT mode"]
    #[inline(always)]
    pub fn fft(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Fft)
    }
    #[doc = "Inverse FFT mode"]
    #[inline(always)]
    pub fn ifft(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ifft)
    }
}
#[doc = "Field `shift` reader - Corresponding to the nine layer butterfly shift operation, 0x0: does not shift; 0x1: shift 1st layer. ..."]
pub type ShiftR = crate::FieldReader<u16>;
#[doc = "Field `shift` writer - Corresponding to the nine layer butterfly shift operation, 0x0: does not shift; 0x1: shift 1st layer. ..."]
pub type ShiftW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `enable` reader - FFT enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - FFT enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_send` reader - FFT DMA enable"]
pub type DmaSendR = crate::BitReader;
#[doc = "Field `dma_send` writer - FFT DMA enable"]
pub type DmaSendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input data arrangement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputMode {
    #[doc = "0: RIRI (real imaginary interleaved)"]
    Riri = 0,
    #[doc = "1: RRRR (only real part)"]
    Rrrr = 1,
    #[doc = "2: First input the real part and then input the imaginary part"]
    Rrii = 2,
}
impl From<InputMode> for u8 {
    #[inline(always)]
    fn from(variant: InputMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for InputMode {
    type Ux = u8;
}
impl crate::IsEnum for InputMode {}
#[doc = "Field `input_mode` reader - Input data arrangement"]
pub type InputModeR = crate::FieldReader<InputMode>;
impl InputModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<InputMode> {
        match self.bits {
            0 => Some(InputMode::Riri),
            1 => Some(InputMode::Rrrr),
            2 => Some(InputMode::Rrii),
            _ => None,
        }
    }
    #[doc = "RIRI (real imaginary interleaved)"]
    #[inline(always)]
    pub fn is_riri(&self) -> bool {
        *self == InputMode::Riri
    }
    #[doc = "RRRR (only real part)"]
    #[inline(always)]
    pub fn is_rrrr(&self) -> bool {
        *self == InputMode::Rrrr
    }
    #[doc = "First input the real part and then input the imaginary part"]
    #[inline(always)]
    pub fn is_rrii(&self) -> bool {
        *self == InputMode::Rrii
    }
}
#[doc = "Field `input_mode` writer - Input data arrangement"]
pub type InputModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, InputMode>;
impl<'a, REG> InputModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RIRI (real imaginary interleaved)"]
    #[inline(always)]
    pub fn riri(self) -> &'a mut crate::W<REG> {
        self.variant(InputMode::Riri)
    }
    #[doc = "RRRR (only real part)"]
    #[inline(always)]
    pub fn rrrr(self) -> &'a mut crate::W<REG> {
        self.variant(InputMode::Rrrr)
    }
    #[doc = "First input the real part and then input the imaginary part"]
    #[inline(always)]
    pub fn rrii(self) -> &'a mut crate::W<REG> {
        self.variant(InputMode::Rrii)
    }
}
#[doc = "Effective width of input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataMode {
    #[doc = "0: 64 bit effective"]
    Width64 = 0,
    #[doc = "1: 128 bit effective"]
    Width128 = 1,
}
impl From<DataMode> for bool {
    #[inline(always)]
    fn from(variant: DataMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `data_mode` reader - Effective width of input data"]
pub type DataModeR = crate::BitReader<DataMode>;
impl DataModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataMode {
        match self.bits {
            false => DataMode::Width64,
            true => DataMode::Width128,
        }
    }
    #[doc = "64 bit effective"]
    #[inline(always)]
    pub fn is_width_64(&self) -> bool {
        *self == DataMode::Width64
    }
    #[doc = "128 bit effective"]
    #[inline(always)]
    pub fn is_width_128(&self) -> bool {
        *self == DataMode::Width128
    }
}
#[doc = "Field `data_mode` writer - Effective width of input data"]
pub type DataModeW<'a, REG> = crate::BitWriter<'a, REG, DataMode>;
impl<'a, REG> DataModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "64 bit effective"]
    #[inline(always)]
    pub fn width_64(self) -> &'a mut crate::W<REG> {
        self.variant(DataMode::Width64)
    }
    #[doc = "128 bit effective"]
    #[inline(always)]
    pub fn width_128(self) -> &'a mut crate::W<REG> {
        self.variant(DataMode::Width128)
    }
}
impl R {
    #[doc = "Bits 0:2 - FFT calculation data length"]
    #[inline(always)]
    pub fn point(&self) -> PointR {
        PointR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - FFT mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:12 - Corresponding to the nine layer butterfly shift operation, 0x0: does not shift; 0x1: shift 1st layer. ..."]
    #[inline(always)]
    pub fn shift(&self) -> ShiftR {
        ShiftR::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - FFT enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FFT DMA enable"]
    #[inline(always)]
    pub fn dma_send(&self) -> DmaSendR {
        DmaSendR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Input data arrangement"]
    #[inline(always)]
    pub fn input_mode(&self) -> InputModeR {
        InputModeR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Effective width of input data"]
    #[inline(always)]
    pub fn data_mode(&self) -> DataModeR {
        DataModeR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FFT calculation data length"]
    #[inline(always)]
    #[must_use]
    pub fn point(&mut self) -> PointW<CtrlSpec> {
        PointW::new(self, 0)
    }
    #[doc = "Bit 3 - FFT mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtrlSpec> {
        ModeW::new(self, 3)
    }
    #[doc = "Bits 4:12 - Corresponding to the nine layer butterfly shift operation, 0x0: does not shift; 0x1: shift 1st layer. ..."]
    #[inline(always)]
    #[must_use]
    pub fn shift(&mut self) -> ShiftW<CtrlSpec> {
        ShiftW::new(self, 4)
    }
    #[doc = "Bit 13 - FFT enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlSpec> {
        EnableW::new(self, 13)
    }
    #[doc = "Bit 14 - FFT DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_send(&mut self) -> DmaSendW<CtrlSpec> {
        DmaSendW::new(self, 14)
    }
    #[doc = "Bits 15:16 - Input data arrangement"]
    #[inline(always)]
    #[must_use]
    pub fn input_mode(&mut self) -> InputModeW<CtrlSpec> {
        InputModeW::new(self, 15)
    }
    #[doc = "Bit 17 - Effective width of input data"]
    #[inline(always)]
    #[must_use]
    pub fn data_mode(&mut self) -> DataModeW<CtrlSpec> {
        DataModeW::new(self, 17)
    }
}
#[doc = "FFT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets ctrl to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u64 = 0;
}
