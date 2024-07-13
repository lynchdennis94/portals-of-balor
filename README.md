Portals of Balor
----
A Roguelike written for the "RoguelikeDev Does The Complete Roguelike Tutorial"

Based off of the Rusty Roguelike Tutorial - https://bfnightly.bracketproductions.com/

Including notes on work done for each week

Week One - Sections 2.0 and 2.1
-----
* Not including the resources folder _yet_. I plan on using a CP437 file from the dwarf fortress wiki as a placeholder, but for now I'll use whatever is being provided by default
* I'm using the new bracket_lib crate, instead of the rltk wrapper. I don't know that I'll necessarily want the extras included in the full bracket_lib (rltk is a portion of the wider library iiuc), but for the sake of giving me options and not requiring a massive rewrite later, I'll go with the current latest-and-greatest
* Defining unicode characters directly - in the tutorial, the 'smiley' face being supplied is written directly. Instead, I'm using the CP437 wiki (https://en.wikipedia.org/wiki/Code_page_437) to define the unicode directly
* Implemented additional keycodes for alternative movement schemes. I typically like using the numpad scheme (including using 5 as a no-op/rest button), and if I'm developing on a laptop without a keypad I want to have the vi-keys available for the full range of motion. The arrow keys implemented in the tutorial are fine, but limits the motion and makes me feel as though I'm getting only a portion of the game
* Added a tile size to the builder to create a bigger playing surface - the default was particularly small
