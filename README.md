# **WIP** gw2api
[![Build Status](https://travis-ci.org/CuriouslyCurious/gw2api.svg?branch=master)](https://travis-ci.org/CuriouslyCurious/gw2api)
[![Crates.io](https://img.shields.io/crates/v/gw2api.svg)](https://crates.io/crates/gw2api)
[![Documentation](https://docs.rs/gw2api/badge.svg)](https://docs.rs/gw2api)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License:Apache-2.0](https://img.shields.io/badge/License-APACHE-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![codecov](https://codecov.io/gh/CuriouslyCurious/gw2api/branch/master/graph/badge.svg)](https://codecov.io/gh/CuriouslyCurious/gw2api)

`gw2api` is a **WIP** wrapper over the official Guild Wars 2 API written in **Rust** using
[ureq](https://github.com/algesten/ureq) as the underlying request maker.

 ```rust
 use gw2api::client::Client;
 use gw2api::v1::build::Build;

 let client = Client::new();
 let build = Build::get_build(&client).unwrap();
 println!("Current build id: {}", build.id());
 ```

## Currently supported endpoints:
* [ ] Endpoints
    * [ ] V2
        * [ ] Account
            * [ ] Bank (Auth)
            * [ ] Dyes (Auth)
            * [ ] Materials (Auth)
            * [ ] Skins (Auth)
            * [ ] Characters (Auth)
            * [ ] Shared Inventory (Auth)
        * [ ] Achievements
            * [ ] Achievements
            * [ ] Achievements Daily
            * [ ] Achievement Groups
            * [ ] Achievement Categories
        * [ ] Commerce
            * [ ] Listings
            * [ ] Echange
            * [ ] Prices
            * [ ] Transactions (Auth)
        * [ ] Guilds
            * [ ] Guild Upgrades
            * [ ] Guild Permissions
            * [ ] Guild Members (Auth Guild Leader)
            * [ ] Guild Ranks (Auth Guild Leader)
            * [ ] Guild Stash (Auth Guild Leader)
            * [ ] Guild Treasury (Auth Guild Leader)
            * [ ] Log (Auth Guild Leader)
            * [ ] Emblems
            * [ ] Teams
        * [ ] PvP
            * [x] Amulets
            * [x] Heroes
            * [ ] Stats (Auth)
            * [x] Games (Auth)
            * [x] Ranks (Auth)
            * [ ] Standings (Auth)
            * [x] Seasons
        * [ ] Items
            * [ ] Recipes
                * [ ] Search
            * [ ] Items
            * [ ] Skins
        * [ ] World vs World
            * [ ] Matches
            * [ ] Objectives
        * [ ] Game Mechanics
            * [ ] Traits
            * [ ] Specializations
        * [ ] Map Information
            * [ ] Continents
            * [ ] Maps
        * [ ] Misc
            * [x] Build
            * [ ] Colors
            * [ ] Currencies
            * [ ] Quaggans
            * [ ] Worlds
            * [ ] Minis
            * [x] Tokeninfo
    * [x] V1
        * [x] Build
        * [x] Colors
        * [x] Continents
        * [x] Event details
        * [x] Event names (DISABLED)
        * [ ] Events (DISABLED) - won't fix
        * [x] Files
        * [x] Guild details
        * [x] Item details
        * [x] Items
        * [x] Map floor
        * [x] Map names
        * [x] Maps
        * [x] Recipe details
        * [x] Recipes
        * [x] Skin details
        * [x] Skins
        * [x] World names
        * [x] WvW (returns 404/503 errors if requested, directly request the sub-endpoints below)
            * [x] Match details
            * [x] Matches
            * [x] Objective names

## Q&A
Q: Y u do dis?!

A: There simply didn't exist a fully functional API wrapper for the API, which I needed.
It's also a good learning experience in how to write wrappers, planning out project structure
and implementing tests. All in all; yay, I spent way too much time on this. (╥\_╥)

## Shoutouts
* [tyria-rs](https://github.com/rmed/tyria-rs) - where I stole some of the project structure from.
I would have used this instead if it were complete.
