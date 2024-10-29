#[doc = "Register `ctrlr0` reader"]
pub type R = crate::R<Ctrlr0Spec>;
#[doc = "Register `ctrlr0` writer"]
pub type W = crate::W<Ctrlr0Spec>;
#[doc = "WORK_MODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WorkMode {
    #[doc = "0: MODE_0"]
    Mode0 = 0,
    #[doc = "1: MODE_1"]
    Mode1 = 1,
    #[doc = "2: MODE_2"]
    Mode2 = 2,
    #[doc = "3: MODE_3"]
    Mode3 = 3,
}
impl From<WorkMode> for u8 {
    #[inline(always)]
    fn from(variant: WorkMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WorkMode {
    type Ux = u8;
}
impl crate::IsEnum for WorkMode {}
#[doc = "Field `work_mode` reader - WORK_MODE"]
pub type WorkModeR = crate::FieldReader<WorkMode>;
impl WorkModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WorkMode {
        match self.bits {
            0 => WorkMode::Mode0,
            1 => WorkMode::Mode1,
            2 => WorkMode::Mode2,
            3 => WorkMode::Mode3,
            _ => unreachable!(),
        }
    }
    #[doc = "MODE_0"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == WorkMode::Mode0
    }
    #[doc = "MODE_1"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == WorkMode::Mode1
    }
    #[doc = "MODE_2"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == WorkMode::Mode2
    }
    #[doc = "MODE_3"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == WorkMode::Mode3
    }
}
#[doc = "Field `work_mode` writer - WORK_MODE"]
pub type WorkModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, WorkMode, crate::Safe>;
impl<'a, REG> WorkModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MODE_0"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(WorkMode::Mode0)
    }
    #[doc = "MODE_1"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(WorkMode::Mode1)
    }
    #[doc = "MODE_2"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(WorkMode::Mode2)
    }
    #[doc = "MODE_3"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(WorkMode::Mode3)
    }
}
#[doc = "TRANSFER_MODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmod {
    #[doc = "0: TRANS_RECV"]
    TransRecv = 0,
    #[doc = "1: TRANS"]
    Trans = 1,
    #[doc = "2: RECV"]
    Recv = 2,
    #[doc = "3: EEROM"]
    Eerom = 3,
}
impl From<Tmod> for u8 {
    #[inline(always)]
    fn from(variant: Tmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmod {
    type Ux = u8;
}
impl crate::IsEnum for Tmod {}
#[doc = "Field `tmod` reader - TRANSFER_MODE"]
pub type TmodR = crate::FieldReader<Tmod>;
impl TmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmod {
        match self.bits {
            0 => Tmod::TransRecv,
            1 => Tmod::Trans,
            2 => Tmod::Recv,
            3 => Tmod::Eerom,
            _ => unreachable!(),
        }
    }
    #[doc = "TRANS_RECV"]
    #[inline(always)]
    pub fn is_trans_recv(&self) -> bool {
        *self == Tmod::TransRecv
    }
    #[doc = "TRANS"]
    #[inline(always)]
    pub fn is_trans(&self) -> bool {
        *self == Tmod::Trans
    }
    #[doc = "RECV"]
    #[inline(always)]
    pub fn is_recv(&self) -> bool {
        *self == Tmod::Recv
    }
    #[doc = "EEROM"]
    #[inline(always)]
    pub fn is_eerom(&self) -> bool {
        *self == Tmod::Eerom
    }
}
#[doc = "Field `tmod` writer - TRANSFER_MODE"]
pub type TmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tmod, crate::Safe>;
impl<'a, REG> TmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TRANS_RECV"]
    #[inline(always)]
    pub fn trans_recv(self) -> &'a mut crate::W<REG> {
        self.variant(Tmod::TransRecv)
    }
    #[doc = "TRANS"]
    #[inline(always)]
    pub fn trans(self) -> &'a mut crate::W<REG> {
        self.variant(Tmod::Trans)
    }
    #[doc = "RECV"]
    #[inline(always)]
    pub fn recv(self) -> &'a mut crate::W<REG> {
        self.variant(Tmod::Recv)
    }
    #[doc = "EEROM"]
    #[inline(always)]
    pub fn eerom(self) -> &'a mut crate::W<REG> {
        self.variant(Tmod::Eerom)
    }
}
#[doc = "Field `data_length` reader - DATA_BIT_LENGTH"]
pub type DataLengthR = crate::FieldReader;
#[doc = "Field `data_length` writer - DATA_BIT_LENGTH"]
pub type DataLengthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "FRAME_FORMAT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameFormat {
    #[doc = "0: STANDARD"]
    Standard = 0,
    #[doc = "1: DUAL"]
    Dual = 1,
    #[doc = "2: QUAD"]
    Quad = 2,
    #[doc = "3: OCTAL"]
    Octal = 3,
}
impl From<FrameFormat> for u8 {
    #[inline(always)]
    fn from(variant: FrameFormat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FrameFormat {
    type Ux = u8;
}
impl crate::IsEnum for FrameFormat {}
#[doc = "Field `frame_format` reader - FRAME_FORMAT"]
pub type FrameFormatR = crate::FieldReader<FrameFormat>;
impl FrameFormatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrameFormat {
        match self.bits {
            0 => FrameFormat::Standard,
            1 => FrameFormat::Dual,
            2 => FrameFormat::Quad,
            3 => FrameFormat::Octal,
            _ => unreachable!(),
        }
    }
    #[doc = "STANDARD"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == FrameFormat::Standard
    }
    #[doc = "DUAL"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == FrameFormat::Dual
    }
    #[doc = "QUAD"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == FrameFormat::Quad
    }
    #[doc = "OCTAL"]
    #[inline(always)]
    pub fn is_octal(&self) -> bool {
        *self == FrameFormat::Octal
    }
}
#[doc = "Field `frame_format` writer - FRAME_FORMAT"]
pub type FrameFormatW<'a, REG> = crate::FieldWriter<'a, REG, 2, FrameFormat, crate::Safe>;
impl<'a, REG> FrameFormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "STANDARD"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(FrameFormat::Standard)
    }
    #[doc = "DUAL"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(FrameFormat::Dual)
    }
    #[doc = "QUAD"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(FrameFormat::Quad)
    }
    #[doc = "OCTAL"]
    #[inline(always)]
    pub fn octal(self) -> &'a mut crate::W<REG> {
        self.variant(FrameFormat::Octal)
    }
}
impl R {
    #[doc = "Bits 6:7 - WORK_MODE"]
    #[inline(always)]
    pub fn work_mode(&self) -> WorkModeR {
        WorkModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TRANSFER_MODE"]
    #[inline(always)]
    pub fn tmod(&self) -> TmodR {
        TmodR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:20 - DATA_BIT_LENGTH"]
    #[inline(always)]
    pub fn data_length(&self) -> DataLengthR {
        DataLengthR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - FRAME_FORMAT"]
    #[inline(always)]
    pub fn frame_format(&self) -> FrameFormatR {
        FrameFormatR::new(((self.bits >> 21) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - WORK_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn work_mode(&mut self) -> WorkModeW<Ctrlr0Spec> {
        WorkModeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - TRANSFER_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn tmod(&mut self) -> TmodW<Ctrlr0Spec> {
        TmodW::new(self, 8)
    }
    #[doc = "Bits 16:20 - DATA_BIT_LENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn data_length(&mut self) -> DataLengthW<Ctrlr0Spec> {
        DataLengthW::new(self, 16)
    }
    #[doc = "Bits 21:22 - FRAME_FORMAT"]
    #[inline(always)]
    #[must_use]
    pub fn frame_format(&mut self) -> FrameFormatW<Ctrlr0Spec> {
        FrameFormatW::new(self, 21)
    }
}
#[doc = "Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrlr0Spec;
impl crate::RegisterSpec for Ctrlr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlr0::R`](R) reader structure"]
impl crate::Readable for Ctrlr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrlr0::W`](W) writer structure"]
impl crate::Writable for Ctrlr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctrlr0 to value 0"]
impl crate::Resettable for Ctrlr0Spec {
    const RESET_VALUE: u32 = 0;
}
