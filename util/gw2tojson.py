#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import requests
import json
import pathlib
import sys
from multiprocessing.dummy import Pool as ThreadPool

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

BASE_URL = "https://api.guildwars2.com"

endpoints = [
    "/v2/account",
    "/v2/account/achievements",
    "/v2/account/bank",
    "/v2/account/buildstorage",
    "/v2/account/dailycrafting",
    "/v2/account/dungeons",
    "/v2/account/dyes",
    "/v2/account/emotes",
    "/v2/account/finishers",
    "/v2/account/gliders",
    "/v2/account/home",
    "/v2/account/home/cats",
    "/v2/account/home/nodes",
    "/v2/account/inventory",
    "/v2/account/luck",
    "/v2/account/mailcarriers",
    "/v2/account/mapchests",
    "/v2/account/masteries",
    "/v2/account/mastery/points",
    "/v2/account/materials",
    "/v2/account/minis",
    "/v2/account/mounts",
    "/v2/account/mounts/skins",
    "/v2/account/mounts/types",
    "/v2/account/novelties",
    "/v2/account/outfits",
    "/v2/account/pvp/heroes",
    "/v2/account/raids",
    "/v2/account/recipes",
    "/v2/account/skins",
    "/v2/account/titles",
    "/v2/account/wallet",
    "/v2/account/worldbosses",

    "/v2/achievements",
    "/v2/achievements/categories",
    "/v2/achievements/daily",
    "/v2/achievements/daily/tomorrow",
    "/v2/achievements/groups",

    "/v2/backstory",
    "/v2/backstory/answers",
    "/v2/backstory/questions",

    "/v2/build",
    "/v2/characters",
    "/v2/colors",

    "/v2/commerce/delivery",
    "/v2/commerce/exchange",
    "/v2/commerce/exchange/coins",
    "/v2/commerce/exchange/gems",
    "/v2/commerce/listings",
    "/v2/commerce/prices",
    "/v2/commerce/transactions",

    "/v2/continents",
    "/v2/createsubtoken",
    "/v2/currencies",
    "/v2/dailycrafting",
    "/v2/dungeons",
    "/v2/emblem",

    "/v2/emblem/backgrounds",
    "/v2/emblem/foregrounds",

    "/v2/emotes",
    "/v2/files",
    "/v2/finishers",
    "/v2/gliders",

    #"/v2/guild (returns 404/503 errors if requested, directly request the sub-endpoints below)",
    #"/v2/guild/:id",
    #"/v2/guild/:id/log",
    #"/v2/guild/:id/members",
    #"/v2/guild/:id/ranks",
    #"/v2/guild/:id/stash",
    #"/v2/guild/:id/storage",
    #"/v2/guild/:id/teams",
    #"/v2/guild/:id/treasury",
    #"/v2/guild/:id/upgrades",
    "/v2/guild/permissions",
    "/v2/guild/search",
    "/v2/guild/upgrades",

    "/v2/home/cats",
    "/v2/home/nodes",
    "/v2/items",
    "/v2/itemstats",
    "/v2/legends",
    "/v2/mailcarriers",
    "/v2/mapchests",
    "/v2/maps",
    "/v2/masteries",
    "/v2/materials",
    "/v2/minis",
    "/v2/mounts",

    "/v2/mounts/skins",
    "/v2/mounts/types",

    "/v2/novelties",
    "/v2/outfits",
    "/v2/pets",
    "/v2/professions",
    "/v2/pvp",

    "/v2/pvp/amulets",
    "/v2/pvp/games",
    "/v2/pvp/heroes",
    "/v2/pvp/ranks",
    "/v2/pvp/seasons",
    "/v2/pvp/seasons/A54849B7-7DBD-4958-91EF-72E18CD659BA/leaderboards",
    "/v2/pvp/standings",
    "/v2/pvp/stats",

    "/v2/quaggans",
    "/v2/quests",
    "/v2/races",
    "/v2/raids",
    "/v2/recipes",

    "/v2/recipes/search",

    "/v2/skills",
    "/v2/skins",
    "/v2/specializations",
    "/v2/stories",

    "/v2/stories/seasons",

    "/v2/titles",
    "/v2/tokeninfo",
    "/v2/traits",
    "/v2/worldbosses",
    "/v2/worlds",
    "/v2/wvw",

    "/v2/wvw/abilities",
    "/v2/wvw/matches",
    "/v2/wvw/matches/stats/1-1/teams",
    "/v2/wvw/objectives",
    "/v2/wvw/ranks",
    "/v2/wvw/upgrades",

    "/v1/build.json",
    "/v1/colors.json",
    "/v1/continents.json",
    "/v1/event_details.json",
    "/v1/files.json",
    "/v1/guild_details.json",
    "/v1/item_details.json",
    "/v1/items.json",
    "/v1/map_floor.json",
    "/v1/map_names.json",
    "/v1/maps.json",
    #"/v1/recipe_details.json",
    #"/v1/recipes.json",
    "/v1/skin_details.json",
    "/v1/skins.json",
    "/v1/world_names.json",
    "/v1/wvw/match_details.json",
    "/v1/wvw/matches.json",
    "/v1/wvw/objective_names.json",

    # Disabled
    #"/v1/event_names.json",
    #"/v1/events.json",
    ]

endpoints = [BASE_URL + endpoint for endpoint in endpoints]

if __name__ == "__main__":
    pool = ThreadPool(8)
    responses = pool.map(requests.get, endpoints)
    pool.close()
    pool.join()

    good_status_codes = [200, 400, 401]

    responding = 0
    for response in responses:
        if response.status_code in good_status_codes:
            print(f"[{GREEN}{response.status_code}{NC}] {response.url} is... {GREEN}ok{NC}.")
            path = pathlib.Path(response.url.replace(BASE_URL, "").lstrip("/"))
            path.parent.mkdir(parents=True, exist_ok=True)
            if not path.name.endswith("json"):
                path = path.parent / (path.name + ".json")
            try:
                j = response.json()
                with open(path, "w") as f:
                    j = json.dumps(j, indent=4)
                    f.write(j)
            except requests.exceptions.JSONDecodeError:
                continue
            except TypeError:
                print(f"Could not parse {path} to json.")
                sys.exit(1)
            responding += 1
        else:
            print(f"[{RED}{response.status_code}{NC}] {response.url} is... {RED}not responding{NC}.")

    print("Endpoints responding: %d/%d = %d%%" % (responding, len(endpoints), responding/len(endpoints)*100))

