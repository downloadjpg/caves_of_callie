silly little roguelike i'm making in the bevy engine, biggest inspirations for this project are Caves of Qud and NetHack. 

# feature goals
- overworld terrain generation ala dwarf fortress
- run on web (https://github.com/bevyengine/bevy/tree/main/examples#setup-2)
- line of sight, map memory
- combat with stats
- looking at objects with detailed descriptions
-

# Architecture

### Core Loop
Speed-based turn system. An individual 'turn' goes as follows:

1. Initiative - Decide which actor is going next. Time 'advances' until the next ready actor.
2. Intent - The acting entity reads the world state to decide its next move
3. IntentionResolve - The intent is deciphered and relevant messages are written to various systems.
4. Resolution - The actions consequences take place.

This doesn't account for chain reactions (i.e., death causing an explosion) or any DoT system...
