#[cfg(test)]
mod tests {
	// The point of these tests is to check how long it takes to invoke a function dynamically
	// vs statically. We'll use sustenet_shared::ServerPlugin as the trait to test against.

	const MAX_INVOKES: usize = 1_000_000_000;

	/// The function to invoke statically.
	/// Static invoke took: 4.2954641s ~ 9% faster than dynamic invoke.
	#[test]
	#[ignore]
	fn test_static_invoke() {
		let mut count = 0;
		let start = std::time::Instant::now();
		for _ in 0..MAX_INVOKES {
			count_invoke(&mut count);
		}
		let duration = start.elapsed();
		println!("Static invoke took: {:?}", duration);
		assert_eq!(count, MAX_INVOKES as u32);
	}

	fn count_invoke(count: &mut u32) {
		*count += 1;
	}

	/// The function to invoke dynamically.
	/// Dynamic invoke took: 4.6038441s
	#[test]
	#[ignore]
	fn test_dynamic_invoke() {
		let mut count = 0;
		let plugin: Box<dyn DynPlugin> = Box::new(TestPlugin);
		let start = std::time::Instant::now();
		for _ in 0..MAX_INVOKES {
			plugin.count_invoke(&mut count);
		}
		let duration = start.elapsed();
		println!("Dynamic invoke took: {:?}", duration);
		assert_eq!(count, MAX_INVOKES as u32);
	}

	pub trait DynPlugin: Send + Sync {
		fn count_invoke(&self, count: &mut u32);
	}
	struct TestPlugin;
	impl DynPlugin for TestPlugin {
		fn count_invoke(&self, count: &mut u32) {
			*count += 1;
		}
	}
}