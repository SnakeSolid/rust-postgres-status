"use strict";

define(["knockout", "reqwest", "moment", "vega", "vega-embed", "Database", "Util"], function(
	ko,
	reqwest,
	moment,
	vega,
	VegaEmbed,
	Database,
	Util
) {
	const CHART_SCHEMA = {
		$schema: "https://vega.github.io/schema/vega-lite/v4.json",
		description: "A simple pie chart with labels.",
		width: "container",
		data: {
			name: "table",
			values: [{ user: "<undefined>", size: 0 }],
		},
		transform: [
			{
				aggregate: [{ op: "sum", field: "size", as: "size" }],
				groupby: ["user"],
			},
			{
				window: [{ op: "sum", field: "size", as: "total_size" }],
				frame: [null, null],
			},
			{
				calculate: "datum.size / datum.total_size * 100",
				as: "percent",
			},
			{
				filter: "datum.percent > 1.0",
			},
		],
		mark: "bar",
		encoding: {
			y: {
				field: "user",
				type: "ordinal",
				axis: { title: "User name", labelFontSize: 14, "tickSize": 30, },
				sort: "-x",
			},
			x: {
				field: "percent",
				type: "quantitative",
				axis: { title: "Used database size (%)" },
			},
		},
	};

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
		this.isChartAvailable = ko.observable(false);
		this.isChartRequired = ko.observable(true);

		// Create Vega chart if it's possible.
		this.chartView = null;
		this.chart = VegaEmbed("#chart", CHART_SCHEMA).then(res => (this.chartView = res.view));

		this.isSuccess = ko.pureComputed(function() {
			return !this.isError();
		}, this);

		this.isChartVisible = ko.pureComputed(function() {
			return this.isChartAvailable() && this.isChartRequired();
		}, this);

		this.isShowChartVisible = ko.pureComputed(function() {
			return !this.isChartRequired();
		}, this);

		this.isHideChartVisible = ko.pureComputed(function() {
			return this.isChartRequired();
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
			if (this.diskCapacity() > 0) {
				return (100.0 * this.diskUsed()) / this.diskCapacity();
			} else {
				return 0;
			}
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
			const name = database.name();
			const confirmed = window.confirm(
				"Database `" + name + "` will be dropped. All active connections will be closed. Are you sure?"
			);

			if (confirmed) {
				reqwest({
					url: "/api/v1/dropdb",
					type: "json",
					method: "POST",
					contentType: "application/json",
					data: JSON.stringify({
						name: name,
					}),
				})
					.then(
						function(resp) {
							if (resp.success) {
								this.databases.remove(database);
								this.diskUsed(this.diskUsed() - database.size());
								this.updateChart();
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
			}
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

	Application.prototype.toggleChart = function() {
		this.isChartRequired(!this.isChartRequired());

		if (this.isChartVisible()) {
			// Force to update chart.
			window.dispatchEvent(new Event("resize"));
		}
	};

	Application.prototype.updateChart = function() {
		if (this.chartView === null) {
			return;
		}

		const dataset = this.databases()
			.filter(row => row.isNotService())
			.map(row => ({ user: row.user(), size: row.size() }));
		const hasData = dataset.length > 0;

		if (hasData) {
			this.chartView
				.change(
					"table",
					vega
						.changeset()
						.remove(row => true)
						.insert(dataset)
				)
				.runAsync()
				.then(() => window.dispatchEvent(new Event("resize")));
		}

		this.isChartAvailable(hasData);
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
						this.updateChart();
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
