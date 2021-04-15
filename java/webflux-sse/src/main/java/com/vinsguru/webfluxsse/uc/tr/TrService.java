package com.vinsguru.webfluxsse.uc.tr;

import com.vinsguru.webfluxsse.producer.Joke;
import com.vinsguru.webfluxsse.uc.tr.model.DriversDTO;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpHeaders;
import org.springframework.http.MediaType;
import org.springframework.http.client.reactive.ReactorClientHttpConnector;
import org.springframework.stereotype.Service;
import org.springframework.web.reactive.function.client.ExchangeStrategies;
import org.springframework.web.reactive.function.client.WebClient;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;
import reactor.core.publisher.Sinks;
import reactor.core.scheduler.Schedulers;
import reactor.netty.http.client.HttpClient;
import reactor.netty.http.client.HttpClientRequest;

import java.time.Duration;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.Arrays;
import java.util.List;

@Service
public class TrService {

    private static final String TR_API_ENDPOINT = "http://setbuilderapi.tridonic.com";
    private static final List<String> LANGS = Arrays.asList("en", "de", "fr", "es", "it", "sv", "cn", "ch", "pl", "mena");


//    val exchangeStrategies = ExchangeStrategies.builder()
//            .codecs { configurer: ClientCodecConfigurer -> configurer.defaultCodecs().maxInMemorySize(16 * 1024 * 1024) }.build()
//        return WebClient.builder().exchangeStrategies(exchangeStrategies).build()

    private final WebClient client = WebClient.builder()
            .baseUrl(TR_API_ENDPOINT)
            .defaultHeader(HttpHeaders.ACCEPT, MediaType.APPLICATION_JSON_VALUE)
            .defaultHeader(HttpHeaders.CONTENT_TYPE, MediaType.APPLICATION_JSON_VALUE)
            .exchangeStrategies(ExchangeStrategies.builder()
                    .codecs(configurer -> configurer.defaultCodecs().maxInMemorySize(256 * 1024 * 1024))
                    .build())
            .clientConnector(new ReactorClientHttpConnector(HttpClient.create()
                    .responseTimeout(Duration.ofSeconds(500))))
            .build();

    @Autowired
    private Sinks.Many<Joke> trSink;

    public Flux<Joke> importDrivers(final LocalDateTime start) {
        return Flux.fromIterable(LANGS)
                .parallel()
                .runOn(Schedulers.boundedElastic())
                .doOnNext(lang -> start(lang, start))
                .flatMap(lang -> getDrivers(lang, start))
                .sequential();
    }

    public Joke save(DriversDTO drivers, String lang, final LocalDateTime start) {
        Thread.getAllStackTraces().keySet()
                .stream()
                .map(t -> t.getName() + " daemon=" + t.isDaemon() + " alive=" + t.isAlive())
                .sorted()
                .forEach(System.out::println);

        System.out.println("----------> " + drivers.getDrivers().size());

        LocalDateTime dateTime = LocalDateTime.now();
        DateTimeFormatter formatter = DateTimeFormatter.ofPattern("dd-MM-yyyy HH:mm:ss");

        //save
        drivers.setLang(lang);
        end(lang, start);

        Joke joke = new Joke();
        joke.setSetup("drivers");
        joke.setPunchline(lang);
        //this.trSink.tryEmitNext(joke);
        return joke;
    }

    public Mono<Joke> getDrivers(String lang, final LocalDateTime start) {
        return this.client.get()
                .uri("/getdrivers/{lang}", lang)
                .httpRequest(httpRequest -> {
                    HttpClientRequest clientRequest = httpRequest.getNativeRequest();
                    clientRequest.responseTimeout(Duration.ofSeconds(500));
                })
                .retrieve()
                .bodyToMono(DriversDTO.class)
                .map(drivers -> save(drivers, lang, start));
    }

    private void start(String lang, LocalDateTime start) {
        log("START", lang, start);
    }

    private void end(String lang, LocalDateTime start) {
        log("END", lang, start);
    }

    private void log(String msg, String lang, LocalDateTime start) {
        LocalDateTime end = LocalDateTime.now();
        Duration d = Duration.between(start, end);
        DateTimeFormatter formatter = DateTimeFormatter.ofPattern("dd-MM-yyyy HH:mm:ss");
        System.out.println(Thread.currentThread().getName()
                + "::" + end.format(formatter)
                + "::" + d.toString()
                + "::" + msg + "::" + lang);
    }

}
