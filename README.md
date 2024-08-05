Portals of Balor
----
A Roguelike written for the 2024 ["RoguelikeDev Does The Complete Roguelike Tutorial"](https://www.reddit.com/r/roguelikedev/comments/1dt8bqm/roguelikedev_does_the_complete_roguelike_tutorial/)

Based off of the Rusty Roguelike Tutorial - https://bfnightly.bracketproductions.com/

Including notes on work done for each week

Week One - Sections 2.0 and 2.1
-----
* Not including the resources folder _yet_. I plan on using a CP437 file from the dwarf fortress wiki as a placeholder, but for now I'll use whatever is being provided by default
* I'm using the new bracket_lib crate, instead of the rltk wrapper. I don't know that I'll necessarily want the extras included in the full bracket_lib (rltk is a portion of the wider library iiuc), but for the sake of giving me options and not requiring a massive rewrite later, I'll go with the current latest-and-greatest
* Defining unicode characters directly - in the tutorial, the 'smiley' face being supplied is written directly. Instead, I'm using the CP437 wiki (https://en.wikipedia.org/wiki/Code_page_437) to define the unicode directly
* Implemented additional keycodes for alternative movement schemes. I typically like using the numpad scheme (including using 5 as a no-op/rest button), and if I'm developing on a laptop without a keypad I want to have the vi-keys available for the full range of motion. The arrow keys implemented in the tutorial are fine, but limits the motion and makes me feel as though I'm getting only a portion of the game
* Added a tile size to the builder to create a bigger playing surface - the default was particularly small
* I'm building this in part to follow off of the tutorial for Rust; I'd also like to expand the gameplay mechanics and pull from some games I enjoy (Dark Souls/Elden Ring, The Elder Scrolls, Diablo II) and put these mechanics in a setting I'd find interesting (fantasy influenced by Celtic mythology). Hence the name! While the initial implementation will hew closely to the tutorial, I hope to branch out over time and incorporate these additional elements - so in future weeks, there may be some level of divergence

Week Two - Sections 2.2 and 2.3
----
* At some point I know the tutorial will clean up the 'magic numbers', but they're already annoying me; I've added the constants for map height and width and cleaned up references to these values that were input directly. I've also updated the terminal builder to use these constants directly
* I'm not _entirely_ sure I have the right placement/calls for the rect implementation. There's an existing rect implementation in bracket_lib, so I may have just made it more difficult to discern between the two. For now, I'm using the explicit super::Rect suggestion in the book, but this may be an opportunity to use the library in the future

Week Three - Sections 2.4 and 2.5 (bonus Sections 4.1, 4.3, 4.5)
----
* I'm not entirely clear on the read vs. write storage usage in the tutorial; for example, when defining the visibility system, the position components are handled via WriteStorage. But the visibility system shouldn't be updating position components, so I've modified this to only use ReadStorage.
* I skipped ahead a bit to section 2.8, where the spawner is defined. By incorporating the spawner for map creation, it lets me then jump ahead to the map generation section to start making more interesting maps.
* In the spawner code, I've defined the consts for map width and height as i32 vars; I don't know why the change to usize happened (possibly a refactor I missed when skipping ahead), but it would have required a bunch of downstream changes in existing code that I don't know we need. If I _do_ end up needing it, I'll make the change back.
* I _also_ needed to add in the spawner logic for monsters. Right now it's mostly a copy-paste from section 2.5; when I get to section 2.8 later, I'll redo this.
* Since I had extra time this week, I added in some of what I wanted to get done last week - more interesting maps! I implemented the following:
    * Section 4.1 - this let me refactor the code a lot into builders and made the other map-building sections easier. This _did_ require me dipping a bit into later parts of the code (specifically parts of Section 2.8) so I could put a spawner together. When I get to that part in week five, I'll review it to make sure I didn't miss anything related that wasn't relevant for the room generation, but without peeking ahead this section wouldn't have really been possible to follow
    * Section 4.3 - the first new map time (BSP Rooms), it wasn't _that_ much different from the 'simple rooms' look at the end of the day. But it let me dabble in some map creation
    * Section 4.5 - this map type (Cellular Automata) seemed unique enough to give my game more flavor, and if I have more time from now until the end I'd like to add other map types to continue giving more flavor. I wanted to add drunkards walk, but there was a lot relying on the Cellular Automata implementation so I figured I'd start here instead
* I did _not_ add the testing harness from Section 4.2 - while I think it's a useful tool and something I'll potentially leverage in the future, for the sake of building maps from a tutorial it didn't seem as necessary for this exercise.

Week Four - Sections 2.6 and 2.7
----
* There was a bug in the monster movement code, where monsters were still stacking up on each other if they both were next to each other, next to the player, and the player moved away. I believe this was because the blocked state was getting updated in the map indexing system, but NOT in the movement system. This meant that monster A would move, but the 'blocked' spot would be the old spot. The second monster, not seeing the spot as blocked, would ALSO move there (as it was the most logical spot), and the monsters would then be stacked. By updating the blocked spot for each monster when processing their move, we now have an updated blocked map.
* I chose not to use the optional scan-lilnes, since I'm not a huge fan of the look. I might revisit this later, but for now I'd prefer just the normal text output