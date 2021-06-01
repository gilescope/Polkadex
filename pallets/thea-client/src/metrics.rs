// Copyright (C) 2020-2021 Polkadex OU
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// This is file is modified from beefy-gadget from Parity Technologies (UK) Ltd.
use prometheus::{register, Counter, Gauge, PrometheusError, Registry, U64};

/// THEA metrics exposed through Prometheus
pub(crate) struct Metrics {
    /// Current active validator set id
    pub thea_validator_set_id: Gauge<U64>,
    /// Total number of votes sent by this node
    pub thea_votes_sent: Counter<U64>,
    /// Most recent concluded voting round
    pub thea_round_concluded: Gauge<U64>,
}

impl Metrics {
    pub(crate) fn register(registry: &Registry) -> Result<Self, PrometheusError> {
        Ok(Self {
            thea_validator_set_id: register(
                Gauge::new(
                    "thea_validator_set_id",
                    "Current THEA active validator set id.",
                )?,
                registry,
            )?,
            thea_votes_sent: register(
                Counter::new("thea_votes_sent", "Number of votes sent by this node")?,
                registry,
            )?,
            thea_round_concluded: register(
                Gauge::new(
                    "thea_round_concluded",
                    "Voting round, that has been concluded",
                )?,
                registry,
            )?,
        })
    }
}

// Note: we use the `format` macro to convert an expr into a `u64`. This will fail,
// if expr does not derive `Display`.
#[macro_export]
macro_rules! metric_set {
    ($self:ident, $m:ident, $v:expr) => {{
        let val: u64 = format!("{}", $v).parse().unwrap();

        if let Some(metrics) = $self.metrics.as_ref() {
            metrics.$m.set(val);
        }
    }};
}

#[macro_export]
macro_rules! metric_inc {
    ($self:ident, $m:ident) => {{
        if let Some(metrics) = $self.metrics.as_ref() {
            metrics.$m.inc();
        }
    }};
}