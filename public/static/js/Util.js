"use strict";

define(["exports"], function(exports) {
	exports.sortBy = function(name, order) {
		return function(a, b) {
			if (a[name]() === b[name]()) {
				return 0;
			} else if (a[name]() < b[name]()) {
				return -order;
			} else if (a[name]() > b[name]()) {
				return order;
			}
		};
	};

	exports.humanSize = function(value) {
		const factors = ["B", "KiB", "MiB", "GiB", "TiB"];
		let size = value;
		let index = 0;

		while (size > 1024 && index < factors.length) {
			size /= 1024;
			index += 1;
		}

		return size.toFixed(1) + " " + factors[index];
	};
});
