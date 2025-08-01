# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0] - 2025-07-25

### Added

- Add npc endpoint (#103)

### Fixed

- Update CI and add a fetch to avoid pipeline failures, update deps (#102)
- Critical path error with names and nicknames

## [2.7.0] - 2025-06-30

### Feature

- Handle complex resistances (#98)
- Change rand library (#99)

### Fixed

- Shop items shadowed by creature items 

## [2.6.0] - 2025-03-29

### Fixed

- Optimize boot time (#93)

## [2.5.0] - 2025-02-19

### Feature

- Improve spells (#89)
- Return focus points (#90)
- Return heighten level (#91)

## [2.4.0] - 2024-12-15

### Added

- Add senses data (range, acuity, vision) (#82)

### Feature

- Align bestiary list api with shop (#83)
- Improve encounter random generator filters (#85)
- Allow ternary operator for attack filter (Optional bool) (#87)

### Fixed

- Persistent startup not loading roles (#86)

## [2.3.0] - 2024-11-11

### Added

- Add docs endpoint, update README.md, allow passing db_path and env_path to start function

### Documentation

- Improve swagger, remove random example data. update dependencies (#76)

### Feature

- Introduce lib compiling, allow clean (slow) startup or persistent (quick but does not consider db update)
- Revamp build system (#79)

## [2.2.0] - 2024-10-02

### Feature

- Filter by trait (#73)
- Template refactor (#74)

## [2.1.0] - 2024-08-26

### Feature

- Adventure group and improve random creature fetching(#69)

## [2.0.1] - 2024-07-23

### Fixed

- Solve encounter generation regression (#67)

## [2.0.0] - 2024-07-22

### Feature

- Add item endpoint (#61)
- Add in creature endpoint PWL and refactor creature variant, minor fixes (#64)

## [1.3.0] - 2024-06-01

### Added

- Add pf version filter, remove alignment from trait list (#56)
- Add changelog and changelog CI (#58)

### Feature

- Introduce pwl, remove lazy static where used. (#47)
- Refactor Creature response according to new db and remastered (#48)
- Introduce Creature Role (#49)
- Refactor DB communication, remove moka cache (#51)

## [1.2.0] - 2024-01-17

### Feature

- Allow variants on encounter generation (#37)

### Fixed

- Change default from any to no alignment (#39)
- Any not converted correctly from db string (#40)

## [1.0.0] - 2023-11-16

### Added

- Add gunicorn support, expand config usage, fix docker image
- Add encounter info: encounter difficulty, exp and difficulty thresholds
- Add endpoint to fetch creature fields (family, rarity, size, alignment), page size must be greater or equal than 1, generalize fetch_keys redis_handler method
- Add method to procedurally create endpoints ending with or without /
- Add weak and elite endpoints
- Add melee, ranged, spell caster and sources fields. add melee, ranged and spell caster filters (#14)
- Add health controller
- Add get filter routers, use dbcache filters field, separate a little bit the cache logic and router initialization
- Add elite and weak modifier. Complete bestiary
- Add swagger at swagger-ui endpoint
- Add get_encounter_info logic

### Feature

- Backend is now deployment ready using docker and fly.io
- Implement random encounter generator, redis_handler can now fetch ids that pass filters
- Allow encounter generation with no filter, difficulty is randomized and the rest ignored
- Introduce get creature API, add filter name to list API
- Add sort and label filters  (#6)
- Add level and hp (min, max) filters (#9)
- Refactor using rust
- Implement cache (that needs to be refactored, it's ported 1:1 from python code)
- Implement full feature routing for bestiary list, with filters and default values.
- Allow request without pagination
- Implement encounter generation in rust (#16)
- Refactor cache to make it more rusty. This has the problem that it will have a slow first call every 24hours for one unlucky user
- Parse sources and return them correctly
- Parse complex sources strings
- Implement traits (#19)
- Implement creature types (#21)
- Implement multiple filter values (#23)
- Implement cache control (#25)
- Implement SQLite db (#28)
- Implement source endpoint (#30)
- Implement generator filtering by number of creatures (#31)

### Fixed

- Now get_bestiary returns json instead of string
- Handle difference between negative levels when calculating encounter info
- Next element wrong address
- Allow CORS calls
- Simplify calculate_encounter_difficulty, fix logical bug with trivial difficulty
- Populate new_ids_dict correctly (#7)
- Side effect on creature cache (#11)
- Refactor to handle correctly json body on post
- Use of scan instead of keys, that raised an error when the number of keys is high
- Creature_id endpoint overriding rarities, families, size, alignment etc
- Encounter info returning impossible on trivial encounters
- Pagination taking more element than requested
- Ln string not in caps
- Encounter info broken with negative enemies
- Set filtering was doing union operation instead of intersection
- Filter correctly vectors containing integer with value < -1

[3.0.0]: https://github.com/RakuJa/BYBE/compare/v2.7.0..v3.0.0
[2.7.0]: https://github.com/RakuJa/BYBE/compare/v2.6.1..v2.7.0
[2.6.0]: https://github.com/RakuJa/BYBE/compare/v2.5.0..v2.6.0
[2.5.0]: https://github.com/RakuJa/BYBE/compare/v2.4.0..v2.5.0
[2.4.0]: https://github.com/RakuJa/BYBE/compare/v2.3.0..v2.4.0
[2.3.0]: https://github.com/RakuJa/BYBE/compare/v2.2.0..v2.3.0
[2.2.0]: https://github.com/RakuJa/BYBE/compare/v2.1.0..v2.2.0
[2.1.0]: https://github.com/RakuJa/BYBE/compare/v2.0.1..v2.1.0
[2.0.1]: https://github.com/RakuJa/BYBE/compare/v2.0.0..v2.0.1
[2.0.0]: https://github.com/RakuJa/BYBE/compare/v1.3.0..v2.0.0
[1.3.0]: https://github.com/RakuJa/BYBE/compare/v1.2.0..v1.3.0
[1.2.0]: https://github.com/RakuJa/BYBE/compare/v1.0.0..v1.2.0

<!-- generated by git-cliff -->
