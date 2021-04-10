package com.vinsguru.webfluxsse.producer;

import lombok.Data;

import java.io.Serializable;

//@Data
public class Joke implements Serializable {

    private static final String JOKE_FORMAT = "Q: %s \nA: %s";

    private String setup;
    private String punchline;

    public Joke() {
    }

    public Joke(final String setup, final String punchline) {
        this.setup = setup;
        this.punchline = punchline;
    }

    public String getSetup() {
        return setup;
    }

    public void setSetup(final String setup) {
        this.setup = setup;
    }

    public String getPunchline() {
        return punchline;
    }

    public void setPunchline(final String punchline) {
        this.punchline = punchline;
    }

    @Override
    public String toString() {
        return String.format(JOKE_FORMAT, this.setup, this.punchline);
    }

    @Override
    public boolean equals(final Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }

        Joke joke = (Joke) o;

        if (setup != null ? !setup.equals(joke.setup) : joke.setup != null) {
            return false;
        }
        return punchline != null ? punchline.equals(joke.punchline) : joke.punchline == null;
    }

    @Override
    public int hashCode() {
        int result = setup != null ? setup.hashCode() : 0;
        result = 31 * result + (punchline != null ? punchline.hashCode() : 0);
        return result;
    }
}
