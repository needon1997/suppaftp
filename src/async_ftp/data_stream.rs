//! # Data Stream
//!
//! This module exposes the async data stream implementation where bytes must be written to/read from

use std::pin::Pin;

use pin_project::pin_project;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf, Result};
use tokio::net::TcpStream;

use super::AsyncTlsStream;

/// Data Stream used for communications. It can be both of type Tcp in case of plain communication or Ssl in case of FTPS
#[pin_project(project = DataStreamProj)]
pub enum DataStream<T>
where
    T: AsyncTlsStream,
{
    Tcp(#[pin] TcpStream),
    Ssl(#[pin] Box<T>),
}

impl<T> DataStream<T>
where
    T: AsyncTlsStream,
{
    /// Unwrap the stream into TcpStream. This method is only used in secure connection.
    pub fn into_tcp_stream(self) -> TcpStream {
        match self {
            DataStream::Tcp(stream) => stream,
            DataStream::Ssl(stream) => stream.tcp_stream(),
        }
    }
}

impl<T> DataStream<T>
where
    T: AsyncTlsStream,
{
    /// Returns a reference to the underlying TcpStream.
    pub fn get_ref(&self) -> &TcpStream {
        match self {
            DataStream::Tcp(ref stream) => stream,
            DataStream::Ssl(ref stream) => stream.get_ref(),
        }
    }
}

// -- async

impl<T> AsyncRead for DataStream<T>
where
    T: AsyncTlsStream,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf,
    ) -> std::task::Poll<Result<()>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_read(cx, buf),
            DataStreamProj::Ssl(stream) => stream.poll_read(cx, buf),
        }
    }
}

impl<T> AsyncWrite for DataStream<T>
where
    T: AsyncTlsStream,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_write(cx, buf),
            DataStreamProj::Ssl(stream) => stream.poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<()>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_flush(cx),
            DataStreamProj::Ssl(stream) => stream.poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<()>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_shutdown(cx),
            DataStreamProj::Ssl(stream) => stream.poll_shutdown(cx),
        }
    }
}
