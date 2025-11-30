pub mod csharp;
pub mod detector;
pub mod generic;
pub mod project;

#[allow(unused_imports)]
pub use csharp::CSharpAnalyzer;
#[allow(unused_imports)]
pub use detector::ProjectDetector;
pub use generic::GenericAnalyzer;
#[allow(unused_imports)]
pub use project::ProjectAnalyzer;
