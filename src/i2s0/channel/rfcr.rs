#[doc = "Register `rfcr` reader"]
pub type R = crate::R<RfcrSpec>;
#[doc = "Register `rfcr` writer"]
pub type W = crate::W<RfcrSpec>;
#[doc = "Trigger level in the RX FIFO at which the receiver data available interrupt generate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Level {
    #[doc = "0: Interrupt trigger when FIFO level is 1"]
    Level1 = 0,
    #[doc = "1: Interrupt trigger when FIFO level is 2"]
    Level2 = 1,
    #[doc = "2: Interrupt trigger when FIFO level is 3"]
    Level3 = 2,
    #[doc = "3: Interrupt trigger when FIFO level is 4"]
    Level4 = 3,
    #[doc = "4: Interrupt trigger when FIFO level is 5"]
    Level5 = 4,
    #[doc = "5: Interrupt trigger when FIFO level is 6"]
    Level6 = 5,
    #[doc = "6: Interrupt trigger when FIFO level is 7"]
    Level7 = 6,
    #[doc = "7: Interrupt trigger when FIFO level is 8"]
    Level8 = 7,
    #[doc = "8: Interrupt trigger when FIFO level is 9"]
    Level9 = 8,
    #[doc = "9: Interrupt trigger when FIFO level is 10"]
    Level10 = 9,
    #[doc = "10: Interrupt trigger when FIFO level is 11"]
    Level11 = 10,
    #[doc = "11: Interrupt trigger when FIFO level is 12"]
    Level12 = 11,
    #[doc = "12: Interrupt trigger when FIFO level is 13"]
    Level13 = 12,
    #[doc = "13: Interrupt trigger when FIFO level is 14"]
    Level14 = 13,
    #[doc = "14: Interrupt trigger when FIFO level is 15"]
    Level15 = 14,
    #[doc = "15: Interrupt trigger when FIFO level is 16"]
    Level16 = 15,
}
impl From<Level> for u8 {
    #[inline(always)]
    fn from(variant: Level) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Level {
    type Ux = u8;
}
impl crate::IsEnum for Level {}
#[doc = "Field `rxchdt` reader - Trigger level in the RX FIFO at which the receiver data available interrupt generate"]
pub type RxchdtR = crate::FieldReader<Level>;
impl RxchdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Level {
        match self.bits {
            0 => Level::Level1,
            1 => Level::Level2,
            2 => Level::Level3,
            3 => Level::Level4,
            4 => Level::Level5,
            5 => Level::Level6,
            6 => Level::Level7,
            7 => Level::Level8,
            8 => Level::Level9,
            9 => Level::Level10,
            10 => Level::Level11,
            11 => Level::Level12,
            12 => Level::Level13,
            13 => Level::Level14,
            14 => Level::Level15,
            15 => Level::Level16,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt trigger when FIFO level is 1"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Level::Level1
    }
    #[doc = "Interrupt trigger when FIFO level is 2"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Level::Level2
    }
    #[doc = "Interrupt trigger when FIFO level is 3"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == Level::Level3
    }
    #[doc = "Interrupt trigger when FIFO level is 4"]
    #[inline(always)]
    pub fn is_level4(&self) -> bool {
        *self == Level::Level4
    }
    #[doc = "Interrupt trigger when FIFO level is 5"]
    #[inline(always)]
    pub fn is_level5(&self) -> bool {
        *self == Level::Level5
    }
    #[doc = "Interrupt trigger when FIFO level is 6"]
    #[inline(always)]
    pub fn is_level6(&self) -> bool {
        *self == Level::Level6
    }
    #[doc = "Interrupt trigger when FIFO level is 7"]
    #[inline(always)]
    pub fn is_level7(&self) -> bool {
        *self == Level::Level7
    }
    #[doc = "Interrupt trigger when FIFO level is 8"]
    #[inline(always)]
    pub fn is_level8(&self) -> bool {
        *self == Level::Level8
    }
    #[doc = "Interrupt trigger when FIFO level is 9"]
    #[inline(always)]
    pub fn is_level9(&self) -> bool {
        *self == Level::Level9
    }
    #[doc = "Interrupt trigger when FIFO level is 10"]
    #[inline(always)]
    pub fn is_level10(&self) -> bool {
        *self == Level::Level10
    }
    #[doc = "Interrupt trigger when FIFO level is 11"]
    #[inline(always)]
    pub fn is_level11(&self) -> bool {
        *self == Level::Level11
    }
    #[doc = "Interrupt trigger when FIFO level is 12"]
    #[inline(always)]
    pub fn is_level12(&self) -> bool {
        *self == Level::Level12
    }
    #[doc = "Interrupt trigger when FIFO level is 13"]
    #[inline(always)]
    pub fn is_level13(&self) -> bool {
        *self == Level::Level13
    }
    #[doc = "Interrupt trigger when FIFO level is 14"]
    #[inline(always)]
    pub fn is_level14(&self) -> bool {
        *self == Level::Level14
    }
    #[doc = "Interrupt trigger when FIFO level is 15"]
    #[inline(always)]
    pub fn is_level15(&self) -> bool {
        *self == Level::Level15
    }
    #[doc = "Interrupt trigger when FIFO level is 16"]
    #[inline(always)]
    pub fn is_level16(&self) -> bool {
        *self == Level::Level16
    }
}
#[doc = "Field `rxchdt` writer - Trigger level in the RX FIFO at which the receiver data available interrupt generate"]
pub type RxchdtW<'a, REG> = crate::FieldWriter<'a, REG, 4, Level, crate::Safe>;
impl<'a, REG> RxchdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt trigger when FIFO level is 1"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level1)
    }
    #[doc = "Interrupt trigger when FIFO level is 2"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level2)
    }
    #[doc = "Interrupt trigger when FIFO level is 3"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level3)
    }
    #[doc = "Interrupt trigger when FIFO level is 4"]
    #[inline(always)]
    pub fn level4(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level4)
    }
    #[doc = "Interrupt trigger when FIFO level is 5"]
    #[inline(always)]
    pub fn level5(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level5)
    }
    #[doc = "Interrupt trigger when FIFO level is 6"]
    #[inline(always)]
    pub fn level6(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level6)
    }
    #[doc = "Interrupt trigger when FIFO level is 7"]
    #[inline(always)]
    pub fn level7(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level7)
    }
    #[doc = "Interrupt trigger when FIFO level is 8"]
    #[inline(always)]
    pub fn level8(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level8)
    }
    #[doc = "Interrupt trigger when FIFO level is 9"]
    #[inline(always)]
    pub fn level9(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level9)
    }
    #[doc = "Interrupt trigger when FIFO level is 10"]
    #[inline(always)]
    pub fn level10(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level10)
    }
    #[doc = "Interrupt trigger when FIFO level is 11"]
    #[inline(always)]
    pub fn level11(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level11)
    }
    #[doc = "Interrupt trigger when FIFO level is 12"]
    #[inline(always)]
    pub fn level12(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level12)
    }
    #[doc = "Interrupt trigger when FIFO level is 13"]
    #[inline(always)]
    pub fn level13(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level13)
    }
    #[doc = "Interrupt trigger when FIFO level is 14"]
    #[inline(always)]
    pub fn level14(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level14)
    }
    #[doc = "Interrupt trigger when FIFO level is 15"]
    #[inline(always)]
    pub fn level15(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level15)
    }
    #[doc = "Interrupt trigger when FIFO level is 16"]
    #[inline(always)]
    pub fn level16(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Level16)
    }
}
impl R {
    #[doc = "Bits 0:3 - Trigger level in the RX FIFO at which the receiver data available interrupt generate"]
    #[inline(always)]
    pub fn rxchdt(&self) -> RxchdtR {
        RxchdtR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trigger level in the RX FIFO at which the receiver data available interrupt generate"]
    #[inline(always)]
    #[must_use]
    pub fn rxchdt(&mut self) -> RxchdtW<RfcrSpec> {
        RxchdtW::new(self, 0)
    }
}
#[doc = "Receive FIFO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcrSpec;
impl crate::RegisterSpec for RfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcr::R`](R) reader structure"]
impl crate::Readable for RfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcr::W`](W) writer structure"]
impl crate::Writable for RfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rfcr to value 0"]
impl crate::Resettable for RfcrSpec {
    const RESET_VALUE: u32 = 0;
}
