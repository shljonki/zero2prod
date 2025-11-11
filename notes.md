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
* web::Data - wraps anything into Arc<T>
  ovo je dobar trik ako imamo bilo što što nije Clonable. Ako wrappamo u Arc<T> postaje Cloneable
* za razliku od web::Form, actix-web neće automatski web::Data neće sam deserializirat iz HttpRequesta
  putem web::FromRequest::from_request() već to moramo sami napraviti preko HttpRequest::app_data::<T>()
  gdje je T neki tip podataka u koji zelimo deserializirati
-> 3.7.1
* 