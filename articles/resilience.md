


Frugal, nomadic environments were the default for me ever since I have started shaping, then creating them for myself. Initially it was really just a lack of recources, but luckily before broader alternatives became available to me I recognized the beauty in utilizing resources more resourcefully and having a more minimal footprint in general.
So far this approach has saved me a lot of money, enabled me to do a lot of cool stuff with stuff that some would consider trash and perhaps most importantly has forced me to understand the underlying mechanisms of hardware, materials, tools and processes. Perhaps even people.
I aways try to celebrate the moments when this approach helps me achieve something otherwise impossible withing the given constraints.
This post is one of those celebrations.

Due mostly to my stupidity one day after moving I have found myself without any sort of traditional computer. While this would not have meant the end of the world automatically, but I also had to demo a program I've been working on that day and I would have preferred to do that using a computer instead of colorful pieces of paper and crayons. I've heard from computer money people it is more "professional" that way.

What Ihad was:
    - stable electric outlets
    - ok wired and wireless internet
    - a VGA projector
    - a keyboard
    - a phone
    - a minipc (4GB RAM, 60GB storage)

First I thought of wiring up the box - it already had Debian installed - and calling it a day, but it turned out that I didnt't have a proper (DVI-VGA) cable for it to have a screen. No problem, I thought I can still just plug it in headless nad ssh into it farom my phone, but it did not boot for some reason and I did not want to go down on that rabbit hole just now.
Well I already had termux open on my phone and luckily I have rewritten the whole app from a nodejs server with terraform ducktaped to i to a relatively self-contained go app with a small SQLite db so it shouldn't really have problems. If the phone would not have worked out I could also just have spun up a cloud VM, but the app was interacting with AWS infra and did not have much consideration for security at that point so I would rather ran it locally.
To my surprise everything worked the first try. I even had some breaking changes at my last commit which I was able to fix up in no time - because VIM is everywhere and I never allowed myself to relax my frugal setup even when I had powerful enough machines to run VScode. As internet is somewhat scarce in the city I live in I have set up the code in a way that I can build a normal version of the binary and one that mocks the functions interfacing AWS so I can run it offline, and because it was already 1AM I did not end up risking some real API calls (still super paranoid of cloud costs) but I'll update if I can confirm that too.

Overall this was an amazing experience that reasurred me to keep things simple and also showed me how easy it is to develop simple graphical apps on my phone for my phone:
    I just need a rust/go websockets server, maybe a SQLite server and a simple vanilla SPA frontend.

To me this approach/lifestyle/philosophy means that while initially I do have to invest more time to familiarize myself with my tools (which arguably has it's own benefits) I retain a lot more flexibility, freedom and control over "my stuff", because my projects and my workflow depends a lot less on external variables, and when I have to I want to make sure that I rely on things which I can just pop the hood of in case something beeps in a mean way and I can service/cusomize it to my taste.
