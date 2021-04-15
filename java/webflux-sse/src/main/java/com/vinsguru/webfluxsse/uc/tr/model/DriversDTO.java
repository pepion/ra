package com.vinsguru.webfluxsse.uc.tr.model;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.List;

public class DriversDTO {

	@JsonProperty("driver_content")
	private List<Driver> drivers;

	private String lang;

	public List<Driver> getDrivers() {
		return drivers;
	}

	public void setDrivers(List<Driver> drivers) {
		this.drivers = drivers;
	}

	public String getLang() {
		return lang;
	}

	public void setLang(final String lang) {
		this.lang = lang;
	}
}
