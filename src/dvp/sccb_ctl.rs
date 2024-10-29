#[doc = "Register `sccb_ctl` reader"]
pub type R = crate::R<SccbCtlSpec>;
#[doc = "Register `sccb_ctl` writer"]
pub type W = crate::W<SccbCtlSpec>;
#[doc = "Field `device_address` reader - DEVICE_ADDRESS"]
pub type DeviceAddressR = crate::FieldReader;
#[doc = "Field `device_address` writer - DEVICE_ADDRESS"]
pub type DeviceAddressW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_address` reader - REG_ADDRESS"]
pub type RegAddressR = crate::FieldReader;
#[doc = "Field `reg_address` writer - REG_ADDRESS"]
pub type RegAddressW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `wdata_byte0` reader - WDATA_BYTE0"]
pub type WdataByte0R = crate::FieldReader;
#[doc = "Field `wdata_byte0` writer - WDATA_BYTE0"]
pub type WdataByte0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `wdata_byte1` reader - WDATA_BYTE1"]
pub type WdataByte1R = crate::FieldReader;
#[doc = "Field `wdata_byte1` writer - WDATA_BYTE1"]
pub type WdataByte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DEVICE_ADDRESS"]
    #[inline(always)]
    pub fn device_address(&self) -> DeviceAddressR {
        DeviceAddressR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_ADDRESS"]
    #[inline(always)]
    pub fn reg_address(&self) -> RegAddressR {
        RegAddressR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - WDATA_BYTE0"]
    #[inline(always)]
    pub fn wdata_byte0(&self) -> WdataByte0R {
        WdataByte0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - WDATA_BYTE1"]
    #[inline(always)]
    pub fn wdata_byte1(&self) -> WdataByte1R {
        WdataByte1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DEVICE_ADDRESS"]
    #[inline(always)]
    #[must_use]
    pub fn device_address(&mut self) -> DeviceAddressW<SccbCtlSpec> {
        DeviceAddressW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_ADDRESS"]
    #[inline(always)]
    #[must_use]
    pub fn reg_address(&mut self) -> RegAddressW<SccbCtlSpec> {
        RegAddressW::new(self, 8)
    }
    #[doc = "Bits 16:23 - WDATA_BYTE0"]
    #[inline(always)]
    #[must_use]
    pub fn wdata_byte0(&mut self) -> WdataByte0W<SccbCtlSpec> {
        WdataByte0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - WDATA_BYTE1"]
    #[inline(always)]
    #[must_use]
    pub fn wdata_byte1(&mut self) -> WdataByte1W<SccbCtlSpec> {
        WdataByte1W::new(self, 24)
    }
}
#[doc = "SCCB Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sccb_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccb_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccbCtlSpec;
impl crate::RegisterSpec for SccbCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sccb_ctl::R`](R) reader structure"]
impl crate::Readable for SccbCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`sccb_ctl::W`](W) writer structure"]
impl crate::Writable for SccbCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sccb_ctl to value 0"]
impl crate::Resettable for SccbCtlSpec {
    const RESET_VALUE: u32 = 0;
}
