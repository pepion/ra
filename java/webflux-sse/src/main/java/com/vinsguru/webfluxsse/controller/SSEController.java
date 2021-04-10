package com.vinsguru.webfluxsse.controller;

import com.vinsguru.webfluxsse.producer.Joke;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;
import reactor.core.publisher.Flux;

@RestController
public class SSEController {

    @Autowired
    private Flux<Joke> flux;

    @GetMapping(value = "/joke/{name}", produces = MediaType.TEXT_EVENT_STREAM_VALUE)
    public Flux<Joke> getJoke(@PathVariable final String name){
        return flux.doOnNext(next-> System.out.println("xxxxxxxx--> " + name + " --> " + Thread.currentThread().getName()));
    }
}
