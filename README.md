# Rantoni Assets

Everything regarding the game Rantoni that doesn't directly have to do with the game is in here. It's more like a dirty repo where I don't exactly try to keep a super clean history or small git dir size.

## Timeline

### Final Prototype (pre-MVP)

Deadline: End of 2024

Fully functioning gameplay, player and 1 enemy, tileset of first area (rail yard, complete tileset) and also some props (doesn't need to include breakable proprs, static ones will do for this milestone). Art direction should be somewhat dialed in at this point.

Some key things will still be missing here:

- No dialogue or story (neither code nor content)
- No menu's or any UI
- No sounds
- No real polish except raw gameplay (and art as much as possible)

### MVP (small vertical slice)

Deadline: March 2025

- Polish the fuck out of the game
- Register as a business (talk with Steuerberater)
- Steam page
- Reach out to Anton

This is all about taking what we have and polishing it to the absolute extreme. We aren't going to add much more stuff in terms of content or features, just making sure that what we got is really good.

- Attacking effects and various other pixel art effects
- Sounds
- Music
- Tutorial
- Menus (main menu, options)
- Proper Title Screen/Main Menu
    - I am thinking something similar to CrossCode could work really nicely, see `https://youtu.be/MULE6YCemXs?t=4`
        - Probably way less good and without any of the animations (although they would be nice), but still something like that, player really close, looking far away to a city, setting probably night or at least evening or similar, the city obviously tokoy, actually for that think about something like `https://www.reddit.com/r/KatanaZero/comments/fwbbeu/katana_zero_desktop_setup/#lightbox`
        - If you do decide to go for animations, perhaps blood dripping of the fists of our hero and protagonist? Or street lights of the city moving (or lights in general)
        - Perhaps you could also do rain or something, with a thunder? Bro that would be sick, probably super hard to pull off though, also given that the images need to be huge this might be quite the memory investment, though wait, we only need them at the start of the game (or at least only when we are in the main menu), so we can just throw them out afterwards, yeah so not much of a problem after all I think
    - DomDom?! What do you think? You are up for the task?
    - Also Dead Cells has a really great title screen
    - Redeemer has a lot simpler title screen but that has great atmosphere
- QA (make sure there aren't many bugs, let other playtest)
- Polish all the visuals (shadows, tilesets, maybe characters)
- Gamemplay juice needs to be maxed
    - Hitstop
    - Input Buffering
    - Enemies on walks insta dying
    - Level transitioning working nicely
    - etc. etc.
- Figure out a name for the game dude

Probably a bunch more. I am hoping it won't take that long, but I also have a bunch of exams and stuff around February so I don't think there is going to be too much time for me to spend on the game. Though by my estimations the targeted deadline is achievable.

This is also the point at which we reach out to Anton and ask if he wants to join the team. Before we do that howver we would first create a steam page (still private if that is possible). Depending on his decision the further development will be changed.

### Demo (30min long vertical slice)

Deadline: End of May 2025

- Discord server
- Proper steam page (public)
- Proper Music
- Story
- Put demo on steam

Now we just add content to the game, new levels, new areas, new enemies, story etc. The target here is to have around an hour of gameplay or slightly shorter, then give this to a bunch of playtesters and get feedback on all parts of the game.

The game should be very close to the finished product, so dialogue and story and all that will be implemented during this cycle.

The steam page will also be made public (with trailer and everything). You should also start a Discord server at this point and make videos and shit about it. What about Lost Oppai? If Anton didn't join, then you can finish that and make a devlog and teaser our game at the end.

Here are some decisions that depend on whether Anton is in the team or not

- Steam capsule art, how much do you want to spend?
- Music, how much do you want to spend?
- What is the story gonna be?
- Will I make devlogs? Who is gonna make the trailer?
- How big is the game gonna be (how long)?

If Anton is not in, then I would say don't spend too much time overthinking any of these, don't spend too much money and don't expect a any financial successes. Also aim for a shorter game he didn't join, focus on getting it down rather then making it super good at this point.

On the other hand, if he did join, then you may expect a slightly more financial success and could due with investing a bit more in art and music. You could also try to go for a slightly longer game (though nothing too long, focus is still on finishing it within 1 year).

Also some words to whether or not to make a demo: I was at first quite conflicted about it, but I think a demo of around 30min could be very good. My main issue with it was that if somebody played the demo till the end and liked it enough to decide to buy the game, then they would need to play through the same thing again. But there are two ways to fix it: Just give a pop up at the start of the game that asks if the player has played through the demo and wants to skip the first section, or have the demo be different enough in terms of level and start (story maybe even) that it isn't that bad. I think the first option might be a bit nicer UX, if a player did not play the demo then he will just say no, if he did and doesn't want to skip it he still has the option to and if he does want to skip then there is that. I would prefer to not have that because I could imagine some players would get kinda anxious that they might miss something or whatever but oh well. Maybe this really isn't that bad of a problem to be honest and this just aint's an issue, should probably get feedback on this from players too.

### Steam Next Fest June 9 - 16

Note: All things considred Steam NF doesn't seem to be all _that_ important (still good, but not the end all be all), smaller more specific events may be better, see `https://x.com/kylewbanks/status/1800920295148056789`.

Okay so if everything went according to plan at this point, you kept all your deadlines and the player feedback was really positive and there weren't many bugs, then you can aim for the June Next Fest. You already have a demo, so you can just use that.

If you didn't keep the deadlines or the feedback was extremely negative or there are a tone of bug reports, then don't go for this Next Fest. There will be on sometime in October most likely. The initial time exstimation by me was wrong and you will **not** be able to finish the game within a year. Take your time, fix the bugs, improve the game, add more content, get feedback.

Push the release date by a few months, aim for a launch somewhere in Novemeber.

### ???

So what next? Well I don't fucking know, by this point this is well over half a year of me trying to predict the future bro. Think for yourself!

## Gameplay

### Statemachine

- `https://www.gamedev.net/forums/topic/639005-how-to-handle-states-in-a-fighting-game/`
- `https://www.gamedev.net/forums/topic/637975-what-makes-a-good-beatem-up-game/`

### AI

- `https://www.gameaipro.com/GameAIPro/GameAIPro_Chapter01_What_is_Game_AI.pdf`

- I tried to implement context steering at some point, but it didn't work well with the pathfinding (i.e. it lead to bugs). But I just thought why not use context steering only when you have clear line of sight? That way pathfinding is not obstructed by context steering but we can still make use of it to avoid obstacles while stalking around the player.
- Don't you fucking dare to put bullet sponges in the game, bro just play Redeemer and you know what **not** to do
- I really think that multiple factions in the game would be sick, something like in Streets of Rogue (different gangs, police, mafia, etc)
    - `https://www.youtube.com/watch?v=RCc78olN0hs`

#### Pathfinding

- `https://www.reddit.com/r/roguelikedev/comments/3slu9c/faq_friday_25_pathfinding/`
    - I think this is a great source of information on all sorts of topics

- `https://www.radicalfishgames.com/?p=186`
- `https://www.radicalfishgames.com/?p=498`
- `https://github.com/bevyengine/bevy/discussions/558`
- `https://www.youtube.com/watch?v=J7mFjlyScHA`

### Attacks

#### Aim Assist

It may be necessary to snap the players attack to an enemy target. How exactly to implement this can be tricky (from a design perspective), this video covers it pretty well `https://www.youtube.com/watch?v=yGci-Lb87zs`.

#### Charging Attacks

Bro, hear me out. Holding down light/heavy attack will trigger a "charged version", for the light attack it will be a continous stream of strikes that start slow and get faster and faster (like a gatling gun), the player is moving forward while doing this, similar to the one attack from wizards of legends with the ice melee attack. The heavy attack charges until a max is hit, when the player releases the character first stomps on the ground real hard, sending all his enemies around him to get fling into the air (though no knockback, just up), then the player jumps up and performs a sick ass all round house heel kick, sending all the enemies into oblivion.

I think this would also make for a great trailer ending.

KILL. THEM. ALL. Then baaam, the hit of the heavy attack or the light attack getting faster and faster with each word.
LEAVE. NO. SURVIVORS.
EVERYBODY. MUST. DIE

### Skills

Not really sure if I want to _implement_ these, but it would be cool to _play_ with these (at least in my head). So what I am thinking are skills that you use and then are on cooldown, really showy stuff.

- Link enemy-damage, when you punch on enemy, all linked enemies take damage the exact same way (same amount, same direction etc.)
    - Side note: I actually got this while debugging just now, there seems to be some kind of shared state that makes enemies either share hurtboxes or their state
- **DOMAIN EXPANSION**, bro that would be sick, maybe slow down time and do the Gojo thing he did in Shibuya, where you go through all enemies killing them
    - Could also be made so that it's a mix between Juggernauts ulti from Dota 2 and Gojo, so you slow down time, then the player is positioned in front of some enemy, he presses some attack button which pretty much instantly kills the enemy (maybe in like 0.1s), then telepored to the next enemy, then rince and repeat until either all enemies are dead in the AOE or the time runs out

### Playtesting, QA

Testing niddy griddy like player stats (movement speed, attack speed bla bla bla) can be easily done by having a txt file or ron file and letting the testers just adjust those values and then hot reload the game state to reflect the changes.

Also allows for easily copy/pasting configs.

### Juice

- Destructable objects, sending enemies through walls is so much fun in Redeemer, should have that too
- Experiment with camera responding to player (like dropkicking an enemy, should the camera move in a specific way?)

## Game References

Overall a great reference for what NOT to do is in the game Redeemer, boy oh boy does that game screw up (though such a juice). Not sure if this is a good reference overall `https://www.youtube.com/watch?v=YCjNT9qGjh4`.

- Redeemer
    - For game feel and gameplay
- Midnight fight express
    - gameplay
- Hades
    - Partially for game feel
    - Animation statemachine
- Wizards of Legends
    - Animations
- Akane
    - Not a great gameplay reference, though I think the enemies have really small anticipation, how does that work?
- CrossCode
    - Leveldesign
- Katana Zero
    - Absolutely amazing tutorial
    - Also game juice
- Streets of Rogue
    - Factions and simulation based chaos/complexity
    - Giving different AI's different rules can result in very interesting and fun situations

## Art References

- Akane
    - General vibe
- `https://guttykreum.itch.io/dark-tokyo-game-assets`
    - For the tileset, something like this should work, I also realized we don't really need 2x2 tileset mapping, it's modern so there are clear cuts pretty much everywhere
- Katana Zero
    - General vibe
- Dead Cells
    - `https://www.gamedeveloper.com/production/art-design-deep-dive-using-a-3d-pipeline-for-2d-animation-in-i-dead-cells-i-`
    - `https://www.youtube.com/watch?v=iNDRre6q98g`
    - `https://stackoverflow.com/questions/70362019/how-to-vectorize-an-image-using-python`

### Blender Stuff

- `https://www.youtube.com/watch?v=AQcovwUHMf0`

## Logic References

- CrossCode
    - `https://www.radicalfishgames.com/?p=498`

### Map Collision/Navmesh Implementation

- Save the collision and navmesh data in a separate file (something handrolled).
- Store a hash of the whole data for each level as well
- Then before you calculate the level data, check the hash, if they are equal (nothing changed) then just go to the next
- Else go ahead and calculate the collision and navmesh node data for the level, then store it in some data struct (something simple will do)
- Finally when everything is done write the struct to the file

- In game you can load the level data into a Res struct on boot
- Then when you need to reference a level just get the Res with the level index or something and boom, there you got all your level data
    - Note that I might need to tweak this if it takes up too much memory, I mean it's only a bunch of vertices so it _should_ be fine, but you should extrapolate how much memory it will cost for like 200 levels or something

## Expenses

- Steam Capsule art
    - Really don't want to spend more than 500 bucks on this
    - Don't be too cheap with this, also we can't really do this later, better to get it right the first time
- Music
    - Bro if I can get noisecream to work on this... dude
    - Though I really, REALLY don't want to spend more then 500 bucks on this, preferable is something like 300 but I doubt that will work
    - Though this can also be budgeted like max 500 at first and then, if there are more whishlists than expected, you can take a bit more cash in the hand and get some more songs, this way you have somewhat of a dynamic budget and don't overhire as badly
- Legal shit? Should we talk with a lawyer or do we need to register a company (GmbH?)

## Marketing

- Steam Page
- Demo
- Reach out to Content Creators
- Online Festivals

### References

- `https://www.youtube.com/watch?v=UJiv14uPOac`
- `https://www.youtube.com/watch?v=EMGTcgsEN68`
- `https://www.youtube.com/watch?v=ht6xx9en-ZU&t=21s`
- `https://www.youtube.com/watch?v=57wxdbJHeng`
- `https://www.reddit.com/r/gamedev/comments/1gere3c/my_first_game_had_more_than_45m_views_on_youtube/`
- `https://github.com/tutsplus/Marketing-Checklist-For-Indie-Game-Developers/blob/master/Indie%20Game%20Developer%20Marketing%20Checklist.md`
- `https://unitycodemonkey.com/video.php?v=dsXrUHWt3bQ`
- `https://unitycodemonkey.com/howtomarketagame.php`
- `https://www.youtube.com/watch?v=hTG1B5mIRDk`
- `https://www.youtube.com/watch?v=-MH5OTWl3zA`

### Specific unique marketing

- Reach out to Ranton, tell him about the game and also ask if he wants to be part, as for example a story writer (also partially marketing in terms of videos or similar content creation)
- Regardless of whether or not Ranton will actually agree to join, we can definitely do some posts on his subreddit as the game was inspired by him
- Events or leaderboard stuff on discord or some similar form of competition (probably not because it's too much work)
- Lost Oppai Game Dev with Rantoni mention at the end (probably kinda cheeky, but oh well)
    - In this case doing a little bit of marketing for Lost Oppai may actually end up helping this game as well, though probably only way too indirectly and not really worth the extra effort? Well you could at least post it on yarnspinner discord
    - Lost Oppai game dev log with a reveal of the game at the end (though only once the steam page is up!)
- Bevy Community
    - Share on main bevy discord server (in showcase channel for example, if you do gamedev logs then post there as well)
    - Possibly post on some more niche bevy servers like specific crates?
    - Put game on assets page
- If you actually manage to finish this game, consider write Mr. Prime an email telling him about it and that he was a major influence (like a mentor). This isn't really about marketing and more about gratitude, but oh well I just put it here. Also if he notices it and streams it I guess that would be somewhat marketing, though I don't want to write an email with this pretence, so maybe don't.

### General Stuff

- Localize steam page!

Just some rough plans for what you can do for marketing. Will probably not be able to do everything.

- Set up press kit and host on server
- Discord server
- Send out steam keys to streamers/youtubers and ask them to play the game
    - You can also just tell them about your demo, that seems to be more common anyways
- Reach out to press/blogs etc
- Youtube Videos (game dev logs) about the game
    - Preferably not this, it is unlikely to lead to much

### Steam Page

- Absolutely look at games with similar genre and what they are doing
- For the thumbnail, look at
    - Midnight Fight Express
    - Suit for Hire
    - RUINER
    - MADNESS: Project Nexus
    - Akane
    - Redeemer
    - This makes the point: `https://youtu.be/hTG1B5mIRDk?t=1082`

## Level Design

### Biomes and Areas

Take a look at Midnight Fight Express if you are running low on ideas, they also got high rises which might provide some good reference.

- Sidestreets of Tokyo
    - This may be too difficult to do in a general way, perhaps reduce this to very specific situations, like for instance, you could just have a straight path (either horizontal or vertical), that would be very easy (lol, relatively) to make
    - Instead you could have something like 
- Interiors?
- City high rises (office buildings and rooms, playing both in the interior as well as on the roofs etc, looking down on the city)
- Sewers?
- Winter region? (Hokkaido maybe? Something with snow would look cool)
- Volcano? (Mr. Fuji? Would be sick, also great visuals)
- Some temple in the mountains (I am picturing some kind of late summer/autumn setting, colorful trees and beatiful nature)

- Shibuya?

Possible locations for gang hideouts:

- Abandoned Factory
- Warehouse by the Docks
- Rooftops of High-rise Buildings
- Old Parking Garage
- Rail Yard
- Construction Site
- Underground Parking Lots
- Forest or Park Hideouts
- Old, Closed Movie Theater
- Closed Subway Station
- Storage Unit Complex
- Deserted Hotel

## Story

Generally I am pretty open as to what the story should be, I do think that a linear story is very much necessary though. Have levels and then you just play through them until you reach the end. Our target audience isn't really that niche or specific, so a simple linear story line should be better suited for a broader audience.

Now regarding the actualy story, I was thinking something pretty similar to Redeemer, some Monk (with maybe some connections of his past to gang or something like that) then an event happens and he is out for blood to kill gang members. Generally that would probably work, could also go with a more philosophically interesting story but I don't really mind too much.

Some gameplay ideas that could influence:
    - If we end up including policeman and civilians then at the very end with the final dude let him incoorporate the stats of the player, something like `Final Badguy: "I have read the reports you know... 276. Fucking 276 civilians! You are not any better than us. No, no you are even worse! You are a monster!`, or something along those lines. I think that could be a pretty cool reflection on the players choices throughout the game and maybe also hit them with a curve ball because they probably didn't think much of killing civilians in a game like this.
    - If there is something like a _Game+_ then perhaps have slightly different dialogue in some cases? Maybe even have a different ending (like a "true ending" or something?). Something like that could easily double the playtime while only requiring a fraction of the time to create, although certianly not all players would play this mode, but it would be nice to give that to people who really enjoyed the game and want to play some more

If you end up implementing multiple gangs then you could do something like some gang wanting to team up with our protagonist, they meet up, they give the proposition, they shake hands, though after a few seconds the hand of the gang member explodes (chad crushed his hand with a hand shake), then says something like "I would never ally with scum like you.", further solidifying his disgust and hatred for gangs.

### How to Implement Story

- It should never **EVER** interrupt gameplay, preferably put them at the end of chapters, don't know if they should always have one, maybe cutscenes? Maybe just simple dialogue? Maybe not all need them, depends on the pacing of the game I guess
- Less is more, this game is about gameplay/mechanics first, everything else is second, story is just a little driving force, if the mechanics are ass than the story is not supposed to save it, the story must not overshadow gameplay
- Keep story segments short, probably less than one minute, towards the end longer segments are toleratable
- Do not, **EVER** start the game with a cutscene or stupid story shit, **EVER**, let the player _want_ to know more about the world, open with gameplay instantly, explain what stuff is about as you go
    - Oh, I just realized this is **exactly** what Breath of the Wild does (to be fair, the story of the game is basically already known, it's obvious that Ganon pulled some shinanigans etc etc, so it's not a fair comparision, Katana Zero is much closer to what we have here), and it's great (also Katana Zero does this a _tiny_ bit, like the first 2 minutes or so are tutorial and then story starts, use that as a reference, works much better)
    - `https://www.youtube.com/watch?v=4k4-CP3q2xw` take a look at this, an old build of the game katana zero, story starts right away, not good I think
    - `https://youtu.be/bups0ZUQdvc?t=308`, this is kinda talking about something similar
- No story before 15min, I feel like that is a decent mark, mabye even 30min (though the game will not be that long in the first place, so yeah something in that ballpark)
    - Okay so future me here, maybe that is a little much, I wouldn't put a hard limit on it, just playtest and see how it feels, also every player takes different time for each segment, keep that in mind
- Possible fix for the issue with no story in the first 15min: Have some kind of radio or TV or something that explains what is going on, like "oh there were gang fights all over tokyo last night with 71 casulties and bla bla bla", but that thing is completely optional, maybe even make it destructable, I am imagining some little ass speaker that just has a dialogue box and it gets destroyed one hit if the player attacks it, signaling to the player that it's not important to listen, but they do have the option to if they are very into lore and story and shit.
    - This could also solve the issue of having the player ask "why? what's the point?" without being so _in your face_ with it

### Possible stories

As I said, I am pretty open to anything, though for the case that I need to write the story myself, here are some potential ideas. Also note that in any story, it will only start **after** the gameplay, regardless of what you actually choose, the player will be left to wonder why he is killing these guys, only then can you start the story (again, it's not like the player is gonna play hours without any kind of story, just let him play the game for a few solid minutes).

- Protagonist is a former elitst fighter (monk/martial artist? something), but now lives a life secluded from any kind of civilization, only with his wife and daughter (or son? his family.). However, one day some kind of messanger arrives, to tell him that the world is in danger (or maybe not the world, maybe just his temple or something of his past, I really, *really* don't like these grandious stories where the hero has to save the entire world, keep it grounded man), so he has to set out to erradicate this danger for the sake of his families protection. You would leave your family in the protection of some old friend you trust, or something like that. What exactly happens here is kinda open, a betrayal? Probably too obvious, but some kind of interesting twist would be good.
- Protagonist is a former elitst fighter (monk/martial artist? something), but now lives a life secluded from any kind of civilization with his family, or well, used to live, they got all killed (or perhaps some got kidnapped?) so you set out to kill them all in act of vengance (or rescue the ones that were kidnappes?). I am not a huge fan of this, super flat story, also the villains are just super boring, story very similar to that of Redeemer.
- Protagonist is just some random guy that works out like crazy as an outlet for his anger (he is working some random shit job, like 9 to 5 office job with some stupid as boss), absolutely hating society and parts of his life but he is holding out somewhat due to workout out everyday, but he hates the weak man around him, the fact that his boss is such a mentally weak man with no integrity or pride or discipline. Then some event happens that make him snap, the whole idea of this plotline is based on epsiode 7 of DanDaDan, if you watch it you will pretty much know exactly what I am talking about. So anyways, this event makes him realize all the problems with society are due to weak man (or at least that his the conclusion our protagonist arrives at). And so begins his journey to eradicate all the weak man (essentially anybody that harms others for their own good). Maybe it could be even more twisted that the protagonist starts to also kill innocent people that just seem weak, like normal office workers that didn't do anything wrong, but from his perspective they also didn't do anything _right_, so perhaps at the start he is rescuing and helping those, but at the end he is just mercilessly killing everybody.
- Some kind of brotherhood-like friendship theme. Maybe the best friend of the protagonist gets into trouble with the gangs or something? Or some premise like that, so now our protagonist has to help him out? Yeah not sure, don't have anything concrete but I really like that kind of brotherhood/bromance vibe.

### Ideas for smaller Story Elements

- Player is walking into a new area, triggers a semi cutscene, boy is crying and player (in cutscene mode) approaches boy, asks what's wrong. Boy sees player and asks for help because the player looks very strong, boy asks player to follow him. They backtrack, seeing all the blood and dead bodies etc. that the player caused, the boy continues to cry, his pace is getting slower? (Not sure about this, should his pace get faster? I mean he sees all the dead bodies and what not..), anyways, music is very slow and somewhat sad, as they get closer to the goal, the boy start screaming for his brother, and we see it's one of the many corpses that the player has just created... how to wrap this up? dunno, maybe just fade out to black, in that case you need to put it this whole thing at the end of a level (which I think is a good move anyways).
- The thing with the exposition speaker at the start of the game

## The Name of The Game

Just some ideas for a name of the game rather than Rantoni.

- Amuka, similar name to Akane, got the idea from the word Akuma - reversed - Amuka. [Google search results](https://trends.google.com/trends/explore?date=now%201-d&q=Akuma,Amuka,Akane&hl=de) shows that `Amuka` isn't really searched for often, which I guess is good? So the connection to Akuma (devil in japanese) would be that the protagonist is getting viewed by his enemies is a devil or oni or some shit like that. So he got the nickname Amuka from them. Don't have strong feelings about this one though, idea came to while I was doing my morning farmer walks.

Generally, I think you should at least be aware of how easy it is to search for the game. For example, _Redeemer_ is a horrible name for a game, there are like a million different things associated with it so whenever you want to search something related to the game you need to search for _redeemer game_.

## Optimization

The biggest concern is memory. I would really like to stay below 2 gigs, and the biggest offender of this are the characters (player, enemies etc). There are two optimizations I can implement fairly easily that should reduce memory by more than 50%.

- Batch frames that are identical together and utilize trickfilms keyframe timestamps to encode the same frame data. Useful for stuff like idle animations or parries or later on enemy anticipation animations. **Expected Result: ~15**
- Really just don't load everything in at once. If you have enemies specific to one area only then only load them in at that area.

## Legal

### Localization

So this happened to Jonas `https://youtu.be/MaFpf3nmHmo?t=798`, I personally think that if you had a github repo with a clear license like MIT that said clearly that anybody that would contribute will have their work licensed under MIT would make this essentially a non problem? Though I guess it would be better to ask a lawyer for that? But if this were a problem then wouldn't all of open source be at risk?

## MISC

### Feedback

- So mock reviews are a thing apparently, see `https://youtu.be/g-geVoA8Xr4?t=793`
- Look at negative reviews on similar games, see `https://youtu.be/g-geVoA8Xr4?t=968`

### Random Shit

- `Anthony the Angry`, a boss that doesn't really have much to do with the story or anything else. Model based on Anton. Before the "boss fight" there is a bit of dialogue, bla bla, and then the fight begins, boss bar appears, epic music starts, Anthoni charges at you full speed, maybe even yelling in a local dialogue box a battle cry... then he trips and falls, puddle of blood starting to spread, his health bar goes from full to zero instantly, music cuts off. Thus, `Anthoni the Angry` was defeated, by the floor. So it's a fake boss battle.
- Oh yeah what about a fake puzzle? Maybe that's too much work.
