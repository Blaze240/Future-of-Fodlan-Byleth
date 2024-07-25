# Soulshift Marth
## Introduction

Skyline plugin which switches Marth's outfit, weapon, and effects during certain animations. Inspired by Cecil Harvey's dual job trait and the "Proteus" effect from the Dissidia Final Fantasy series.

## Features
* Outfit swap to **Dark Knight** armor during the following moves:
  * Neutral Attack
  * Forward Tilt
  * Up Tilt
  * Down Tilt
  * Forward Smash
  * Up Smash
  * Down Smash
* Outfit swap to **Paladin** armor during the following moves:
  * Neutral Aerial
  * Forward Aerial
  * Back Aerial
  * Up Aerial
  * Down Aerial
* Falchion and circlet change to match the respective forms
* **Proteus** effect which randomly switches Marth's form during the following:
  * Dash attack
  * Wait animation
  * All taunts
  * Initial startup of Shield Breaker
  * First swing of Dancing Blade
  * Dolphin Slash
  * Counter (can switch forms between initial startup and counterattack)
  * All win screens
* Any animation/move not listed will use the form set to prior (applies to most movement animations and throws)
* Custom Effects
  * Darkness-based sword trails for Dark Knight Marth. These are duplicated and reimplemented into the .eff file via the effect converter.
  * Added the following effects to the .eff file via effect converter (not native to Marth):
    * Sephiroth's aura effect during Down Taunt (visible during Marth's taunts while in Dark Knight form)
    * Pyra's starting effect when switching to Mythra (recolored and spawned when switching to Dark Knight)
    * Mythra's starting effect when switching to Pyra (recolored and spawned when switching to Paladin)
 * Dynamic entry effects are controlled by a unique flag which is set when changing forms; if Marth is in one form then performs a move that would result in the same form, then the entry effect will not play.
   * **Example**: if Marth is already in Paladin form and performs any aerial, the associated change effect wil not spawn. However, if Marth was in Dark Knight form prior and does an aerial, then the associated Dark Knight spawn effect will play
* **Single slot capability**: the mod's functions are assigned in the motion_list.yml file, and the c04 folder can be renamed alongside the skin (note that this plugin will only work with the attached skin) 
