use reqwest::Response;
use std::{future::Future, io, pin::pin, task::Poll};
use tokio::io::AsyncRead;
#[derive(Default)]
pub struct ResponseReader {
    inner: Option<Response>,
    buf: Vec<u8>,
}

impl std::fmt::Debug for ResponseReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            Some(res) => res.fmt(f),
            _ => f.write_str("None"),
        }
    }
}

impl ResponseReader {
    pub fn new(inner: Option<Response>) -> ResponseReader {
        Self {
            inner,
            ..Default::default()
        }
    }
}

impl AsyncRead for ResponseReader {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        let this = self.get_mut();
        if let Some(res) = &mut this.inner {
            loop {
                let chunk = res.chunk();
                match Future::poll(pin!(chunk), cx) {
                    Poll::Ready(Ok(bytes)) => {
                        if let Some(bytes) = bytes {
                            this.buf.extend_from_slice(&bytes.slice(0..bytes.len()));
                            if this.buf.len() >= buf.capacity() {
                                buf.put_slice(&this.buf[..buf.capacity()]);
                                this.buf = this.buf[buf.capacity()..].to_owned();
                                return Poll::Ready(Ok(()));
                            }
                        } else {
                            buf.put_slice(&this.buf);
                            this.buf.clear();
                            return Poll::Ready(Ok(()));
                        }
                    }
                    Poll::Ready(Err(err)) => {
                        return Poll::Ready(Err(io::Error::new(
                            std::io::ErrorKind::Other,
                            err.to_string(),
                        )));
                    }
                    Poll::Pending => return Poll::Pending,
                }
            }
        } else {
            Poll::Pending
        }
    }
}
