# Unhaunter

A paranormal investigation game in an isometric perspective.

## How to Play

Your task is to expel the ghost(s) from a location (currently 1 ghost only).

To be able to do this, first you need to identify the ghost among 44 different
possible ghost types. Each ghost type interacts with your equipment in a
different way. Each piece of equipment is responsible for detecting a type of
evidence.

There are 8 types of evidence. A ghost has 5 evidences out of those 8.

Locate the ghost, test the different equipment, note down in the van which
evidences you found, create the "Unhaunter Ghost Repellent" and use it to expel
the ghost.

Once you're done, you can click "End Mission" on the van and you'll get the
mission score.

Press [E] on the van to access the van UI for the journal and other useful
stuff. The same key is used to open doors or actuate switches and lamps.

Press [R] to activate the gear on your right hand, [T] for the gear in the left
hand.

Press [Q] to cycle the inventory on your right hand. [TAB] to swap your left
and right hands.

## Evidences

* Freezing temps: Thermometer. The room that the ghost frequents can go below
  zero. Be warned that lights heat up the room and open doors will leak air
  outside. These factors limit your ability to read freezing temps.

* Floating Orbs: Night Vision IR Camera. The ghost's breach (grey translucent
  dust) can illuminate under Night Vision (IR). The room has to be dark for this
  effect to be perceptible.

* UV Ectoplasm: UV Torch. The ghost might glow green under UV light. Other light
  sources might make this very hard to see.

* EMF Level 5: EMF Meter. The EMF Meter might read EMF5 for some ghosts.

* EVP Recording: Recorder. The recorder might show up a "EVP Recorded" message.

* Spirit Box: Spirit Box. Screams and other paranormal sounds might be heard
  through the static.

* RL Presence: Red Light Torch. The ghost might glow orange under this light.
  Other light sources might need to be off for the effect to be evident.

* 500+ cpm: Geiger Counter. The device might read above 500cpm for some ghosts.

## Basic Strategy

First, locate where the ghost is - what room it tends to roam. For this, it is
best to turn on all the lights in the house and open all doors.

Press [T] to enable the torch; it has several power settings, but be warned
that it might turn itself off by overheating.

The ghost has a spawn point, which also sets its favorite room. This spawn point
is known as the ghost's breach, which can be seen as a form of white,
semi-transparent dust. This is better seen using the location's lights; a
regular torch will not show it.

The ghost always roams around the breach. So most of the analysis and tests
should be done in this room.

Once this is located, turn off the lights of that room and the rooms contiguous
to it. Also, close the doors of the room.

Try the different equipment and note which ones gave positive results.

Go to the van to note these down in the journal. It is not possible to note
these down outside of the van, so if you cannot memorize them, you'll need to
make more trips to the van to write them down.

As you set these on the van, the list of possible ghosts will narrow. Once
you're sure which one it is, select it and you'll be able to click 
"Craft Unhaunter Ghost Repellent".

This will fill a vial in your inventory with the repellent for that particular
ghost type. Go to the ghost room (breach) and wait for the ghost to be there,
then activate the vial, which will spread the substance.

If successful, the ghost should disappear. Be warned, there's no cue indicating
if it worked. You need to double-check that the ghost is gone.
If needed, you can refill the vial in the van.

Once you're sure there are no more ghosts, proceed to the van
and click "End Mission".

## Building and Installing

There are no binary, packages or installers for Unhaunter. So far there are no
plans for them, the only way to run the game is to build it from sources.

Download the repository as usual via your favorite method, for example:

$ git clone https://github.com/deavid/unhaunter.git

You'll need to have Rust, follow the steps to install it at:

https://www.rust-lang.org/tools/install

You'll need also to install dependencies for Bevy, follow the instructions for
your operating system at:

https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies

With this, you should be able to run the game by simply running:

$ cargo run

The command must be run from the game source folder.

NOTE: The game is currently being developed and built on a single developer 
  machine using Debian GNU/Linux in some Testing/Sid version in an AMD machine
  using a NVIDIA 3080. The game should run in a wide range of computers and
  configurations, but it has not been tested.
