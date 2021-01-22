#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `POWER_DOWN`"]
pub type POWER_DOWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POWER_DOWN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ERR_BADWRITE`"]
pub type ERR_BADWRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR_BADWRITE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - When 1, the cache memories are powered down. They retain state,\\n but can not be accessed. This reduces static power dissipation.\\n Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot\\n be enabled when powered down.\\n Cache-as-SRAM accesses will produce a bus error response when\\n the cache is powered down."]
    #[inline(always)]
    pub fn power_down(&self) -> POWER_DOWN_R {
        POWER_DOWN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, writes to any alias other than 0x0 (caching, allocating)\\n will produce a bus fault. When 0, these writes are silently ignored.\\n In either case, writes to the 0x0 alias will deallocate on tag match,\\n as usual."]
    #[inline(always)]
    pub fn err_badwrite(&self) -> ERR_BADWRITE_R {
        ERR_BADWRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - When 1, enable the cache. When the cache is disabled, all XIP accesses\\n will go straight to the flash, without querying the cache. When enabled,\\n cacheable XIP accesses will query the cache, and the flash will\\n not be accessed if the tag matches and the valid bit is set.\\n\\n If the cache is enabled, cache-as-SRAM accesses have no effect on the\\n cache data RAM, and will produce a bus error response."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - When 1, the cache memories are powered down. They retain state,\\n but can not be accessed. This reduces static power dissipation.\\n Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot\\n be enabled when powered down.\\n Cache-as-SRAM accesses will produce a bus error response when\\n the cache is powered down."]
    #[inline(always)]
    pub fn power_down(&mut self) -> POWER_DOWN_W {
        POWER_DOWN_W { w: self }
    }
    #[doc = "Bit 1 - When 1, writes to any alias other than 0x0 (caching, allocating)\\n will produce a bus fault. When 0, these writes are silently ignored.\\n In either case, writes to the 0x0 alias will deallocate on tag match,\\n as usual."]
    #[inline(always)]
    pub fn err_badwrite(&mut self) -> ERR_BADWRITE_W {
        ERR_BADWRITE_W { w: self }
    }
    #[doc = "Bit 0 - When 1, enable the cache. When the cache is disabled, all XIP accesses\\n will go straight to the flash, without querying the cache. When enabled,\\n cacheable XIP accesses will query the cache, and the flash will\\n not be accessed if the tag matches and the valid bit is set.\\n\\n If the cache is enabled, cache-as-SRAM accesses have no effect on the\\n cache data RAM, and will produce a bus error response."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
