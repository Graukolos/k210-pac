#[doc = "Register `swhsdst` reader"]
pub type R = crate::R<SwhsdstSpec>;
#[doc = "Register `swhsdst` writer"]
pub type W = crate::W<SwhsdstSpec>;
#[doc = "Field `req` reader - Software handshake request for channel destination"]
pub type ReqR = crate::BitReader;
#[doc = "Field `req` writer - Software handshake request for channel destination"]
pub type ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `req_we` reader - Write enable bit for software handshake request"]
pub type ReqWeR = crate::BitReader;
#[doc = "Field `req_we` writer - Write enable bit for software handshake request"]
pub type ReqWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sglreq` reader - Software handshake single request for channel destination"]
pub type SglreqR = crate::BitReader;
#[doc = "Field `sglreq` writer - Software handshake single request for channel destination"]
pub type SglreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sglreq_we` reader - Write enable bit for software handshake"]
pub type SglreqWeR = crate::BitReader;
#[doc = "Field `sglreq_we` writer - Write enable bit for software handshake"]
pub type SglreqWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lst` reader - Software handshake last request for channel destination"]
pub type LstR = crate::BitReader;
#[doc = "Field `lst` writer - Software handshake last request for channel destination"]
pub type LstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lst_we` reader - Write enable bit for software handshake last request"]
pub type LstWeR = crate::BitReader;
#[doc = "Field `lst_we` writer - Write enable bit for software handshake last request"]
pub type LstWeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software handshake request for channel destination"]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write enable bit for software handshake request"]
    #[inline(always)]
    pub fn req_we(&self) -> ReqWeR {
        ReqWeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software handshake single request for channel destination"]
    #[inline(always)]
    pub fn sglreq(&self) -> SglreqR {
        SglreqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write enable bit for software handshake"]
    #[inline(always)]
    pub fn sglreq_we(&self) -> SglreqWeR {
        SglreqWeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software handshake last request for channel destination"]
    #[inline(always)]
    pub fn lst(&self) -> LstR {
        LstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write enable bit for software handshake last request"]
    #[inline(always)]
    pub fn lst_we(&self) -> LstWeR {
        LstWeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software handshake request for channel destination"]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> ReqW<SwhsdstSpec> {
        ReqW::new(self, 0)
    }
    #[doc = "Bit 1 - Write enable bit for software handshake request"]
    #[inline(always)]
    #[must_use]
    pub fn req_we(&mut self) -> ReqWeW<SwhsdstSpec> {
        ReqWeW::new(self, 1)
    }
    #[doc = "Bit 2 - Software handshake single request for channel destination"]
    #[inline(always)]
    #[must_use]
    pub fn sglreq(&mut self) -> SglreqW<SwhsdstSpec> {
        SglreqW::new(self, 2)
    }
    #[doc = "Bit 3 - Write enable bit for software handshake"]
    #[inline(always)]
    #[must_use]
    pub fn sglreq_we(&mut self) -> SglreqWeW<SwhsdstSpec> {
        SglreqWeW::new(self, 3)
    }
    #[doc = "Bit 4 - Software handshake last request for channel destination"]
    #[inline(always)]
    #[must_use]
    pub fn lst(&mut self) -> LstW<SwhsdstSpec> {
        LstW::new(self, 4)
    }
    #[doc = "Bit 5 - Write enable bit for software handshake last request"]
    #[inline(always)]
    #[must_use]
    pub fn lst_we(&mut self) -> LstWeW<SwhsdstSpec> {
        LstWeW::new(self, 5)
    }
}
#[doc = "Channel Software handshake Destination Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swhsdst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swhsdst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwhsdstSpec;
impl crate::RegisterSpec for SwhsdstSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`swhsdst::R`](R) reader structure"]
impl crate::Readable for SwhsdstSpec {}
#[doc = "`write(|w| ..)` method takes [`swhsdst::W`](W) writer structure"]
impl crate::Writable for SwhsdstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets swhsdst to value 0"]
impl crate::Resettable for SwhsdstSpec {
    const RESET_VALUE: u64 = 0;
}
