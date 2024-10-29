#[doc = "Register `dir_bidx[%s]` reader"]
pub type R = crate::R<DirBidxSpec>;
#[doc = "Register `dir_bidx[%s]` writer"]
pub type W = crate::W<DirBidxSpec>;
#[doc = "Field `rd_idx(0-3)` reader - rd_idx%s"]
pub type RdIdxR = crate::FieldReader;
#[doc = "Field `rd_idx(0-3)` writer - rd_idx%s"]
pub type RdIdxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "rd_idx(0-3)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `rd_idx0` field.</div>"]
    #[inline(always)]
    pub fn rd_idx(&self, n: u8) -> RdIdxR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RdIdxR::new(((self.bits >> (n * 8)) & 0x3f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "rd_idx(0-3)"]
    #[inline(always)]
    pub fn rd_idx_iter(&self) -> impl Iterator<Item = RdIdxR> + '_ {
        (0..4).map(move |n| RdIdxR::new(((self.bits >> (n * 8)) & 0x3f) as u8))
    }
    #[doc = "Bits 0:5 - rd_idx0"]
    #[inline(always)]
    pub fn rd_idx0(&self) -> RdIdxR {
        RdIdxR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - rd_idx1"]
    #[inline(always)]
    pub fn rd_idx1(&self) -> RdIdxR {
        RdIdxR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - rd_idx2"]
    #[inline(always)]
    pub fn rd_idx2(&self) -> RdIdxR {
        RdIdxR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - rd_idx3"]
    #[inline(always)]
    pub fn rd_idx3(&self) -> RdIdxR {
        RdIdxR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "rd_idx(0-3)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `rd_idx0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn rd_idx(&mut self, n: u8) -> RdIdxW<DirBidxSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RdIdxW::new(self, n * 8)
    }
    #[doc = "Bits 0:5 - rd_idx0"]
    #[inline(always)]
    #[must_use]
    pub fn rd_idx0(&mut self) -> RdIdxW<DirBidxSpec> {
        RdIdxW::new(self, 0)
    }
    #[doc = "Bits 8:13 - rd_idx1"]
    #[inline(always)]
    #[must_use]
    pub fn rd_idx1(&mut self) -> RdIdxW<DirBidxSpec> {
        RdIdxW::new(self, 8)
    }
    #[doc = "Bits 16:21 - rd_idx2"]
    #[inline(always)]
    #[must_use]
    pub fn rd_idx2(&mut self) -> RdIdxW<DirBidxSpec> {
        RdIdxW::new(self, 16)
    }
    #[doc = "Bits 24:29 - rd_idx3"]
    #[inline(always)]
    #[must_use]
    pub fn rd_idx3(&mut self) -> RdIdxW<DirBidxSpec> {
        RdIdxW::new(self, 24)
    }
}
#[doc = "Direction Sample Buffer Read Index Configure Register (16 directions * 2 values * 4 indices)\n\nYou can [`read`](crate::Reg::read) this register and get [`dir_bidx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir_bidx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirBidxSpec;
impl crate::RegisterSpec for DirBidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dir_bidx::R`](R) reader structure"]
impl crate::Readable for DirBidxSpec {}
#[doc = "`write(|w| ..)` method takes [`dir_bidx::W`](W) writer structure"]
impl crate::Writable for DirBidxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dir_bidx[%s]
to value 0"]
impl crate::Resettable for DirBidxSpec {
    const RESET_VALUE: u32 = 0;
}
