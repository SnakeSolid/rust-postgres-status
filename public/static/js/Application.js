"use strict";

define(["knockout", "reqwest", "moment", "Database", "Util"], function(ko, reqwest, moment, Database, Util) {
	const SORT_NAME = "SortName";
	const SORT_USER = "SortUser";
	const SORT_SIZE = "SortSize";
	const SORT_MODIFIED = "SortModified";

	const ORDER_ASC = "OrderAsc";
	const ORDER_DESC = "OrderDesc";

	const Application = function() {
		this.diskUsed = ko.observable(0.0);
		this.diskCapacity = ko.observable(0.0);
		this.softThreshold = ko.observable(0.0);
		this.hardThreshold = ko.observable(0.0);
		this.sortColumn = ko.observable(SORT_MODIFIED);
		this.sortOrder = ko.observable(ORDER_ASC);
		this.databases = ko.observableArray([]);
		this.isLoading = ko.observable(false);
		this.isError = ko.observable(false);
		this.errorMessage = ko.observable();

		this.isSuccess = ko.pureComputed(function() {
			return !this.isError();
		}, this);

		this.progressState = ko.pureComputed(function() {
			const result = {};

			result["success"] = this.diskUsed() < this.softThreshold();
			result["warning"] = this.diskUsed() >= this.softThreshold() && this.diskUsed() < this.hardThreshold();
			result["error"] = this.diskUsed() >= this.hardThreshold();

			return result;
		}, this);

		this.cssForName = ko.pureComputed(function() {
			const isColumnMatches = this.sortColumn() === SORT_NAME;
			const result = {};

			result["sorted"] = isColumnMatches;
			result["ascending"] = isColumnMatches && this.sortOrder() === ORDER_ASC;
			result["descending"] = isColumnMatches && this.sortOrder() === ORDER_DESC;

			return result;
		}, this);

		this.cssForUser = ko.pureComputed(function() {
			const isColumnMatches = this.sortColumn() === SORT_USER;
			const result = {};

			result["sorted"] = isColumnMatches;
			result["ascending"] = isColumnMatches && this.sortOrder() === ORDER_ASC;
			result["descending"] = isColumnMatches && this.sortOrder() === ORDER_DESC;

			return result;
		}, this);

		this.cssForSize = ko.pureComputed(function() {
			const isColumnMatches = this.sortColumn() === SORT_SIZE;
			const result = {};

			result["sorted"] = isColumnMatches;
			result["ascending"] = isColumnMatches && this.sortOrder() === ORDER_ASC;
			result["descending"] = isColumnMatches && this.sortOrder() === ORDER_DESC;

			return result;
		}, this);

		this.cssForModified = ko.pureComputed(function() {
			const isColumnMatches = this.sortColumn() === SORT_MODIFIED;
			const result = {};

			result["sorted"] = isColumnMatches;
			result["ascending"] = isColumnMatches && this.sortOrder() === ORDER_ASC;
			result["descending"] = isColumnMatches && this.sortOrder() === ORDER_DESC;

			return result;
		}, this);

		this.diskUsedHuman = ko.pureComputed(function() {
			return Util.humanSize(this.diskUsed());
		}, this);

		this.diskCapacityHuman = ko.pureComputed(function() {
			return Util.humanSize(this.diskCapacity());
		}, this);

		this.loadPercent = ko.pureComputed(function() {
			return (100.0 * this.diskUsed()) / this.diskCapacity();
		}, this);

		this.loadPercentHuman = ko.pureComputed(function() {
			return this.loadPercent().toFixed(0);
		}, this);

		this.progressStyle = ko.pureComputed(function() {
			return {
				width: this.loadPercent().toFixed(0) + "%",
			};
		}, this);

		this.hasResults = ko.pureComputed(function() {
			return this.databases().length > 0;
		}, this);

		this.dropDatabase = function(database) {
			reqwest({
				url: "/api/v1/dropdb",
				type: "json",
				method: "POST",
				contentType: "application/json",
				data: JSON.stringify({
					name: database.name(),
				}),
			})
				.then(
					function(resp) {
						if (resp.success) {
							this.databases.remove(database);
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

			this.isLoading(true);
			this.isError(false);
			this.errorMessage("");
		}.bind(this);

		this.updateState();
	};

	Application.prototype.updateSortHeader = function(sortField) {
		if (this.sortColumn() !== sortField) {
			this.sortColumn(sortField);
			this.sortOrder(ORDER_DESC);
		} else if (this.sortOrder() === ORDER_ASC) {
			this.sortOrder(ORDER_DESC);
		} else if (this.sortOrder() === ORDER_DESC) {
			this.sortOrder(ORDER_ASC);
		}
	};

	Application.prototype.sortDatabases = function(sortField) {
		let sortOrder;

		if (this.sortOrder() == ORDER_DESC) {
			sortOrder = 1;
		} else {
			sortOrder = -1;
		}

		if (this.sortColumn() == SORT_NAME) {
			this.databases.sort(Util.sortBy("name", sortOrder));
		} else if (this.sortColumn() == SORT_USER) {
			this.databases.sort(Util.sortBy("user", sortOrder));
		} else if (this.sortColumn() == SORT_SIZE) {
			this.databases.sort(Util.sortBy("size", sortOrder));
		} else if (this.sortColumn() == SORT_MODIFIED) {
			this.databases.sort(Util.sortBy("modified", sortOrder));
		}
	};

	Application.prototype.sortByName = function() {
		this.updateSortHeader(SORT_NAME);
		this.sortDatabases();
	};

	Application.prototype.sortByUser = function() {
		this.updateSortHeader(SORT_USER);
		this.sortDatabases();
	};

	Application.prototype.sortBySize = function() {
		this.updateSortHeader(SORT_SIZE);
		this.sortDatabases();
	};

	Application.prototype.sortByModified = function() {
		this.updateSortHeader(SORT_MODIFIED);
		this.sortDatabases();
	};

	Application.prototype.forceUpdate = function() {
		reqwest({
			url: "/api/v1/update",
			type: "json",
			method: "POST",
		})
			.then(
				function(resp) {
					if (resp.success) {
						this.updateState();
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

		this.isLoading(true);
		this.isError(false);
		this.errorMessage("");
	};

	Application.prototype.updateState = function() {
		reqwest({
			url: "/api/v1/state",
			type: "json",
			method: "POST",
		})
			.then(
				function(resp) {
					if (resp.success) {
						this.diskUsed(resp.result.disk_used);
						this.diskCapacity(resp.result.disk_capacity);
						this.softThreshold(resp.result.soft_threshold);
						this.hardThreshold(resp.result.hard_threshold);
						this.databases(
							resp.result.databases.map(function(database) {
								return new Database(database);
							})
						);
						this.sortDatabases();
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

		this.isLoading(true);
		this.isError(false);
		this.errorMessage("");
	};

	return Application;
});
