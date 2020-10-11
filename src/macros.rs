#[macro_export]
macro_rules! transitive_from {
	(#[$root:ty][$child:ty][
		$($grandchild:ty $({
			$($further:tt)*
		})?),* $(,)?
	]) => {
		$(
			$(transitive_from!{#
				[$root][$child][
					$($further)*
				]
			})?
			impl From<$grandchild> for $root {
				fn from(g: $grandchild) -> Self {
					<$root>::from(<$child>::from(g))
				}
			}
		)*
	};
	($($root:ty $({
		$($child:ty $({
			$($grandchild:tt)*
		})?),* $(,)?
	})?),* $(,)?) => {
		$($(
			transitive_from!{
				$($child $({
					$($grandchild)*
				})?),*
			}
			$($(
				transitive_from!{#
					[$root][$child][
						$($grandchild)*
					]
				}
			)?)*
		)?)*
		
	};
}
