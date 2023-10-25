10/24/2023

10:19 PM

I don't have too much time today

I want to use some kind of http client/library to pull from a local API

10:26 PM

top of the list is hyper... let's go

10:31 PM

hmm... this tokio runtime

reqwest also uses it

mmhmm this is a good client... (Pulp Fiction reference)

looking at ureq, me likey

10:34 PM

holy sh it just works, hell yeah

I'm down with this, I am looking into the runtime/async thing though because that is a basic need

https://stackoverflow.com/questions/71357431/why-are-asynchronous-runtimes-like-tokio-necessary

ABI I'm learning new acronyms man application binary interface mmm

10:56 PM

distracted, I gotta learn some other stuff for tomorrow

gotta read up on async in rust

10/23/2023

7:18 PM

I have a headache right now but I did figure out what I want to make

It's pretty much a home bot, interfaces with all the random crap I have like my home camera, anemometer, different pis in the house on same network

The UI eventually will be ReactJS eg. Tauri but I'll focus on the backend stuff/Rust

10:00 PM

I'm going to start... silence

I will achieve something

10:02 PM

alright silence

10:05 PM

oh here we go, hash browns baby

`const` has to have types

`::` path separator, crates, modules and items

10:14 PM

ahh... let can't be used in global scope

10:31 PM

damn it... getting distracted bro

I'm stuck trying to make a string into an array lol

every time I edit it, it just gets worse lmao (the red suigglies)

10:36 PM

omg finally... no more red

10:40 PM

ahh... gotta split it out more

10:47 PM

holy f can't compare two strings

oh damn trailing new line nice

https://stackoverflow.com/a/38171882/2710227

good god this was so hard lmao and it sucks!

10:52 PM

I could use this

https://docs.rs/ctrlc/latest/ctrlc/

but not going to be a CLI in the end, will have a front end

11:00 PM

this is the output

```
Green@DESKTOP-OUAAKTM MINGW64 ~/projects/rusty-shenanigans/code (master)
$ cargo run
   Compiling code v0.1.0 (C:\Users\Green\projects\rusty-shenanigans\code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `target\debug\code.exe`
Home v0.0.0 - 2023-10-23
devices (typed in)
pi zero w, pi 2b
```

oh shit that makes sense, the major, minor, feature or whatever release numbering

---

10/22/2023

1:00 PM

back on but on main, big computer

1:03 PM

yeah I've been stuck on this trying to use chrono

made a cargo file didn't work

tried another way of importing

why does even the doc code not work lol ugh

1:08 PM

my lizard brain wants to scream, any code I try won't compile lol

code from the docs or SO

1:13 PM

holy s... okay I had to use `cargo cmd...` not the compiler

https://stackoverflow.com/a/60519381/2710227

f me man

1:16 PM

omg there we go... got the release going too nice

https://stackoverflow.com/a/60947511/2710227

okay so all the stuff will work just have to use cargo to do stuff not rust compiler

3:06 PM

I have lost motivation, briefly did something else for a freelance client but idk...

I want to make something but not sure what, so much hurdle too for something I can just sh out with JS

I'm not sure if I want an actual terminal dependency or make one but with web eg. Tauri

---

10/21/2023

ugh... lost my git history

I'm doing this on an Asus C100P Chromebook running Xenial XFCE on an SD Card on VS Code

I was able to get hello world running. VS Code file discovery is slow

I keep accidentally clicking somewhere I think because my thumb keeps grazing the touchpad

8:21 PM

geez what a waste of time using git ssh over git

I don't have a battery meter looks like

8:26 PM

alright added a battery icon to my panel

8:48 PM

I was thinking I could do the CLI thing which has API interfaces to my local RPis that hit the web eg. google spreadsheet

that would be a useful/familiar ground thing to learn Rust with

20:59 PM

I'm watching tv I will write something though, probably as I cook now, 55% charge

21:18 PM

having import issues with a crate, trying to use chrono

9:38 PM

lol I can't get past this chrono unresolved...

---

10/18/2023

10:48 PM

Please Jacob do it

hell yeah babyyyyyy hello world

I'm trying to make an "API" for like a note app, but it's just in memory

just to flex my code skills

it's like a text rpg game

11:20 PM

omg... my ADHD brain man, turn off social media

get prompt, that's what I want, then spit it back out

11:47 PM

omg I got sidetracked again damn

okay `&` is a reference there's also `*` to dereference hmm

12:12 AM

brief break

I think I can make like a CLI TODO list that would be good because I need to store shit, delete by id, etc..., make an array of strings, manipulate or an object



---

10/17/2023

10:51 PM

I'm pretty spent today and spent the remainder of my mental energy watching tv... so...

I did come up with something more tangible, back to the classics, a CLI CRUD app

that'll get me to start writing rust

---

10/16/2023

11:13 PM

tired af

gonna at least start the app, doesn't really count for progress

cargo mobile takes a while to install

I'm thinking about reprogramming my insect quad robot in rust, that would be a challenge

Also to actually do inverse kinematics to move the legs vs. manually-programmed gaits

11:20 PM

gotta update rust meesa got 1.64

11:24 PM

hmm don't know why I need a name unless it's that com stuff in android

also this `bevy` or `wry` what is...

bevy is game engine

whoa this egui thing is neat

wgpu graphics api

winit, window creation and management library

so I guess I have to use wry for android

11:31 PM

what is this NDK_HOME

11:37 PM

had to go into android studio

https://developer.android.com/studio/projects/install-ndk

install it from sdk manager along with cmake, maybe not necessary

11:46 PM

damn another problem

11:52 PM

hmm I got it to work but turning that developer mode toggle is sus, haven't done that ever on windows

11:55 PM

so far `cargo androind open` didn't work... `cargo run` is doing something

hmm... I might just stick with tauri desktop for now and React Native

11:59 PM

wait a minute... (Independence Day chess scene)

12:01 PM

I so sorry, I will revisit another time

---

10/12/2023

12:38 AM

I'm going to make a job hunting app, has defined requirements/made some before.

Will use Rust heavily hopefully and make a desktop app to refresh skills again.

12:46 AM

ehh... I don't see a design yet in my mind

I have to sqlite too, last time I tried I failed

<img src="./no-design-yet.JPG"/>
