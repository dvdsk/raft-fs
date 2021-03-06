use tokio::net::TcpStream;
use tokio_serde::formats::Bincode;
use tokio_util::codec::{Framed, LengthDelimitedCodec};


type FrameType = Framed<TcpStream, LengthDelimitedCodec>;
type Codec<I, O> = Bincode<I, O>;

/// A stream recieving: I: incoming item, O: outgoing
pub type MsgStream<I, O> = tokio_serde::Framed<FrameType, I, O, Codec<I, O>>;

/// I: incoming item, O: outgoing
pub fn wrap<I, O>(socket: TcpStream) -> MsgStream<I, O> {
    let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());

    let deserialized_stream =
        tokio_serde::Framed::new(length_delimited, Bincode::<I, O>::default());

    deserialized_stream
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Request, Response};

    #[tokio::test]
    async fn it_compiles() {
        use futures::SinkExt;

        let tcp = TcpStream::connect("127.0.0.1:1234").await.unwrap();
        let mut msgs: MsgStream<Response, Request> = wrap(tcp);

        msgs.send(Request::Test).await.unwrap();
    }
}
