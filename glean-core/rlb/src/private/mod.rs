// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The different metric types supported by the Glean SDK to handle data.

#[cfg(not(feature = "glean-dynamic"))]
mod types {
    pub mod timespan;
    pub mod counter;
    pub mod labeled;
    pub mod string;
    pub mod uuid;
}

#[cfg(feature = "glean-dynamic")]
mod types {
    #[path = "../dynamic/macros.rs"]
    #[macro_use]
    mod macros;

    #[path = "../dynamic/timespan.rs"]
    pub mod timespan;

    #[path = "../dynamic/counter.rs"]
    pub mod counter;

    #[path = "../dynamic/string.rs"]
    pub mod string;

    #[path = "../dynamic/uuid.rs"]
    pub mod uuid;

    #[path = "../dynamic/labeled.rs"]
    pub mod labeled;
}

mod boolean;
mod custom_distribution;
mod datetime;
mod event;
mod memory_distribution;
mod ping;
mod quantity;
mod recorded_experiment_data;
mod string_list;
mod timing_distribution;

pub use boolean::BooleanMetric;
pub use custom_distribution::CustomDistributionMetric;
pub use datetime::{Datetime, DatetimeMetric};
pub use event::EventMetric;
pub use memory_distribution::MemoryDistributionMetric;
pub use ping::PingType;
pub use quantity::QuantityMetric;
pub use recorded_experiment_data::RecordedExperimentData;
pub use string_list::StringListMetric;
pub use timing_distribution::TimingDistributionMetric;
pub use types::counter::CounterMetric;
pub use types::labeled::{AllowLabeled, LabeledMetric};
pub use types::string::StringMetric;
pub use types::timespan::TimespanMetric;
pub use types::uuid::UuidMetric;
