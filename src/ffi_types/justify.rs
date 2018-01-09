use internal;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Justify {
	FlexStart = 0,
	Center = 1,
	FlexEnd = 2,
	SpaceBetween = 3,
	SpaceAround = 4,
}

impl From<Justify> for internal::YGJustify {
	fn from(j: Justify) -> internal::YGJustify {
		match j {
			Justify::FlexStart => internal::YGJustify::YGJustifyFlexStart,
			Justify::Center => internal::YGJustify::YGJustifyCenter,
			Justify::FlexEnd => internal::YGJustify::YGJustifyFlexEnd,
			Justify::SpaceBetween => internal::YGJustify::YGJustifySpaceBetween,
			Justify::SpaceAround => internal::YGJustify::YGJustifySpaceAround,
		}
	}
}

impl From<internal::YGJustify> for Justify {
	fn from(j: internal::YGJustify) -> Justify {
		match j {
			internal::YGJustify::YGJustifyFlexStart => Justify::FlexStart,
			internal::YGJustify::YGJustifyCenter => Justify::Center,
			internal::YGJustify::YGJustifyFlexEnd => Justify::FlexEnd,
			internal::YGJustify::YGJustifySpaceBetween => Justify::SpaceBetween,
			internal::YGJustify::YGJustifySpaceAround => Justify::SpaceAround,
		}
	}
}