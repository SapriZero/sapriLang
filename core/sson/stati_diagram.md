                    ┌─────────────────────────────────────────────────────────┐
                    │                         START                          │
                    └─────────────────────────────┬───────────────────────────┘
                                                  ↓
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                         │
│  ┌──────┐    lex    ┌───────┐   parse   ┌─────────┐   resolve   ┌────────────┐         │
│  │ INIT │ ────────→ │ LEXING │ ────────→ │ PARSING │ ──────────→ │ RESOLVING  │         │
│  └──────┘           └───────┘           └─────────┘             └──────┬─────┘         │
│                                                                        │               │
│                                                    dedup               │               │
│                                      ┌─────────────────────────────────┘               │
│                                      ↓                                                  │
│                              ┌─────────────┐                                           │
│                              │ DEDUPLICATING│                                          │
│                              └──────┬──────┘                                           │
│                                     │ validate                                         │
│                                     ↓                                                  │
│                              ┌─────────────┐      flush      ┌──────────┐             │
│                              │ VALIDATING  │ ──────────────→ │ VALIDATED│             │
│                              └──────┬──────┘                  └────┬─────┘             │
│                                     │                              │                   │
│                                     │ (S ≥ 0.9)                    │ flush             │
│                                     └──────────────────────────────┘                   │
│                                                    ↓                                   │
│                                            ┌────────────┐                             │
│                                            │  EXPORTED  │                             │
│                                            └────────────┘                             │
│                                                                                         │
│  ┌─────────────────────────────────────────────────────────────────────────────────┐   │
│  │                           GESTIONE ERRORI                                       │   │
│  │                                                                                 │   │
│  │  ┌───────┐   rec    ┌───────┐                                                   │   │
│  │  │ ERROR │ ───────→ │ LEXING │ (solo generative)                               │   │
│  │  └───────┘          └───────┘                                                   │   │
│  │                                                                                 │   │
│  │  Strict mode: errore → stop                                                    │   │
│  │  Generative mode: errore → warning → rec → continua                            │   │
│  └─────────────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────────────────┘


Riepilogo del file
Sezione	Contenuto	Righe
ParserState enum	Init, Lexing, Parsing, Resolving, Deduplicating, Validating, Validated, Exported, Error	25-45
ParserAction enum	Lex, Parse, Resolve, Dedup, Validate, Err, Rec, Flush, Nop	50-70
Transition struct	Transizione con guardia e fallback	75-100
MachineContext	Contesto con stato, errori, warning, S score	105-180
ParseStats	Statistiche estese	185-200
ParserStateMachine	Macchina principale con pipeline	205-350
Azioni	Lex, Parse, Resolve, Dedup, Validate, Err, Rec, Flush	250-330
StateMachine impl	transition, current_state, is_valid_transition, run_pipeline	360-400
SimpleStateMachine	Versione semplificata per test	410-450
Tests	Strict, generative, transition, S score, timeout, action naming	460-530
Diagramma degli stati
text

                    ┌─────────────────────────────────────────────────────────┐
                    │                         START                          │
                    └─────────────────────────────┬───────────────────────────┘
                                                  ↓
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                         │
│  ┌──────┐    lex    ┌───────┐   parse   ┌─────────┐   resolve   ┌────────────┐         │
│  │ INIT │ ────────→ │ LEXING │ ────────→ │ PARSING │ ──────────→ │ RESOLVING  │         │
│  └──────┘           └───────┘           └─────────┘             └──────┬─────┘         │
│                                                                        │               │
│                                                    dedup               │               │
│                                      ┌─────────────────────────────────┘               │
│                                      ↓                                                  │
│                              ┌─────────────┐                                           │
│                              │ DEDUPLICATING│                                          │
│                              └──────┬──────┘                                           │
│                                     │ validate                                         │
│                                     ↓                                                  │
│                              ┌─────────────┐      flush      ┌──────────┐             │
│                              │ VALIDATING  │ ──────────────→ │ VALIDATED│             │
│                              └──────┬──────┘                  └────┬─────┘             │
│                                     │                              │                   │
│                                     │ (S ≥ 0.9)                    │ flush             │
│                                     └──────────────────────────────┘                   │
│                                                    ↓                                   │
│                                            ┌────────────┐                             │
│                                            │  EXPORTED  │                             │
│                                            └────────────┘                             │
│                                                                                         │
│  ┌─────────────────────────────────────────────────────────────────────────────────┐   │
│  │                           GESTIONE ERRORI                                       │   │
│  │                                                                                 │   │
│  │  ┌───────┐   rec    ┌───────┐                                                   │   │
│  │  │ ERROR │ ───────→ │ LEXING │ (solo generative)                               │   │
│  │  └───────┘          └───────┘                                                   │   │
│  │                                                                                 │   │
│  │  Strict mode: errore → stop                                                    │   │
│  │  Generative mode: errore → warning → rec → continua                            │   │
│  └─────────────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────────────────┘

Prossimo passo

Ora abbiamo:

    ✅ lib.rs (definizioni)

    ✅ validator_impl.rs (BaseValidator)

    ✅ resolver_impl.rs (PathResolver)

    ✅ registry_impl.rs (FunctionRegistry)

    ✅ obj_adapter_impl.rs (ObjAdapter)

    ✅ state_machine_impl.rs (StateMachine)

Vuoi che procediamo con:

    error_impl.rs (errori e recovery dettagliati)

    Aggiornamento lib.rs (esportare tutti i moduli e fare i test di integrazione)

    tests/ (test di integrazione end-to-end)

