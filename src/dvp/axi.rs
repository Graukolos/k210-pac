#[doc = "Register `axi` reader"]
pub type R = crate::R<AxiSpec>;
#[doc = "Register `axi` writer"]
pub type W = crate::W<AxiSpec>;
#[doc = "GM_MLEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GmMlen {
    #[doc = "0: GM_MLEN_1BYTE"]
    Byte1 = 0,
    #[doc = "3: GM_MLEN_4BYTE"]
    Byte4 = 3,
}
impl From<GmMlen> for u8 {
    #[inline(always)]
    fn from(variant: GmMlen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GmMlen {
    type Ux = u8;
}
impl crate::IsEnum for GmMlen {}
#[doc = "Field `gm_mlen` reader - GM_MLEN"]
pub type GmMlenR = crate::FieldReader<GmMlen>;
impl GmMlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GmMlen> {
        match self.bits {
            0 => Some(GmMlen::Byte1),
            3 => Some(GmMlen::Byte4),
            _ => None,
        }
    }
    #[doc = "GM_MLEN_1BYTE"]
    #[inline(always)]
    pub fn is_byte1(&self) -> bool {
        *self == GmMlen::Byte1
    }
    #[doc = "GM_MLEN_4BYTE"]
    #[inline(always)]
    pub fn is_byte4(&self) -> bool {
        *self == GmMlen::Byte4
    }
}
#[doc = "Field `gm_mlen` writer - GM_MLEN"]
pub type GmMlenW<'a, REG> = crate::FieldWriter<'a, REG, 8, GmMlen>;
impl<'a, REG> GmMlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GM_MLEN_1BYTE"]
    #[inline(always)]
    pub fn byte1(self) -> &'a mut crate::W<REG> {
        self.variant(GmMlen::Byte1)
    }
    #[doc = "GM_MLEN_4BYTE"]
    #[inline(always)]
    pub fn byte4(self) -> &'a mut crate::W<REG> {
        self.variant(GmMlen::Byte4)
    }
}
impl R {
    #[doc = "Bits 0:7 - GM_MLEN"]
    #[inline(always)]
    pub fn gm_mlen(&self) -> GmMlenR {
        GmMlenR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GM_MLEN"]
    #[inline(always)]
    #[must_use]
    pub fn gm_mlen(&mut self) -> GmMlenW<AxiSpec> {
        GmMlenW::new(self, 0)
    }
}
#[doc = "AXI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiSpec;
impl crate::RegisterSpec for AxiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi::R`](R) reader structure"]
impl crate::Readable for AxiSpec {}
#[doc = "`write(|w| ..)` method takes [`axi::W`](W) writer structure"]
impl crate::Writable for AxiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets axi to value 0"]
impl crate::Resettable for AxiSpec {
    const RESET_VALUE: u32 = 0;
}
