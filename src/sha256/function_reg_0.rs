#[doc = "Register `function_reg_0` reader"]
pub type R = crate::R<FunctionReg0Spec>;
#[doc = "Register `function_reg_0` writer"]
pub type W = crate::W<FunctionReg0Spec>;
#[doc = "Field `en` reader - write:SHA256 enable register. read:Calculation completed flag"]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - write:SHA256 enable register. read:Calculation completed flag"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `overflow` reader - SHA256 calculation overflow flag"]
pub type OverflowR = crate::BitReader;
#[doc = "Field `overflow` writer - SHA256 calculation overflow flag"]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Endian setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    #[doc = "0: Little endian"]
    Le = 0,
    #[doc = "1: Big endian"]
    Be = 1,
}
impl From<Endian> for bool {
    #[inline(always)]
    fn from(variant: Endian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `endian` reader - Endian setting"]
pub type EndianR = crate::BitReader<Endian>;
impl EndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endian {
        match self.bits {
            false => Endian::Le,
            true => Endian::Be,
        }
    }
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == Endian::Le
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == Endian::Be
    }
}
#[doc = "Field `endian` writer - Endian setting"]
pub type EndianW<'a, REG> = crate::BitWriter<'a, REG, Endian>;
impl<'a, REG> EndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn le(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Le)
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn be(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Be)
    }
}
impl R {
    #[doc = "Bit 0 - write:SHA256 enable register. read:Calculation completed flag"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - SHA256 calculation overflow flag"]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Endian setting"]
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write:SHA256 enable register. read:Calculation completed flag"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<FunctionReg0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 8 - SHA256 calculation overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OverflowW<FunctionReg0Spec> {
        OverflowW::new(self, 8)
    }
    #[doc = "Bit 16 - Endian setting"]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> EndianW<FunctionReg0Spec> {
        EndianW::new(self, 16)
    }
}
#[doc = "Function configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`function_reg_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`function_reg_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FunctionReg0Spec;
impl crate::RegisterSpec for FunctionReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`function_reg_0::R`](R) reader structure"]
impl crate::Readable for FunctionReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`function_reg_0::W`](W) writer structure"]
impl crate::Writable for FunctionReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets function_reg_0 to value 0"]
impl crate::Resettable for FunctionReg0Spec {
    const RESET_VALUE: u32 = 0;
}
