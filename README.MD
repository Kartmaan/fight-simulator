# fight-simulator
This program simulates a turn-based fight between two entities, each with its own attack and defense characteristics.

This project was carried out as part of a **personal training** in Rust. The main goal was to familiarize oneself with as many concepts of the language as possible :
* `structs`
* `enum`
* `trait`
* `HashMap`
* `lazy_static`
* `rand`
* Generic types
* ...

## Features
* Bestiary of mobs with their own characteristics that can be easily called to put them into fights
* Player with different classes (warrior, archer) each directly influencing their characteristics with bonuses and penalties.
* Simulation of a turn-based fight between the player and a mob or between two players or two mobs.
* Calculation of damage based on the attributes of the player and the mob like armor value, HP, critical damage probability, etc.
* Stylized display of information each round until one of the fighters wins

## Screenshots
Fight between a Warrior (Lost) and an Archer (Duriel) :
![cap](https://github.com/user-attachments/assets/522201b3-faac-434b-9940-9ce91c03ab4b)
