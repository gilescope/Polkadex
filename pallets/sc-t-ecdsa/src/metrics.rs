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
