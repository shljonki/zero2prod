Chapter 3
* actixWeb - server, stvori app, middleware itd za posluzivanje clienta, tj. primanje http requestova (REST API)
* reqwest - client, slanje http requestova na server putem REST API
* TCPListener - da binda na random port u compiletimeu i proslijedi ga nasem programu,
  random port se dobije pomocu 127.0.0.1:0

Chapter 3.5
-> 2.3
* web::Form je extractor URL encoded payloada
* web::FromRequest::from_request() fju actix-web poziva nad svakim argumentom nekog handlera
  nekog routa (subscribe handler npr) sto znaci da svaki argument (recimo web::Form<FormData>)
  mora implementirat FromRequest trait jer je u njemu from_request() pa ce from_request() pokusat
  deserializirat request body od HttpRequesta u zeljenu strukturu recimo FormData
* Serde - da mozemo pretvarat random data format <-> rust data type.
  koristi neki svoj serde data model kao posrednika izmedu Serialize i Serializer
  https://www.youtube.com/watch?v=BI_bHCGRgMY
-> 3.2
* sqlx crate - PgConnection struct je entrypoint za spojit se na Postgres database
* sa mod dodamo neki module isto kao sto crate dodajemo u cargo.toml, sa use koristimo neku fju iz modula/cratea
  sa pub mod taj mod napravimo javnim da ga i ostali (recimo parrenti) mogu koristit
  sa pub use fju napravimo javnom da je drugi (recimo parrenti) mogu koristit
  i mod i fja/struktura/itd u modu moraju biti public da bi je parrent mogo koristit
* config crate - swiss army knife za environment variables, configuration files, etc.
-> 3.6
* web::Data<T> - wraps anything into Arc<T>
  ovo je dobar trik ako imamo bilo što što nije Clonable. Ako wrappamo u Arc<T> postaje Cloneable
* za razliku od web::Form, actix-web neće automatski deserializirat web::Data<T> iz HttpRequesta
  putem web::FromRequest::from_request() već to moramo sami napraviti preko HttpRequest::app_data::<T>()
  gdje je T neki tip podataka u koji zelimo deserializirati
* HttpServer za svaku CPU jezgru napravi zasebnog workera, tj. svaki put će pokrenuti novu aplikaciju.
  Zato aplikacija uvijek mora imati vlasništvo nad podacima pa PgConnection koji šaljemo putem App::app_data
  mora biti Cloneable pa koristimo web::Data<T>
* .execute koji koristi tu konekciju (PgConnect) u subscription.rs zahtjeva podatke koji implementiraju
  Executor: Send + Debug pa tako nesmijemo koristiti &PgConnect nego samo &mut PgConnect. To je zato što
  nesmijemo imati više konkurentnih istih konekcija kako si nebi paralelno prebrisali podatke.
  &mut je unique identifier po dizajnu compilera (smije postojati samo jedna mutable referenca na istu
  vrijednost u cijelom programu) no web::Data<T> preko kojeg dobivamo PgConnect ne dopušta da šaljemo
  reference na <T>. Drugi način da postignemo ne konkurentne konekcije tj. interior mutability je da
  &PgConnect wrappamo u Mutex<> no tada bi unutar workera jedan thread mogao zakljucati DB i blokirat
  je za cijelog workera.
* PgPool je pool konekcija na postgres koje također imaju interior mutability no malo drugačiji. Ako
  postoji slobodna PgConnect konekcija sqlx ce posudit nju a ako ne postoji, stvorit će novu (ne istu, drugu)
  ili ce pricekati da se neka stara oslobodi. Tako da je PgPool skup različitih konekcija na isti DB.
-> 3.7.1

Chapter 4 - Observability
-> 3.3
* Logger je struktura u koju se zapisuju logovi i kaj sve ne, no ona mora implementirat Log trait
  da bi išta loggala
* Log crate je fasada, tj. API koji nam kaze koje sve fje Log trait mora imat. u crateu su izlistani
  razliciti loggeri tipa env_logger, simple_logger itd. u kojima su definirani Logger strukture i
  Log traitovi
* actix-web ima Logger middleware tj. Logger strukturu ali ona nema definiran Log trait pa koristimo
  Log trait iz env_logger. env_logger crate takoder ima svoju Logger strukturu no ne koristimo nju
  već koristimo middleware Logger i Log trait iz env_logger crata.
  koji Log trait ćemo koristiti kazemo preko set_logger fje u log crateu. u nasem slucaju set_logger
  se poziva u ovom initu na kraju -> env_logger::Builder::from_env(Env::default()).init();
-> 5
* sa log crateom je bio problem kaj ako imamo paralelne procese, ne znamo koj log je čiji, pa smo dodali
  request id na njih. Problem je što middleware Logger nije svjestan tog request_id-a pa ga nemremo
  poslati van aplikacije.
* tu dolazi tracing crate koji je napredniji log crate jer ne logira u samo jednom trenutku (event) nego
  cijeli period (span) od kad mu kazemo da starta pa do kraja fje ili dok ga ne ugasimo.
* span.enter je za single threded/proces fje, za future je .instrument(span)
-> 5.5
* Subscriber trait u tracing crateu je isto što i Log trait u log crateu. tj. On je fasada koja opisuje
  koje fje moramo definirat za span telemetriju. implementaciju Subscribera uzmemo iz jednog od javno
  dostupnih crateova, recimo tracing-bunyan-formatter cratea.
* Subscriber trait sadrzi sve potrebno za skupljanje traceva pa tako i kreiranje span ID-a. Pošto svaki
  span mora imat točno jedan dodjeljen ID, ne mozemo imati više subscribera za jedan span. Zato koristimo
  Layer trait, koji ima ulogu observera. implementacija Layer traita pokazuje sa kojim Subscriberima se
  može koristit.
* iskombinirat cemo 3 layera. oni nesto nadodaju na Subscriber trait, nisam ziher kaj
  tracing_subscriber::filter::EnvFilter je log level
  tracing_bunyan_formatter::JsonStorageLayer spremi metadatu
  tracing_bunyan_formatter::BunyanFormatterLayer je za ispis JSONa
* JsonStorageLayer implementira tracing_subscriber::registry::LookupSpan zato da bi imao pristup Registryu.
  Registry je Subscriber koji zapravo ne tracea ništa nego sprema metadatu koja je exposana Layerima upravo
  preko tog LookupSpan traita. Ako Layer želi imat pristup Registry metadati, mora implementirat LookupSpan trait,
  pa ga zato JsonStorageLayer implementira. Span metadata je npr kad je span nastao, odnosi izmedu spanova,
  koji spanovi su aktivni koji ne. Registry takoder moze i user-datu spremat, koja se naziva extensions.
* definirali smo Subscribera, u nasem slucaju Registry jer nam je trebo zbog BunyanFormatterLayera, pa smo
  na taj Registry/Subscribera dodali Layere i na postavili tog subscribera kao defaultnog za traceanje
-> 5.8
* Log Tracer
