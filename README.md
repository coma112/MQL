# MQL 🇭🇺 - Magyar Query Language

## Mi ez?

Az MQL (Magyar Query Language) egy SQL-szerű nyelv, amely 100%-ban magyar kulcsszavakat használ. 
A projekt csak viccből készül(t).
Nem egy szerveren van tárolva az adat hanem JSON fájlokba.

Minden JSON fájl egy adatbázis és azon belül vannak táblák.

## Telepítés és használat

```bash
# Fordítás
cargo build --release

# Futtatás
cargo run
```

## Jelenleg támogatott parancsok

| SQL parancs | MQL parancs | Leírás | Példa |
|------------|-------------|---------|-------|
| `CREATE TABLE` | `LÉTREHOZ TÁBLA` | Új tábla létrehozása | `LÉTREHOZ TÁBLA felhasznalok` |
| `CREATE DATABASE` | `LÉTREHOZ ADATBÁZIS` | Új adatbázis létrehozása | `LÉTREHOZ ADATBÁZIS webshop` |
| `DROP TABLE` | `LEDOB TÁBLA` | Tábla törlése | `LEDOB TÁBLA felhasznalok` |
| `DROP DATABASE` | `LEDOB ADATBÁZIS` | Adatbázis törlése | `LEDOB ADATBÁZIS webshop` |
| `HELP` | `HELP` | Súgó megjelenítése | `HELP` |
| `EXIT` / `QUIT` | `EXIT` / `QUIT` | Kilépés a programból | `EXIT` |

## Használati példák

```sql
-- Adatbázis létrehozása
LÉTREHOZ ADATBÁZIS webshop

-- Tábla létrehozása
LÉTREHOZ TÁBLA termekek

-- Tábla törlése
LEDOB TÁBLA termekek

-- Adatbázis törlése
LEDOB ADATBÁZIS webshop

-- Súgó
HELP

-- Kilépés
EXIT
```

## Tervezett funkciók (hamarosan)

- [ ] `BEILLESZT` - INSERT INTO (adatok beszúrása)
- [ ] `KIVÁLASZT` - SELECT (adatok lekérdezése)
- [ ] `FRISSÍT` - UPDATE (adatok módosítása)
- [ ] `TÖRÖL` - DELETE (sorok törlése)
- [ ] `HOL` - WHERE (feltételek)
- [ ] `RENDEZ` - ORDER BY (rendezés)
- [ ] `CSOPORT` - GROUP BY (csoportosítás)
- [ ] `Típusok` - INTEGER/INT (EGÉSZ_SZÁM), STRING (HÚR), BOOL (LOGIKAI_ÉRTÉK)
- [ ] `Operátorok` - Ezek megmaradnak mint szimbólumok:)

## Miért?

Mert miért ne?

## Hozzájárulás

Pull requestek és ötletek mindig szívesen látottak! Ha van ötleted, hogy melyik SQL kulcsszó milyen magyar megfelelőt kapjon, nyiss egy issue-t!

## License

Ez egy viccprojekt, használd szabadon!

---