// Copyright 2018 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::connection::ConnectionLimit;
use crate::transport::TransportError;
use crate::Multiaddr;
use std::{fmt, io};

/// Errors that can occur in the context of an established `Connection`.
#[derive(Debug)]
pub enum ConnectionError<THandlerErr> {
    /// An I/O error occurred on the connection.
    // TODO: Eventually this should also be a custom error?
    IO(io::Error),

    /// The connection handler produced an error.
    Handler(THandlerErr),
}

impl<THandlerErr> fmt::Display for ConnectionError<THandlerErr>
where
    THandlerErr: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::IO(err) => write!(f, "Connection error: I/O error: {}", err),
            ConnectionError::Handler(err) => write!(f, "Connection error: Handler error: {}", err),
        }
    }
}

impl<THandlerErr> std::error::Error for ConnectionError<THandlerErr>
where
    THandlerErr: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConnectionError::IO(err) => Some(err),
            ConnectionError::Handler(err) => Some(err),
        }
    }
}

/// Errors that can occur in the context of a pending outgoing `Connection`.
///
/// Note: Addresses for an outbound connection are dialed in parallel. Thus, compared to
/// [`PendingInboundConnectionError`], one or more [`TransportError`]s can occur for a single
/// connection.
pub type PendingOutboundConnectionError<TTransErr> =
    PendingConnectionError<Vec<(Multiaddr, TransportError<TTransErr>)>>;

/// Errors that can occur in the context of a pending incoming `Connection`.
pub type PendingInboundConnectionError<TTransErr> =
    PendingConnectionError<TransportError<TTransErr>>;

/// Errors that can occur in the context of a pending `Connection`.
#[derive(Debug)]
pub enum PendingConnectionError<TTransErr> {
    /// An error occurred while negotiating the transport protocol(s) on a connection.
    Transport(TTransErr),

    /// The connection was dropped because the connection limit
    /// for a peer has been reached.
    ConnectionLimit(ConnectionLimit),

    /// Pending connection attempt has been aborted.
    Aborted,

    /// The peer identity obtained on the connection did not
    /// match the one that was expected or is otherwise invalid.
    InvalidPeerId,

    /// An I/O error occurred on the connection.
    // TODO: Eventually this should also be a custom error?
    IO(io::Error),
}

impl<TTransErr> fmt::Display for PendingConnectionError<TTransErr>
where
    TTransErr: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PendingConnectionError::IO(err) => write!(f, "Pending connection: I/O error: {}", err),
            PendingConnectionError::Aborted => write!(f, "Pending connection: Aborted."),
            PendingConnectionError::Transport(err) => {
                write!(
                    f,
                    "Pending connection: Transport error on connection: {}",
                    err
                )
            }
            PendingConnectionError::ConnectionLimit(l) => {
                write!(f, "Connection error: Connection limit: {}.", l)
            }
            PendingConnectionError::InvalidPeerId => {
                write!(f, "Pending connection: Invalid peer ID.")
            }
        }
    }
}

impl<TTransErr> std::error::Error for PendingConnectionError<TTransErr>
where
    TTransErr: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PendingConnectionError::IO(err) => Some(err),
            PendingConnectionError::Transport(_) => None,
            PendingConnectionError::InvalidPeerId => None,
            PendingConnectionError::Aborted => None,
            PendingConnectionError::ConnectionLimit(..) => None,
        }
    }
}
