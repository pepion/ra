package com.vinsguru.webfluxsse.uc.tr.model;

public class Link {

	private String name;
	private String url;

	public Link() {

	}

	public Link(String name, String url) {
		this.name = name;
		this.url = url;
	}

	public String getName() {
		return name;
	}

	public void setName(String name) {
		this.name = name;
	}

	public String getUrl() {
		return url;
	}

	public void setUrl(String url) {
		this.url = url;
	}

	@Override
	public String toString() {
		return "Link [name=" + name + ", url=" + url + "]";
	}
}
