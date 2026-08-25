/// Lightweight key representing `LCPEntry`
///
/// Used only to construct LCP
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LCPKey {
    String(String),
    Function(String),
    Class(String),
    Interface(String),
}
