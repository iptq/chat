use bytes::BytesMut;
use failure::Error;
use tokio_io::codec::Decoder;

pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = Vec<u8>;
    type Error = Error;

    fn decode(&mut self, _: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(None)
    }
}
