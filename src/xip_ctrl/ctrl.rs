#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - When 1, enable the cache. When the cache is disabled, all XIP accesses  
 will go straight to the flash, without querying the cache. When enabled,  
 cacheable XIP accesses will query the cache, and the flash will  
 not be accessed if the tag matches and the valid bit is set.  

 If the cache is enabled, cache-as-SRAM accesses have no effect on the  
 cache data RAM, and will produce a bus error response."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - When 1, enable the cache. When the cache is disabled, all XIP accesses  
 will go straight to the flash, without querying the cache. When enabled,  
 cacheable XIP accesses will query the cache, and the flash will  
 not be accessed if the tag matches and the valid bit is set.  

 If the cache is enabled, cache-as-SRAM accesses have no effect on the  
 cache data RAM, and will produce a bus error response."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ERR_BADWRITE` reader - When 1, writes to any alias other than 0x0 (caching, allocating)  
 will produce a bus fault. When 0, these writes are silently ignored.  
 In either case, writes to the 0x0 alias will deallocate on tag match,  
 as usual."]
pub type ERR_BADWRITE_R = crate::BitReader<bool>;
#[doc = "Field `ERR_BADWRITE` writer - When 1, writes to any alias other than 0x0 (caching, allocating)  
 will produce a bus fault. When 0, these writes are silently ignored.  
 In either case, writes to the 0x0 alias will deallocate on tag match,  
 as usual."]
pub type ERR_BADWRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `POWER_DOWN` reader - When 1, the cache memories are powered down. They retain state,  
 but can not be accessed. This reduces static power dissipation.  
 Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot  
 be enabled when powered down.  
 Cache-as-SRAM accesses will produce a bus error response when  
 the cache is powered down."]
pub type POWER_DOWN_R = crate::BitReader<bool>;
#[doc = "Field `POWER_DOWN` writer - When 1, the cache memories are powered down. They retain state,  
 but can not be accessed. This reduces static power dissipation.  
 Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot  
 be enabled when powered down.  
 Cache-as-SRAM accesses will produce a bus error response when  
 the cache is powered down."]
pub type POWER_DOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When 1, enable the cache. When the cache is disabled, all XIP accesses  
 will go straight to the flash, without querying the cache. When enabled,  
 cacheable XIP accesses will query the cache, and the flash will  
 not be accessed if the tag matches and the valid bit is set.  

 If the cache is enabled, cache-as-SRAM accesses have no effect on the  
 cache data RAM, and will produce a bus error response."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, writes to any alias other than 0x0 (caching, allocating)  
 will produce a bus fault. When 0, these writes are silently ignored.  
 In either case, writes to the 0x0 alias will deallocate on tag match,  
 as usual."]
    #[inline(always)]
    pub fn err_badwrite(&self) -> ERR_BADWRITE_R {
        ERR_BADWRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, the cache memories are powered down. They retain state,  
 but can not be accessed. This reduces static power dissipation.  
 Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot  
 be enabled when powered down.  
 Cache-as-SRAM accesses will produce a bus error response when  
 the cache is powered down."]
    #[inline(always)]
    pub fn power_down(&self) -> POWER_DOWN_R {
        POWER_DOWN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enable the cache. When the cache is disabled, all XIP accesses  
 will go straight to the flash, without querying the cache. When enabled,  
 cacheable XIP accesses will query the cache, and the flash will  
 not be accessed if the tag matches and the valid bit is set.  

 If the cache is enabled, cache-as-SRAM accesses have no effect on the  
 cache data RAM, and will produce a bus error response."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - When 1, writes to any alias other than 0x0 (caching, allocating)  
 will produce a bus fault. When 0, these writes are silently ignored.  
 In either case, writes to the 0x0 alias will deallocate on tag match,  
 as usual."]
    #[inline(always)]
    #[must_use]
    pub fn err_badwrite(&mut self) -> ERR_BADWRITE_W<1> {
        ERR_BADWRITE_W::new(self)
    }
    #[doc = "Bit 3 - When 1, the cache memories are powered down. They retain state,  
 but can not be accessed. This reduces static power dissipation.  
 Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot  
 be enabled when powered down.  
 Cache-as-SRAM accesses will produce a bus error response when  
 the cache is powered down."]
    #[inline(always)]
    #[must_use]
    pub fn power_down(&mut self) -> POWER_DOWN_W<3> {
        POWER_DOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x03"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
