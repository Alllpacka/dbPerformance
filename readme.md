# Performance Testing 
## Aufgabe:
- Auswahl und Generierung eines großen (—1 million) Datasets mittels eines Data-Generators
- einspielen in eine SQL-Datenbank (mysql, postgresql, ...)
- einspielen in eine MongoDB-Datenbank
- Performanceaussagen verschiedenen Datenbanken und Betriebssysteme
- Generierung von Indizes und damit verbundene Performancegewinne

## Lösung 1:
Mithilfe der [Fake Library](https://docs.rs/fake/latest/fake/) Daten generieren:
~~~rust
#[derive(Serialize, Dummy, Clone)]
pub struct Data {
    #[dummy(faker = "FirstName()")]
    firstname: String,
    #[dummy(faker = "LastName()")]
    lastname: String,
    #[dummy(faker = "CityName()")]
    city: String,
    #[dummy(faker = "(18..65)")]
    age: u8,
    #[dummy(faker = "SafeEmail()")]
    email: String,
    #[dummy(faker = "PhoneNumber()")]
    phone_number: String,
}
~~~
mit einem Stuct kann genau angegeben werden welche Daten Faker generieren soll

## Lösung 2 & 3:
Mithilfe der [postgreSQL](https://docs.rs/postgres/0.19.7/postgres/) und [MongoDB](https://docs.rs/mongodb/2.8.2/mongodb/) Libraries

## Lösung 4:
Ebenfalls mithilfe der postgreSQL und MongoDB Libraries, aber mit der Eingebauten ``std::time::Instant`` Methode

### Windows:
#### Insert:
##### PostgreSQL:
``414s``
##### MongoDB:
``14s``

#### Select:
##### PostgreSQL:
``2065ms``
##### MongoDB:
``17ms``

## Lösung 5:
In Bearbeitung