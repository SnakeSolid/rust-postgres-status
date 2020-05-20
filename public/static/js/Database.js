"use strict";

define(["knockout", "moment", "Util"], function(ko, moment, Util) {
	const Database = function(params) {
		this.service = ko.observable(params["service"]);
		this.name = ko.observable(params["name"]);
		this.user = ko.observable(params["user"] || "\u2014");
		this.modified = ko.observable(params["modified"]);
		this.size = ko.observable(params["size"]);

		this.isService = ko.pureComputed(function() {
			return this.service();
		}, this);

		this.isNotService = ko.pureComputed(function() {
			return !this.service();
		}, this);

		this.sizeHuman = ko.pureComputed(function() {
			return Util.humanSize(this.size());
		}, this);

		this.modifiedFormat = ko.pureComputed(function() {
			return moment.unix(this.modified()).format("YYYY.MM.DD HH:mm");
		}, this);

		this.modifiedFromNow = ko.pureComputed(function() {
			return moment.unix(this.modified()).fromNow(false);
		}, this);
	};

	return Database;
});
