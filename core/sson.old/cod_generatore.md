
1. Perché questa separazione è geniale

Separazione	Vantaggio
Definizioni (struct, trait)	Leggere, compilazione veloce, interfaccia chiara
Implementazioni (_impl.rs)	Modificabili senza toccare le definizioni, hot-reload possibile
.sson come specifica	Generatore di codice, documentazione eseguibile, validatore

Con core/obj e core/base, le implementazioni diventano molto semplici.


.sson specifica
    │
    ↓ (genera)
validator_impl.rs
    │
    ↓ (usa)
registry_impl.rs
    │
    ├── register("req", fn)
    ├── register("min", fn)
    ├── register("pattern", fn)
    └── ...
    │
    ↓ (chiamato da)
BaseValidator.validate()
    │
    ↓ (usa)
registry.call("req", ctx, constraint) → bool


3. Generazione del codice dal .sson

Il .sson che abbiamo scritto può generare automaticamente:
Dal .sson	Genera
[.fields] type_e	Enum Type in types.rs
[.validator] S_f	SScore type alias
[.function_registry] validators_m	Registration boilerplate in validator_impl.rs
[.obj_adapter] methods_l	Trait ObjAdapter in lib.rs
[.state_machine]	Stato enum e transizioni

Esempio di generazione:

Dal .sson:

[.function_registry]
validators_m:
  req: "campo obbligatorio"
  min: "valore minimo numerico"
  
  Genera in validator_impl.rs:
  
registry.register("req", |ctx, c| ctx.obj.get(&c.target).is_some());
registry.register("min", |ctx, c| {
    ctx.obj.get(&c.target).as_number() >= c.params.get("value").as_number()
});

4. Vantaggi di questa architettura
Aspetto	Vantaggio
Definizioni leggere	Solo struct e trait, poche righe
Implementazioni separate	Si può fare hot-reload (ricaricare _impl.rs)
Usa core/obj	Non reinventa la ruota per dati dinamici
Usa core/base	Funzioni pure, composizione, memoization
Generato da .sson	La specifica è la fonte di verità
Testabile per parti	Si mockano le implementazioni


core/sson/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # Definizioni (type, struct, trait, mod)
│   ├── types.rs                  # Type aliases, enums
│   ├── validator_impl.rs         # Implementazione validatori
│   ├── resolver_impl.rs          # Implementazione resolver
│   ├── state_machine_impl.rs     # Implementazione stati
│   ├── registry_impl.rs          # Function registry
│   └── obj_adapter_impl.rs       # Adapter per core/obj


Le implementazioni (*_impl.rs) possono essere ricaricate a caldo. 
Le definizioni (lib.rs, types.rs) sono stabili.

