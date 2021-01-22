#[doc = "Reader of register CH7_DBG_CTDREQ"]
pub type R = crate::R<u32, super::CH7_DBG_CTDREQ>;
#[doc = "Reader of field `CH7_DBG_CTDREQ`"]
pub type CH7_DBG_CTDREQ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch7_dbg_ctdreq(&self) -> CH7_DBG_CTDREQ_R {
        CH7_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}
