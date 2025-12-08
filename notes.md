[Chapter 3]: https://lpalmieri.com/posts/2020-08-09-zero-to-production-3-how-to-bootstrap-a-new-rust-web-api-from-scratch/
[Chapter 3.5]: https://lpalmieri.com/posts/2020-08-31-zero-to-production-3-5-html-forms-databases-integration-tests/
[Chapter 4]: https://lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/
[Chapter 5]: https://lpalmieri.com/posts/2020-11-01-zero-to-production-5-how-to-deploy-a-rust-application/#1-we-must-talk-about-deployments
[Chapter 6]: https://lpalmieri.com/posts/2020-12-11-zero-to-production-6-domain-modelling
[Chapter 6.5]: https://lpalmieri.com/posts/an-introduction-to-property-based-testing-in-rust/
[Chapter 7 - Part 0]: https://lpalmieri.com/posts/how-to-write-a-rest-client-in-rust-with-reqwest-and-wiremock/
[Chapter 7 - Part 1]: https://lpalmieri.com/posts/skeleton-and-principles-for-a-maintainable-test-suite/
[Chapter 7 - Part 2]: https://lpalmieri.com/posts/zero-downtime-deployments/
[Chapter 8]: https://lpalmieri.com/posts/error-handling-rust/
[Chapter 9]: https://lpalmieri.com/posts/naive-newsletter-delivery
[Chapter 10 - Part 0]: https://lpalmieri.com/posts/password-authentication-in-rust/
[Chapter 10 - Part 1]: https://lpalmieri.com/posts/session-based-authentication-in-rust/
[Chapter 11]: https://lpalmieri.com/posts/idempotency/

[`actix-web` crate]: https://crates.io/crates/actix-web
[`serde` crate]: https://crates.io/crates/serde
[`reqwest` crate]: https://crates.io/crates/reqwest
[`sqlx` crate]: https://crates.io/crates/sqlx
[`config` crate]: https://crates.io/crates/config
[`log` crate]: https://crates.io/crates/log
[`env_logger` crate]: https://crates.io/crates/env_logger
[`tracing` crate]: https://crates.io/crates/tracing
[`tracing-log` crate]: https://crates.io/crates/tracing-log
[`tracing_subscriber` crate]: https://crates.io/crates/tracing-subscriber
[`tracing-bunyan-formatter` crate]: https://crates.io/crates/tracing-bunyan-formatter
[`tracing-actix-web` crate]: https://crates.io/crates/tracing-actix-web
[`cargo-udeps` crate]: https://crates.io/crates/cargo-udeps
[`secrecy` crate]: https://docs.rs/secrecy/0.10.3/secrecy/index.html

