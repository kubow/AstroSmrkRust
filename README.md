# Kefer Astrology

Astrology application meant to run anywhere.

![preview](./astro-rust.png)

## Details

- backend part build with Rust + Tauri package.
  - hold the development process
  - releases packages
- frontend part is build in SvelteKit + Vite.
- start developing by `yarn tauri dev`.
- build by `cargo tauri build`

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


## To-Do

- Adding libraries to the backend
    - i18n for internationalization / translations
    - Data store using [DuckDB](https://duckdb.org/) or [SQLite](https://sqlite.org/)
- Installing additional libraries for the frontend part
    - `@astrodraw/astrochart`
    - `date-picker-svelte`
    - `d3` to rebuild the whole zodiac?


## Notes



- [stephaneworkspace (stephaneworkspace) / Repositories](https://github.com/stephaneworkspace?language=rust&page=2&q=&sort=&tab=repositories)
	- [libastro/src/lib.rs at master · stephaneworkspace/libastro](https://github.com/stephaneworkspace/libastro/pulse)
	- [stephaneworkspace/astro-rust: Astronomical algorithms in Rust](https://github.com/stephaneworkspace/astro-rust)
- [solar_system_ephemeris/src/physics.rs at master · Niisc/solar_system_ephemeris](https://github.com/Niisc/solar_system_ephemeris/blob/master/src/physics.rs)


Rust libraries
- [libswe_sys - Rust](https://docs.rs/libswe-sys/latest/libswe_sys/)
- [libastro - Rust](https://docs.rs/libastro/latest/libastro/)
- [Astrology — Rust library // Lib.rs](https://lib.rs/crates/astrology)
- 



Frontend
- [josefaidt/svelte-themer: A theming engine for your Svelte apps using CSS Variables, persisted.](https://github.com/josefaidt/svelte-themer)
- [Theme configuration file](https://docs.developers.optimizely.com/configured-commerce/docs/theme-configuration-file-in-classic)



Swiss ephemerides
- [swisseph documentation](https://www.astro.com/swisseph/swisseph.htm)
- 
