package com.vinsguru.webfluxsse.uc.tr;

import com.vinsguru.webfluxsse.producer.Joke;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;
import reactor.core.publisher.Flux;

import java.time.LocalDateTime;

@RestController
public class TrController {
    @Autowired
    private TrService trService;

    @Autowired
    private Flux<Joke> trFlux;

    @GetMapping(value = "/tr/{name}", produces = MediaType.TEXT_EVENT_STREAM_VALUE)
    public Flux<Joke> getTr(@PathVariable final String name){
        LocalDateTime start = LocalDateTime.now();
        return trService.importDrivers(start);
        //return trFlux.doOnNext(next-> System.out.println("xxxxxxxx--> " + name + " --> " + Thread.currentThread().getName()));
    }
}
