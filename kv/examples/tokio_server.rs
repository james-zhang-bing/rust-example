use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::{CommandRequest, CommandResponse, Service, ServiceInner, SledDb, KvError};
use prost::Message;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let service: Service<SledDb> = ServiceInner::new(SledDb::new("/tmp/kvserver"))
        .fn_before_send(|res| match res.message.as_ref() {
            "" => res.message = "altered. Original message is empty.".into(),
            s => res.message = format!("altered: {}", s),
        })
        .into();
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            // let mut stream =
            //     AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
            let mut framed=Framed::new(stream,LengthDelimitedCodec::new());
            while let Some(Ok(cmd_bytes)) = framed.next().await {
                info!("Got a new command: {:?}", cmd_bytes);
               let cmd=CommandRequest::decode(cmd_bytes)?;

                let res = svc.execute(cmd).encode_to_vec().into();
                
                framed.send(res).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
            Ok::<(),KvError>(())
        });
    }
}