pub const STANDARD_RESOURCE_CATEGORIES: &[&str] = &[
    "ColorSpace",
    "ExtGState",
    "Font",
    "Pattern",
    "Properties",
    "Shading",
    "XObject",
    "ProcSet",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceCategory {
    ColorSpace,
    ExtGState,
    Font,
    Pattern,
    Properties,
    Shading,
    XObject,
}

impl ResourceCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceCategory::ColorSpace => "ColorSpace",
            ResourceCategory::ExtGState => "ExtGState",
            ResourceCategory::Font => "Font",
            ResourceCategory::Pattern => "Pattern",
            ResourceCategory::Properties => "Properties",
            ResourceCategory::Shading => "Shading",
            ResourceCategory::XObject => "XObject",
        }
    }

    pub fn category_prefix(category: ResourceCategory) -> &'static str {
        match category {
            ResourceCategory::ColorSpace => "CS",
            ResourceCategory::ExtGState => "GS",
            ResourceCategory::Font => "F",
            ResourceCategory::Pattern => "P",
            ResourceCategory::Properties => "Pr",
            ResourceCategory::Shading => "Sh",
            ResourceCategory::XObject => "Im",
        }
    }
}
