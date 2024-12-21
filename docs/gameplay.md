# Gameplay

## Juice

- Destructable objects, sending enemies through walls is so much fun in Redeemer, should have that too
- Experiment with camera responding to player (like dropkicking an enemy, should the camera move in a specific way?)

## AI

- `https://www.gameaipro.com/GameAIPro/GameAIPro_Chapter01_What_is_Game_AI.pdf`

- I tried to implement context steering at some point, but it didn't work well with the pathfinding (i.e. it lead to bugs). But I just thought why not use context steering only when you have clear line of sight? That way pathfinding is not obstructed by context steering but we can still make use of it to avoid obstacles while stalking around the player.
- Don't you fucking dare to put bullet sponges in the game, bro just play Redeemer and you know what **not** to do
- I really think that multiple factions in the game would be sick, something like in Streets of Rogue (different gangs, police, mafia, etc)
    - `https://www.youtube.com/watch?v=RCc78olN0hs`

### Map Collision/Navmesh Implementation

- Save the collision and navmesh data in a separate file (something handrolled).
- Store a hash of the whole data for each level as well
- Then before you calculate the level data, check the hash, if they are equal (nothing changed) then just go to the next
- Else go ahead and calculate the collision and navmesh node data for the level, then store it in some data struct (something simple will do)
- Finally when everything is done write the struct to the file

- In game you can load the level data into a Res struct on boot
- Then when you need to reference a level just get the Res with the level index or something and boom, there you got all your level data
    - Note that I might need to tweak this if it takes up too much memory, I mean it's only a bunch of vertices so it _should_ be fine, but you should extrapolate how much memory it will cost for like 200 levels or something

## Attacks

### Aim Assist

It may be necessary to snap the players attack to an enemy target. How exactly to implement this can be tricky (from a design perspective), this video covers it pretty well `https://www.youtube.com/watch?v=yGci-Lb87zs`.

### Charging Attacks

Bro, hear me out. Holding down light/heavy attack will trigger a "charged version", for the light attack it will be a continous stream of strikes that start slow and get faster and faster (like a gatling gun), the player is moving forward while doing this, similar to the one attack from wizards of legends with the ice melee attack. The heavy attack charges until a max is hit, when the player releases the character first stomps on the ground real hard, sending all his enemies around him to get fling into the air (though no knockback, just up), then the player jumps up and performs a sick ass all round house heel kick, sending all the enemies into oblivion.

I think this would also make for a great trailer ending.

KILL. THEM. ALL. Then baaam, the hit of the heavy attack or the light attack getting faster and faster with each word.
LEAVE. NO. SURVIVORS.
EVERYBODY. MUST. DIE

## Skills

Not really sure if I want to _implement_ these, but it would be cool to _play_ with these (at least in my head). So what I am thinking are skills that you use and then are on cooldown, really showy stuff.

- Link enemy-damage, when you punch on enemy, all linked enemies take damage the exact same way (same amount, same direction etc.)
    - Side note: I actually got this while debugging just now, there seems to be some kind of shared state that makes enemies either share hurtboxes or their state
- **DOMAIN EXPANSION**, bro that would be sick, maybe slow down time and do the Gojo thing he did in Shibuya, where you go through all enemies killing them
    - Could also be made so that it's a mix between Juggernauts ulti from Dota 2 and Gojo, so you slow down time, then the player is positioned in front of some enemy, he presses some attack button which pretty much instantly kills the enemy (maybe in like 0.1s), then telepored to the next enemy, then rince and repeat until either all enemies are dead in the AOE or the time runs out