# TOC
- [TOC](#toc)
- [Chapter 3 - How To Bootstrap A Rust Web API From Scratch](#chapter-3---how-to-bootstrap-a-rust-web-api-from-scratch)
- [Chapter 3.5 - HTML forms, Databases, Integration tests](#chapter-35---html-forms-databases-integration-tests)
  - [2.3](#23)
  - [3.2](#32)
  - [3.6](#36)
- [Chapter 4 - Telemetry](#chapter-4---telemetry)
  - [3.3](#33)
  - [5](#5)
  - [5.5](#55)
  - [5.7](#57)
  - [5.8](#58)
  - [5.9](#59)
  - [5.11](#511)
  - [5.12](#512)
  - [5.13](#513)
  - [5.14](#514)
- [Chapter 5 - Going Live](#chapter-5---going-live)
  - [](#)

# [Chapter 3 - How To Bootstrap A Rust Web API From Scratch][Chapter 3]
* [`actix-web` crate] - server, stvori app, middleware itd za posluzivanje clienta, tj. primanje http requestova (REST API)
* [`reqwest`][[`reqwest` crate]] - client, slanje http requestova na server putem REST API
* `TCPListener` - da binda na random port u compiletimeu i proslijedi ga nasem programu,
  random port se dobije pomocu 127.0.0.1:0

# [Chapter 3.5 - HTML forms, Databases, Integration tests][Chapter 3.5]
## 2.3
* `web::Form` je extractor URL encoded payloada
* `web::FromRequest::from_request()` fju actix-web poziva nad svakim argumentom nekog handlera
  nekog routa (subscribe handler npr) sto znaci da svaki argument (recimo `web::Form<FormData>`)
  mora implementirat `FromRequest` trait jer je u njemu `from_request()` pa ce from_request()` pokusat
  deserializirat request body od `HttpRequesta` u zeljenu strukturu recimo FormData
* [`Serde` crate][serde crate] - da mozemo pretvarat random data format <rust data type.
  koristi neki svoj serde data model kao posrednika izmedu `Serialize` i `Serializer`
* [decrusting Serde](https://www.youtube.com/watch?v=BI_bHCGRgMY)
## 3.2
* [`sqlx` crate] - `PgConnection` struct je entrypoint za spojit se na Postgres database, tj. konekcija na DB.
* sa mod dodamo neki module isto kao sto crate dodajemo u cargo.toml, sa use koristimo neku fju iz modula/cratea
  sa pub mod taj mod napravimo javnim da ga i ostali (recimo parrenti) mogu koristit
  sa pub use fju napravimo javnom da je drugi (recimo parrenti) mogu koristit
  i mod i fja/struktura/itd u modu moraju biti public da bi je parrent mogo koristit
* [`config` crate] - swiss army knife za environment variables, configuration files, etc.
## 3.6
* `web::Data<T>` - wraps anything into `Arc<T>`
  ovo je dobar trik ako imamo bilo što što nije `Cloneable` . Ako wrappamo u `Arc<T>` postaje `Cloneable`
* za razliku od `web::Form`, actix-web neće automatski deserializirat `web::Data<T>` iz `HttpRequesta`
  putem `web::FromRequest::from_request()` već to moramo sami napraviti preko `HttpRequest::app_data::<T>()`
  gdje je T neki tip podataka u koji zelimo deserializirati
* HttpServer za svaku CPU jezgru napravi zasebnog workera, tj. svaki put će pokrenuti novu aplikaciju.
  Zato aplikacija uvijek mora imati vlasništvo nad podacima pa `PgConnection` koji šaljemo putem App::app_data
  mora biti `Cloneable` pa koristimo `web::Data<T>`
* .execute koji koristi tu konekciju (`PgConnect`) u subscription.rs zahtjeva podatke koji implementiraju
  Executor: `Send`  + Debug pa tako nesmijemo koristiti `&PgConnect` (nije `Send` ) nego samo &mut `PgConnect`.
  To je zato što nesmijemo imati više konkurentnih istih konekcija kako si nebi paralelno prebrisali podatke.
  &mut je unique identifier po dizajnu compilera (smije postojati samo jedna mutable referenca na istu
  vrijednost u cijelom programu) pa zato &mut `PgConnect` u teoriji smije no `web::Data<T>` preko kojeg
  dobivamo `PgConnect` ne dopušta da šaljemo reference na `<T>`. Drugi način da postignemo ne konkurentne
  konekcije tj. interior mutability je da `&PgConnect` wrappamo u Mutex<> no tada bi unutar workera jedan
  thread mogao zakljucati DB i blokirat je za cijelog workera.
* `PgPool` je pool konekcija na postgres koje također imaju interior mutability no malo drugačiji. Ako
  postoji slobodna `PgConnect` konekcija sqlx ce posudit nju a ako ne postoji, stvorit će novu (ne istu, drugu)
  ili ce pricekati da se neka stara oslobodi. Tako da je `PgPool` skup različitih konekcija na isti DB.

# [Chapter 4 - Telemetry][Chapter 4]
## 3.3
* `Logger` je struktura u koju se zapisuju logovi i kaj sve ne, no ona mora implementirat `Log` trait
  da bi išta loggala
* [`log` crate] je fasada, tj. API koji nam kaze koje sve fje `Log` trait mora imat. u crateu su
  izlistani razliciti loggeri tipa [`env_logger` crate], `simple_logger` itd. u kojima su definirani
  `Logger` strukture i `Log` traitovi
* actix-web ima `Logger` middleware tj. `Logger` strukturu ali ona nema definiran `Log` trait pa koristimo
  `Log` trait iz `env_logger`. [`env_logger` crate] takoder ima svoju `Logger` strukturu
  no ne koristimo nju već koristimo `middleware::Logger` i `Log` trait iz [`env_logger` crate]a.
  koji `Log` trait ćemo koristiti kazemo preko `set_logger` fje u `log` crateu. u nasem slucaju `set_logger`
  se poziva u ovom initu na kraju `env_logger::Builder::from_env(Env::default()).init();`
## 5
* sa `log` crateom je bio problem kaj ako imamo paralelne procese, ne znamo koj log je čiji, pa smo dodali
  request id na njih. Problem je što middleware `Log` nije svjestan tog request_id-a pa ga nemremo
  poslati van aplikacije.
* tu dolazi `tracing` crate koji je napredniji `log` crate jer ne logira u samo jednom trenutku (event) nego
  cijeli period (span) od kad mu kazemo da starta pa do kraja fje ili dok ga ne ugasimo.
* span.enter je za single threded/proces fje, za future je .instrument(span)
## 5.5
* `Subscriber` trait u [`tracing` crate]u je isto što i `Log` trait u [`log` crate]u. tj. On je fasada
  koja opisuje koje fje moramo definirat za span telemetriju. implementaciju `Subscriber`a uzmemo iz jednog od javno
  dostupnih crateova, recimo [`tracing-bunyan-formatter` crate]a.
* `Subscriber` trait sadrzi sve potrebno za skupljanje traceva pa tako i kreiranje span ID-a. Pošto svaki
  span mora imat točno jedan dodjeljen ID, ne mozemo imati više subscribera za jedan span. Zato koristimo
  `Layer` trait, koji ima ulogu observera. implementacija `Layer` traita pokazuje sa kojim Subscriberima se
  može koristit.
## 5.7
* iskombinirat cemo 3 layera. oni nesto nadodaju na `Subscriber` trait. 
  [`tracing_subscriber::filter::EnvFilter`][`tracing_subscriber` crate] je log level
  [`tracing_bunyan_formatter::JsonStorageLayer`][`tracing-bunyan-formatter` crate] spremi metadatu
  `tracing_bunyan_formatter::BunyanFormatterLayer` je za ispis JSONa
* `JsonStorageLayer` implementira `tracing_subscriber::registry::LookupSpan` zato da bi imao pristup `Registry`u.
  `Registry` je `Subscriber` koji zapravo ne tracea ništa nego sprema metadatu koja je exposana Layerima upravo
  preko tog `LookupSpan` traita. Ako `Layer` želi imat pristup `Registry` metadati, mora implementirat `LookupSpan`
  trait, pa ga zato `JsonStorageLayer` implementira. `Span` metadata je npr kad je span nastao, odnosi izmedu
  spanova, koji spanovi su aktivni koji ne. Registry takoder moze i user-datu spremat, koja se naziva extensions.
* definirali smo `Subscribera`, u nasem slucaju `Registry` jer nam je trebo zbog `BunyanFormatterLayera`, pa smo
  na taj `Registry`/`Subscribera` dodali Layere i na postavili tog subscribera kao defaultnog za traceanje
## 5.8
* kad god se dogodi tracing event ili span, triggera se log record pa ga logger iz [`log` crate]a pokupi. To nam
  je omogućio `log` feature u `tracing` crateu. no obrnuto, kada `log` crate ide nesto loggirat, ne triggera se tracing
  event, pa koristimo [`tracing-log` crate].
* `LogTracer` je struktura koja implementira `log::Log` trait (vidi [3.3](#33) gore) na način da consumea
  `Record` (to je payload nekog loga) i pretvara ga u `tracing::Event`
* tracing's feature log: tracing event -> log record
* tracing-log crate: log record -> tracing event
## 5.9
* [`cargo-udeps` crate] sluzi za micanje Unused DEPendeciesa
## 5.11
* u testovima imamo problem što svaki test poziva `spawn_app()` i onda svaki put ispocetka postaavljaju
  defaultni `Log` i `Subscriber`, što zapravo ne možemo radit, pa nam program počne paničarit. Zato koristimo
  once-cell crate koji ima `Lazy` strukturu koja nam omogućava da nešto zapišemo samo jednom a kod ostalih pokušaja
  pisanja, samo nam vrati već zapisanu vrijednost.
* stavili smo da u `get_subscriber()` treba poslat nesto sto implementira
  `Sink: for<'a> MakeWriter<'a> + Send  + Sync  + 'static` tj. nesto sto implementira MakeWriter sto su
  `std::io::{sink, stdout}` pa onda ovisno o tome je li `TEST_LOG` postavljen ili ne, ispisujemo traceve na stdout
  ili ih bacimo/sinkamo pomocu `std::io::sink`
## 5.12
* mozemo stavit `[tracing::instrument]` macro na pocetak fje, pa ce se napravit span svaki put kad udemo u tu fju
## 5.13
* [`secrecy` crate] nam zabrani da printamo ili koristimo stvari koje smo wrappali u `SecretBox` tako
  što `Display` trait ne implementira a `Debug` implementira tako da umjesto contenta napiše REDACTED
* sakrili smo password iz config filea od DB, a pošto `connection_string()` sadrzi password, i njega smo stavili u
  `SecretBox`
## 5.14
* `middleware::Logger` ne stavi request_id kad dode HTTP request, pa koristimo drugi novi - `TracingLogger` iz
  [`tracing-actix-web` crate]a. taj request_id se propagira dalje na pod spanove.
* u `spawn_app()` namjestimo subscribera koji će skupljat eventove i spanove, a u `run()` kod rađenja nove App,
  definiramo loggera koji će logove pretvarat u eventove i spanove. u `subscribe()` pomoću #instrument macroa kazemo
  da hocemo novi span svaki put kad udemo u tu i `insert_subscriber()` fje
* `spawn_app()`: namjesti defaultnog subscribera i defaultnog loggera `LogTracer` -> `run()`: namjesti da `TracingLogger` triggera
  spanove za svaki HTTP request -> `subscribe()`: #instrument triggera spanove
  -> `insert_subscriber()`: #instrument triggera spanove
* a da bi `TracingLogger` mogao triggerat eventove i spanove, to smo omogucili pomocu [`tracing-log` crate]a

# [Chapter 5 - Going Live][Chapter 5]
## 