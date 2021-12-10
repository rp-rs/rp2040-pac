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
#[doc = "Field `POWER_DOWN` reader - When 1, the cache memories are powered down. They retain state,  
 but can not be accessed. This reduces static power dissipation.  
 Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot  
 be enabled when powered down.  
 Cache-as-SRAM accesses will produce a bus error response when  
 the cache is powered down."]
pub struct POWER_DOWN_R(crate::FieldReader<bool, bool>);
impl POWER_DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POWER_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWER_DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWER_DOWN` writer - When 1, the cache memories are powered down. They retain state,  
 but can not be accessed. This reduces static power dissipation.  
 Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot  
 be enabled when powered down.  
 Cache-as-SRAM accesses will produce a bus error response when  
 the cache is powered down."]
pub struct POWER_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_DOWN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ERR_BADWRITE` reader - When 1, writes to any alias other than 0x0 (caching, allocating)  
 will produce a bus fault. When 0, these writes are silently ignored.  
 In either case, writes to the 0x0 alias will deallocate on tag match,  
 as usual."]
pub struct ERR_BADWRITE_R(crate::FieldReader<bool, bool>);
impl ERR_BADWRITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_BADWRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_BADWRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_BADWRITE` writer - When 1, writes to any alias other than 0x0 (caching, allocating)  
 will produce a bus fault. When 0, these writes are silently ignored.  
 In either case, writes to the 0x0 alias will deallocate on tag match,  
 as usual."]
pub struct ERR_BADWRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_BADWRITE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EN` reader - When 1, enable the cache. When the cache is disabled, all XIP accesses  
 will go straight to the flash, without querying the cache. When enabled,  
 cacheable XIP accesses will query the cache, and the flash will  
 not be accessed if the tag matches and the valid bit is set.  

 If the cache is enabled, cache-as-SRAM accesses have no effect on the  
 cache data RAM, and will produce a bus error response."]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - When 1, enable the cache. When the cache is disabled, all XIP accesses  
 will go straight to the flash, without querying the cache. When enabled,  
 cacheable XIP accesses will query the cache, and the flash will  
 not be accessed if the tag matches and the valid bit is set.  

 If the cache is enabled, cache-as-SRAM accesses have no effect on the  
 cache data RAM, and will produce a bus error response."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - When 1, the cache memories are powered down. They retain state,  
 but can not be accessed. This reduces static power dissipation.  
 Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot  
 be enabled when powered down.  
 Cache-as-SRAM accesses will produce a bus error response when  
 the cache is powered down."]
    #[inline(always)]
    pub fn power_down(&self) -> POWER_DOWN_R {
        POWER_DOWN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, writes to any alias other than 0x0 (caching, allocating)  
 will produce a bus fault. When 0, these writes are silently ignored.  
 In either case, writes to the 0x0 alias will deallocate on tag match,  
 as usual."]
    #[inline(always)]
    pub fn err_badwrite(&self) -> ERR_BADWRITE_R {
        ERR_BADWRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - When 1, enable the cache. When the cache is disabled, all XIP accesses  
 will go straight to the flash, without querying the cache. When enabled,  
 cacheable XIP accesses will query the cache, and the flash will  
 not be accessed if the tag matches and the valid bit is set.  

 If the cache is enabled, cache-as-SRAM accesses have no effect on the  
 cache data RAM, and will produce a bus error response."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - When 1, the cache memories are powered down. They retain state,  
 but can not be accessed. This reduces static power dissipation.  
 Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot  
 be enabled when powered down.  
 Cache-as-SRAM accesses will produce a bus error response when  
 the cache is powered down."]
    #[inline(always)]
    pub fn power_down(&mut self) -> POWER_DOWN_W {
        POWER_DOWN_W { w: self }
    }
    #[doc = "Bit 1 - When 1, writes to any alias other than 0x0 (caching, allocating)  
 will produce a bus fault. When 0, these writes are silently ignored.  
 In either case, writes to the 0x0 alias will deallocate on tag match,  
 as usual."]
    #[inline(always)]
    pub fn err_badwrite(&mut self) -> ERR_BADWRITE_W {
        ERR_BADWRITE_W { w: self }
    }
    #[doc = "Bit 0 - When 1, enable the cache. When the cache is disabled, all XIP accesses  
 will go straight to the flash, without querying the cache. When enabled,  
 cacheable XIP accesses will query the cache, and the flash will  
 not be accessed if the tag matches and the valid bit is set.  

 If the cache is enabled, cache-as-SRAM accesses have no effect on the  
 cache data RAM, and will produce a bus error response."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
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
}
#[doc = "`reset()` method sets CTRL to value 0x03"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
