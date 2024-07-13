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