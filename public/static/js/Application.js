"use strict";

define(["knockout", "reqwest", "moment", "Database"], function(ko, reqwest, moment, Database) {
	const Application = function() {
		this.diskUsed = ko.observable(0.0);
		this.diskCapacity = ko.observable(0.0);
		this.softThreshold = ko.observable(0.0);
		this.hardThreshold = ko.observable(0.0);
		this.databases = ko.observableArray([]);
		this.isLoading = ko.observable(false);
		this.isError = ko.observable(false);
		this.errorMessage = ko.observable();

		this.loadPercent = ko.pureComputed(function() {
			return (100.0 * this.diskUsed()) / this.diskCapacity();
		}, this);

		this.progressStyle = ko.pureComputed(function() {
			return {
				width: this.loadPercent().toFixed(0) + "%",
			};
		}, this);

		this.hasResults = ko.pureComputed(function() {
			return this.databases().length > 0;
		}, this);

		this.updateState();
	};

	Application.prototype.updateState = function() {
		const self = this;
		const res = reqwest({
			url: "/api/v1/state",
			type: "json",
			method: "POST",
		})
			.then(
				function(resp) {
					if (resp.success) {
						const databases = resp.result.databases
							.map(function(database) {
								return new Database(database);
							})
							.sort(function(a, b) {
								return b.modified() - a.modified();
							});

						this.diskUsed(
							resp.result.databases.reduce(function(a, b) {
								return a.size + b.size;
							}, 0.0)
						);
						this.diskCapacity(resp.result.disk_capacity);
						this.softThreshold(resp.result.soft_threshold);
						this.hardThreshold(resp.result.hard_threshold);
						this.databases(databases);
					} else {
						this.isError(true);
						this.errorMessage(resp.message);
					}

					this.isLoading(false);
				}.bind(this)
			)
			.fail(
				function(err, msg) {
					this.isLoading(false);
					this.isError(true);
					this.errorMessage(msg || err.responseText);
				}.bind(this)
			);

		this.isLoading(false);
		this.isError(false);
		this.errorMessage("");
	};

	return Application;
});
