## Tabella riorganizzata con ottimizzazione dei bit

---

## 1. Nuova struttura a livelli

| Codice | Bit | Tipo | Sottotipo | Indice | Bit totali | Capacità |
|--------|-----|------|-----------|--------|------------|----------|
| **PAROLE CORTE** |
| `10` | 2 | Parola corta (1-3 char) | - | 12 | 14 | 4096 |
| **FREQUENTI (solo le più comuni)** |
| `11` | 2 | Frequente 1 (top) | - | 16 | 18 | 65536 |
| **PERSONAGGI** |
| `010 00` | 5 | Personaggi famosi | - | 16 | 21 | 65536 |
| **NOMI DI PERSONA** |
| `010 01 0` | 5 | Nomi italiani | - | 16 | 21 | 65536 |
| `010 01 1` | 5 | Nomi stranieri | - | 16 | 21 | 65536 |
| **LUOGHI** |
| `010 10 00` | 6 | Nazioni/Continenti | - | 16 | 22 | 65536 |
| `010 10 01` | 6 | Regioni/Province | - | 16 | 22 | 65536 |
| `010 10 10` | 6 | Città comuni | - | 16 | 22 | 65536 |
| `010 10 11` | 6 | Luoghi rari (32 bit) | 32 | 38 | 4B |
| **SIGLE E TERMINI TECNICI** |
| `010 11 0` | 5 | Sigle comuni | - | 16 | 21 | 65536 |
| `010 11 1` | 5 | Termini tecnici | - | 16 | 21 | 65536 |
| **VERBI** |
| `011 00 0` | 5 | Verbi comuni (top) | - | 16 | 21 | 65536 |
| `011 00 1` | 5 | Verbi rari (32 bit) | 32 | 37 | 4B |
| **SPAZIO/TEMPO/DIREZIONE/QUANTITÀ (con flag)** |
| `011 01 00` | 6 | Spaziale (SP) | 12 | 18 | 4096 |
| `011 01 01` | 6 | Temporale (TM) | 12 | 18 | 4096 |
| `011 01 10` | 6 | Direzione (DR) | 12 | 18 | 4096 |
| `011 01 11` | 6 | Quantità (QT) | 12 | 18 | 4096 |
| **NUMERI** |
| `00 00` | 4 | Numero | variabile | variabile | ∞ |
| **PUNTEGGIATURA** |
| `00 01` | 4 | Punteggiatura/spazio | 8 | 12 | 256 |
| **PAROLE SCONOSCIUTE** |
| `00 10` | 4 | Parola sconosciuta | 16 | 20 | 65536 |
| **CALENDARIO** |
| `00 11` | 4 | Calendario | 8 | 12 | 256 |

---

## 2. Flag per relazioni spaziali/temporali (12 bit = 4096 valori)

### Flag spaziali (SP)

| Flag | Simbolo | Significato | Esempi |
|------|---------|-------------|--------|
| `SP_HOR_LEFT` | `-*0x` | orizzontale, a sinistra | "a sinistra", "alla sua sinistra" |
| `SP_HOR_RIGHT` | `-*1x` | orizzontale, a destra | "a destra", "alla sua destra" |
| `SP_HOR_CENTER` | `-*2x` | orizzontale, al centro | "al centro", "in mezzo" |
| `SP_VER_UP` | `|*0x` | verticale, sopra | "sopra", "al di sopra" |
| `SP_VER_DOWN` | `|*1x` | verticale, sotto | "sotto", "al di sotto" |
| `SP_VER_MID` | `|*2x` | verticale, a metà | "a metà", "nel mezzo" |
| `SP_DEPTH_FRONT` | `/*0x` | profondità, davanti | "davanti", "di fronte" |
| `SP_DEPTH_BACK` | `/*1x` | profondità, dietro | "dietro", "alle spalle" |
| `SP_DEPTH_MID` | `/*2x` | profondità, in mezzo | "in mezzo", "tra" |
| `SP_CLOSE` | `~*0x` | vicino | "vicino", "presso" |
| `SP_FAR` | `~*1x` | lontano | "lontano", "distante" |

### Flag temporali (TM)

| Flag | Simbolo | Significato | Esempi |
|------|---------|-------------|--------|
| `TM_PAST` | `t-*` | passato | "prima", "ieri", "già" |
| `TM_PRESENT` | `t0*` | presente | "ora", "adesso", "oggi" |
| `TM_FUTURE` | `t+*` | futuro | "dopo", "domani", "più tardi" |
| `TM_DURATION` | `t~*` | durata | "durante", "mentre" |
| `TM_POINT` | `t.*` | istante | "quando", "nel momento in cui" |

### Flag direzionali (DR)

| Flag | Simbolo | Significato | Esempi |
|------|---------|-------------|--------|
| `DR_NORTH` | `↑` | nord | "nord", "settentrionale" |
| `DR_SOUTH` | `↓` | sud | "sud", "meridionale" |
| `DR_EAST` | `→` | est | "est", "orientale" |
| `DR_WEST` | `←` | ovest | "ovest", "occidentale" |
| `DR_UP` | `⇧` | su | "su", "in alto" |
| `DR_DOWN` | `⇩` | giù | "giù", "in basso" |

### Flag quantitativi (QT)

| Flag | Simbolo | Significato | Esempi |
|------|---------|-------------|--------|
| `QT_MUCH` | `*+` | molto | "molto", "tanto" |
| `QT_LITTLE` | `*-` | poco | "poco", "scarso" |
| `QT_MORE` | `+` | più | "più", "maggiore" |
| `QT_LESS` | `-` | meno | "meno", "minore" |
| `QT_ABOUT` | `≈` | circa | "circa", "quasi" |
| `QT_TOO` | `!` | troppo | "troppo", "eccessivo" |
| `QT_ENOUGH` | `=` | abbastanza | "abbastanza", "sufficiente" |

---

## 3. Vantaggi di questa organizzazione

| Vantaggio | Descrizione |
|-----------|-------------|
| **Frequenti ridotti** | Solo `11` e `010 00` per i veramente frequenti (131k parole) |
| **Più spazio per semantica** | `010 01` usato per nomi (131k), `010 10` per luoghi (262k) |
| **Flag spaziali/temporali** | 12 bit (4096 valori) per esprimere tutte le relazioni |
| **Simboli intuitivi** | `-*0x` = sinistra, `t+*` = futuro, `↑` = nord |
| **L'AI capisce subito** | Non deve analizzare "sopra", sa già che è SP_VER_UP |

---

## 4. Riepilogo capacità totali

| Categoria | Capacità | Bit medi |
|-----------|----------|----------|
| Parole corte | 4.096 | 14 |
| Frequenti | 131.072 | 18-21 |
| Personaggi | 65.536 | 21 |
| Nomi | 131.072 | 21 |
| Luoghi | 262.144 + 4B | 22-38 |
| Sigle/Tecnici | 131.072 | 21 |
| Verbi | 65.536 + 4B | 21-37 |
| Spazio/Tempo/Direzione/Quantità | 16.384 | 18 |
| Punteggiatura | 256 | 12 |
| Calendario | 256 | 12 |
| Sconosciute | 65.536 | 20 |
| **Totale** | **~1.000.000** | **~20** |
