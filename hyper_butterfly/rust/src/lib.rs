//! HyperButterfly: 하이퍼볼릭 기하학을 위한 효율적인 Rust 라이브러리
//!
//! 이 라이브러리는 하이퍼볼릭 공간에서의 연산과
//! 버터플라이 팩터화를 통한 효율적인 변환을 제공합니다.

mod config;
mod manifolds;
mod maps;
mod ops;
mod utils;

#[cfg(feature = "cuda")]
mod cuda;

pub use manifolds::{
    dynamic_curvature::LearningCurvatureManifold, KleinModel, LorentzModel, Manifold, ManifoldType,
    PoincareBall,
};
pub use maps::{exp_map, geodesic, log_map};
pub use ops::{butterfly_transform, mobius_add, mobius_scalar};
pub use utils::numeric::{atanh, safe_tanh};

/// 자주 사용되는 핵심 기능들을 쉽게 가져올 수 있는 prelude 모듈
pub mod prelude {
    pub use crate::{
        atanh, butterfly_transform, exp_map, geodesic, log_map, mobius_add, mobius_scalar,
        safe_tanh, KleinModel, LearningCurvatureManifold, LorentzModel, Manifold, ManifoldType,
        PoincareBall,
    };
}
