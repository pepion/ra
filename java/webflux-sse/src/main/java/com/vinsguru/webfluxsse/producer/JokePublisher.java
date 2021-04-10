package com.vinsguru.webfluxsse.producer;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Service;
import org.springframework.web.reactive.function.client.WebClient;
import reactor.core.publisher.Sinks;

import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

@Service
public class JokePublisher {

    @Autowired
    private WebClient webClient;

    @Autowired
    private Sinks.Many<Joke> sink;

    @Scheduled(fixedRate = 3600000)
    public void publish(){
        this.webClient
                .get()
                .retrieve()
                .bodyToMono(Joke.class)
                .subscribe(joke-> {
                    LocalDateTime dateTime = LocalDateTime.now();
                    DateTimeFormatter formatter = DateTimeFormatter.ofPattern("dd-MM-yyyy HH:mm:ss");
                    System.out.println(dateTime.format(formatter)+"::"+ joke.toString());
                    this.sink.tryEmitNext(joke);
                });
    }

}
