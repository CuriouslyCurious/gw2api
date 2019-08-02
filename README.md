
# **WIP** gw2api
[![Build Status](https://travis-ci.org/CuriouslyCurious/gw2api.svg?branch=master)](https://travis-ci.org/CuriouslyCurious/gw2api)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License:Apache-2.0](https://img.shields.io/badge/License-APACHE-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Coverage Status](https://coveralls.io/repos/github/CuriouslyCurious/gw2api/badge.svg?branch=master)](https://coveralls.io/github/CuriouslyCurious/gw2api?branch=master)

`gw2api` is a **WIP** wrapper over the official Guild Wars 2 API written in **Rust** using
[reqwest](https://github.com/seanmonstar/reqwest) as the underlying request maker.

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
            * [ ] Amulet
            * [ ] Stats (Auth)
            * [ ] Games (Auth)
            * [ ] Standings (Auth)
            * [ ] Season
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
    * [ ] V1
        * [ ] Build
        * [ ] Colors
        * [ ] Continents
        * [ ] Event details
        * [ ] Event names
        * [ ] Events
        * [ ] Files
        * [ ] Guild details
        * [ ] Item details
        * [ ] Items
        * [ ] Map floor
        * [ ] Map names
        * [ ] Maps
        * [ ] Recipe details
        * [ ] Recipes
        * [ ] Skin details
        * [ ] Skins
        * [ ] World names
        * [ ] WvW (returns 404/503 errors if requested, directly request the sub-endpoints below)
            * [ ] Match details
            * [ ] Matches
            * [ ] Objective names

## Q&A
Q: Y u do dis?!

A: There simply didn't exist a fully functional API wrapper for the API, which I needed.
It's also a good learning experience in how to write wrappers, planning out project structure
and implementing tests. All in all; yay.

## Shoutouts
* [tyria-rs](https://github.com/rmed/tyria-rs) - where I stole some of the project structure from.
I would have used this instead if it were complete.
