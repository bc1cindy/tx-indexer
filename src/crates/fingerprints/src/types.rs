#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSortingType {
    Single,
    Ascending,
    Descending,
    Bip69,
    Historical,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputStructureType {
    Single,
    Double,
    Multi,
    Bip69,
}
