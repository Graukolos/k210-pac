#[doc = "Register `rcr` reader"]
pub type R = crate::R<RcrSpec>;
#[doc = "Register `rcr` writer"]
pub type W = crate::W<RcrSpec>;
#[doc = "Desired data resolution of receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wlen {
    #[doc = "0: Ignore the word length"]
    Ignore = 0,
    #[doc = "1: 12-bit data resolution of the receiver"]
    Resolution12 = 1,
    #[doc = "2: 16-bit data resolution of the receiver"]
    Resolution16 = 2,
    #[doc = "3: 20-bit data resolution of the receiver"]
    Resolution20 = 3,
    #[doc = "4: 24-bit data resolution of the receiver"]
    Resolution24 = 4,
    #[doc = "5: 32-bit data resolution of the receiver"]
    Resolution32 = 5,
}
impl From<Wlen> for u8 {
    #[inline(always)]
    fn from(variant: Wlen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wlen {
    type Ux = u8;
}
impl crate::IsEnum for Wlen {}
#[doc = "Field `wlen` reader - Desired data resolution of receiver"]
pub type WlenR = crate::FieldReader<Wlen>;
impl WlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wlen> {
        match self.bits {
            0 => Some(Wlen::Ignore),
            1 => Some(Wlen::Resolution12),
            2 => Some(Wlen::Resolution16),
            3 => Some(Wlen::Resolution20),
            4 => Some(Wlen::Resolution24),
            5 => Some(Wlen::Resolution32),
            _ => None,
        }
    }
    #[doc = "Ignore the word length"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == Wlen::Ignore
    }
    #[doc = "12-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn is_resolution12(&self) -> bool {
        *self == Wlen::Resolution12
    }
    #[doc = "16-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn is_resolution16(&self) -> bool {
        *self == Wlen::Resolution16
    }
    #[doc = "20-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn is_resolution20(&self) -> bool {
        *self == Wlen::Resolution20
    }
    #[doc = "24-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn is_resolution24(&self) -> bool {
        *self == Wlen::Resolution24
    }
    #[doc = "32-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn is_resolution32(&self) -> bool {
        *self == Wlen::Resolution32
    }
}
#[doc = "Field `wlen` writer - Desired data resolution of receiver"]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wlen>;
impl<'a, REG> WlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ignore the word length"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Ignore)
    }
    #[doc = "12-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution12(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Resolution12)
    }
    #[doc = "16-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution16(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Resolution16)
    }
    #[doc = "20-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution20(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Resolution20)
    }
    #[doc = "24-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution24(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Resolution24)
    }
    #[doc = "32-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution32(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Resolution32)
    }
}
impl R {
    #[doc = "Bits 0:2 - Desired data resolution of receiver"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Desired data resolution of receiver"]
    #[inline(always)]
    #[must_use]
    pub fn wlen(&mut self) -> WlenW<RcrSpec> {
        WlenW::new(self, 0)
    }
}
#[doc = "Receive Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcrSpec;
impl crate::RegisterSpec for RcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::Readable for RcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::Writable for RcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rcr to value 0"]
impl crate::Resettable for RcrSpec {
    const RESET_VALUE: u32 = 0;
}
