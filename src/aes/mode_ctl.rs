#[doc = "Register `mode_ctl` reader"]
pub type R = crate::R<ModeCtlSpec>;
#[doc = "Register `mode_ctl` writer"]
pub type W = crate::W<ModeCtlSpec>;
#[doc = "Cipher mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CipherMode {
    #[doc = "0: Electronic Codebook"]
    Ecb = 0,
    #[doc = "1: Cipher Block Chaining"]
    Cbc = 1,
    #[doc = "2: Galois/Counter Mode"]
    Gcm = 2,
}
impl From<CipherMode> for u8 {
    #[inline(always)]
    fn from(variant: CipherMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CipherMode {
    type Ux = u8;
}
impl crate::IsEnum for CipherMode {}
#[doc = "Field `cipher_mode` reader - Cipher mode"]
pub type CipherModeR = crate::FieldReader<CipherMode>;
impl CipherModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CipherMode> {
        match self.bits {
            0 => Some(CipherMode::Ecb),
            1 => Some(CipherMode::Cbc),
            2 => Some(CipherMode::Gcm),
            _ => None,
        }
    }
    #[doc = "Electronic Codebook"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == CipherMode::Ecb
    }
    #[doc = "Cipher Block Chaining"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == CipherMode::Cbc
    }
    #[doc = "Galois/Counter Mode"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == CipherMode::Gcm
    }
}
#[doc = "Field `cipher_mode` writer - Cipher mode"]
pub type CipherModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, CipherMode>;
impl<'a, REG> CipherModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Electronic Codebook"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(CipherMode::Ecb)
    }
    #[doc = "Cipher Block Chaining"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(CipherMode::Cbc)
    }
    #[doc = "Galois/Counter Mode"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut crate::W<REG> {
        self.variant(CipherMode::Gcm)
    }
}
#[doc = "Key mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyMode {
    #[doc = "0: AES-128"]
    Aes128 = 0,
    #[doc = "1: AES-192"]
    Aes192 = 1,
    #[doc = "2: AES-256"]
    Aes256 = 2,
}
impl From<KeyMode> for u8 {
    #[inline(always)]
    fn from(variant: KeyMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KeyMode {
    type Ux = u8;
}
impl crate::IsEnum for KeyMode {}
#[doc = "Field `key_mode` reader - Key mode"]
pub type KeyModeR = crate::FieldReader<KeyMode>;
impl KeyModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<KeyMode> {
        match self.bits {
            0 => Some(KeyMode::Aes128),
            1 => Some(KeyMode::Aes192),
            2 => Some(KeyMode::Aes256),
            _ => None,
        }
    }
    #[doc = "AES-128"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KeyMode::Aes128
    }
    #[doc = "AES-192"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == KeyMode::Aes192
    }
    #[doc = "AES-256"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KeyMode::Aes256
    }
}
#[doc = "Field `key_mode` writer - Key mode"]
pub type KeyModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, KeyMode>;
impl<'a, REG> KeyModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AES-128"]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut crate::W<REG> {
        self.variant(KeyMode::Aes128)
    }
    #[doc = "AES-192"]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut crate::W<REG> {
        self.variant(KeyMode::Aes192)
    }
    #[doc = "AES-256"]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut crate::W<REG> {
        self.variant(KeyMode::Aes256)
    }
}
#[doc = "Input key order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    #[doc = "0: Big Endian"]
    Be = 0,
    #[doc = "1: Little Endian"]
    Le = 1,
}
impl From<Endian> for bool {
    #[inline(always)]
    fn from(variant: Endian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `key_order` reader - Input key order"]
pub type KeyOrderR = crate::BitReader<Endian>;
impl KeyOrderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endian {
        match self.bits {
            false => Endian::Be,
            true => Endian::Le,
        }
    }
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == Endian::Be
    }
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == Endian::Le
    }
}
#[doc = "Field `key_order` writer - Input key order"]
pub type KeyOrderW<'a, REG> = crate::BitWriter<'a, REG, Endian>;
impl<'a, REG> KeyOrderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn be(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Be)
    }
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn le(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Le)
    }
}
#[doc = "Field `input_order` reader - Input data order"]
pub use KeyOrderR as InputOrderR;
#[doc = "Field `output_order` reader - Output data order"]
pub use KeyOrderR as OutputOrderR;
#[doc = "Field `input_order` writer - Input data order"]
pub use KeyOrderW as InputOrderW;
#[doc = "Field `output_order` writer - Output data order"]
pub use KeyOrderW as OutputOrderW;
impl R {
    #[doc = "Bits 0:2 - Cipher mode"]
    #[inline(always)]
    pub fn cipher_mode(&self) -> CipherModeR {
        CipherModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Key mode"]
    #[inline(always)]
    pub fn key_mode(&self) -> KeyModeR {
        KeyModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Input key order"]
    #[inline(always)]
    pub fn key_order(&self) -> KeyOrderR {
        KeyOrderR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Input data order"]
    #[inline(always)]
    pub fn input_order(&self) -> InputOrderR {
        InputOrderR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Output data order"]
    #[inline(always)]
    pub fn output_order(&self) -> OutputOrderR {
        OutputOrderR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cipher mode"]
    #[inline(always)]
    #[must_use]
    pub fn cipher_mode(&mut self) -> CipherModeW<ModeCtlSpec> {
        CipherModeW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Key mode"]
    #[inline(always)]
    #[must_use]
    pub fn key_mode(&mut self) -> KeyModeW<ModeCtlSpec> {
        KeyModeW::new(self, 3)
    }
    #[doc = "Bit 5 - Input key order"]
    #[inline(always)]
    #[must_use]
    pub fn key_order(&mut self) -> KeyOrderW<ModeCtlSpec> {
        KeyOrderW::new(self, 5)
    }
    #[doc = "Bit 7 - Input data order"]
    #[inline(always)]
    #[must_use]
    pub fn input_order(&mut self) -> InputOrderW<ModeCtlSpec> {
        InputOrderW::new(self, 7)
    }
    #[doc = "Bit 9 - Output data order"]
    #[inline(always)]
    #[must_use]
    pub fn output_order(&mut self) -> OutputOrderW<ModeCtlSpec> {
        OutputOrderW::new(self, 9)
    }
}
#[doc = "AES mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeCtlSpec;
impl crate::RegisterSpec for ModeCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode_ctl::R`](R) reader structure"]
impl crate::Readable for ModeCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`mode_ctl::W`](W) writer structure"]
impl crate::Writable for ModeCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mode_ctl to value 0"]
impl crate::Resettable for ModeCtlSpec {
    const RESET_VALUE: u32 = 0;
}
