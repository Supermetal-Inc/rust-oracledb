//-----------------------------------------------------------------------------
// Copyright (c) 2026, Oracle and/or its affiliates.
//
// This software is dual-licensed to you under the Universal Permissive License
// (UPL) 1.0 as shown at https://oss.oracle.com/licenses/upl and Apache License
// 2.0 as shown at http://www.apache.org/licenses/LICENSE-2.0. You may choose
// either license.
//
// If you elect to accept the software under the Apache License, Version 2.0,
// the following applies:
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// mod.rs
//
// Submodule for connections.
//-----------------------------------------------------------------------------

use std::net::{Shutdown, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::Error;

/// Cancels a connection by shutting down its transport. Obtain this handle before
/// starting a blocking call. Cancellation is permanent: the connection cannot be
/// reused, and pooled connections are discarded. After the connection is returned
/// to its pool or closed, this handle has no effect on subsequent borrowers.
#[derive(Clone)]
pub struct CancellationHandle {
    socket: Arc<Mutex<Option<TcpStream>>>,
    cancelled: Arc<AtomicBool>,
}

impl CancellationHandle {
    pub(crate) fn new(socket: TcpStream, cancelled: Arc<AtomicBool>) -> Self {
        Self { socket: Arc::new(Mutex::new(Some(socket))), cancelled }
    }

    /// Interrupts blocked I/O without acquiring the connection's client mutex.
    /// Repeated cancellation, including after the connection closes, is harmless.
    pub fn cancel(&self) -> Result<(), Error> {
        if let Some(socket) = self.socket.lock()?.take() {
            self.cancelled.store(true, Ordering::Release);
            socket.shutdown(Shutdown::Both)?;
        }
        Ok(())
    }

    pub(crate) fn invalidate(&self) -> Result<(), Error> {
        self.socket.lock()?.take();
        Ok(())
    }
}
