"use strict";

requirejs.config({
	paths: {
		knockout: "https://cdnjs.cloudflare.com/ajax/libs/knockout/3.4.2/knockout-min",
		moment: "https://cdnjs.cloudflare.com/ajax/libs/moment.js/2.22.2/moment.min",
		reqwest: "https://cdnjs.cloudflare.com/ajax/libs/reqwest/2.0.5/reqwest.min",
		vega: "https://cdnjs.cloudflare.com/ajax/libs/vega/5.11.1/vega.min",
		"vega-embed": "https://cdnjs.cloudflare.com/ajax/libs/vega-embed/6.7.1/vega-embed.min",
		"vega-lite": "https://cdnjs.cloudflare.com/ajax/libs/vega-lite/4.11.0/vega-lite.min",
	},
	shim: {
		reqwest: {
			exports: "reqwest",
		},
		"vega-embed": {
			deps: ["vega", "vega-lite"],
		},
	},
	waitSeconds: 15,
});

// Start the main application logic.
requirejs(
	["knockout", "Application"],
	function(ko, Application) {
		const application = new Application();

		ko.applyBindings(application);
	},
	function(err) {
		console.log(err.requireType);

		if (err.requireType === "timeout") {
			console.log("modules: " + err.requireModules);
		}

		throw err;
	}
);
