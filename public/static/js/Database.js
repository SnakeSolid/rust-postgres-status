"use strict";

define(["knockout", "moment"], function(ko, moment) {
	const Database = function(params) {
		this.name = ko.observable(params["name"]);
		this.modified = ko.observable(params["modified"]);
		this.size = ko.observable(params["size"]);

		this.sizeHuman = ko.pureComputed(function() {
			const factors = [ "B","KiB","MiB","GiB","TiB" ];
			let size = this.size();
			let index = 0;

			while (size > 1024 && index < factors.length) {
				size /= 1024;
				index += 1; 
			}

			return size.toFixed(1) + " " + factors[index];
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
