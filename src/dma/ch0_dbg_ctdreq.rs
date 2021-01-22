#[doc = "Reader of register CH0_DBG_CTDREQ"]
pub type R = crate::R<u32, super::CH0_DBG_CTDREQ>;
#[doc = "Reader of field `CH0_DBG_CTDREQ`"]
pub type CH0_DBG_CTDREQ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch0_dbg_ctdreq(&self) -> CH0_DBG_CTDREQ_R {
        CH0_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}
