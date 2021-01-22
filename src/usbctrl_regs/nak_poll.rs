#[doc = "Reader of register NAK_POLL"]
pub type R = crate::R<u32, super::NAK_POLL>;
#[doc = "Writer for register NAK_POLL"]
pub type W = crate::W<u32, super::NAK_POLL>;
#[doc = "Register NAK_POLL `reset()`'s with value 0x0010_0010"]
impl crate::ResetValue for super::NAK_POLL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0010
    }
}
#[doc = "Reader of field `DELAY_FS`"]
pub type DELAY_FS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DELAY_FS`"]
pub struct DELAY_FS_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_FS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DELAY_LS`"]
pub type DELAY_LS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DELAY_LS`"]
pub struct DELAY_LS_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_LS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:25 - NAK polling interval for a full speed device"]
    #[inline(always)]
    pub fn delay_fs(&self) -> DELAY_FS_R {
        DELAY_FS_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - NAK polling interval for a low speed device"]
    #[inline(always)]
    pub fn delay_ls(&self) -> DELAY_LS_R {
        DELAY_LS_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - NAK polling interval for a full speed device"]
    #[inline(always)]
    pub fn delay_fs(&mut self) -> DELAY_FS_W {
        DELAY_FS_W { w: self }
    }
    #[doc = "Bits 0:9 - NAK polling interval for a low speed device"]
    #[inline(always)]
    pub fn delay_ls(&mut self) -> DELAY_LS_W {
        DELAY_LS_W { w: self }
    }
}
