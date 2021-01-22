#[doc = "Reader of register FIFO"]
pub type R = crate::R<u32, super::FIFO>;
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `VAL`"]
pub type VAL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 15 - 1 if this particular sample experienced a conversion error. Remains in the same location if the sample is shifted."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x0fff) as u16)
    }
}
