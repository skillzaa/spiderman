# spiderman - An event based string parser.
2021-11-12
---
> Spiderman can be used as a simple parser or can be used as a framework for building your own parsers.
---
## Basic Idea:
The spiderman scan the provided text line by line (in future I may add word by word scans as well).
Every line is checked according to **Triggers** created by the user. 
Once such a line is found a function (*even handler function*) that is provided by the user is run.
In addition to Triggers the user also needs to create **Records** (as many as user wish) to record different parts of the string.
The **even handler** function gets a "bag of goodies" called **SpiderPack**. This pack has all the user recorders which he has created. 
The **SpiderPack** also has other items like **Flags** etc.
When we *Start a Recorder* it starts recording the lines being scanned.
When we **Stop the Recorder** the lines being scanned are skipped by the recorder.

## How It Works 
- We provide string to the SpiderMan.
- We create **Triggers** (events) for each line. A trigger needs 4 main informartions:
  1. Name of the trigger (to identify it later).
  2. What to look for (look_for) in each line.
  3. Where to look at (At::LineStart,At::LineEnd,At::Anywhere )
  4. The name of the function to run incase an event is fired (i.e such a line is found).
- We also creat **Records** to record portions of the text being scanned.
- When a trigger happens it has the pointer to an event handler function, which is executed.
- The function that you provide as **event handler** should have the following signature and should always return true. Incase it fails to return true the process will stop.
``` rust
fn(spider_pack:&mut SpiderPack)->bool
```
*For the time being spiderman just uses line by line scan of the string provided. In future I may add word by word iteration also.*

---
# Example
---

### Notes for me
- the auto_append is seperate from **every** function. The **every** event handling call on everyline only when the recorder is start.
- once a recorder is start
  - it can auto append 
  - AND
  - use the every function.
  - it can do both which ever the user want.
  - The default behaviour of auto append is true. but as long as the record is start.!!!!
- the user can Also not append unless the recorder is start.

- Just like **every** there is an **eof** fn with every recorder that will be run at the end of file.

## New Features
1. recorder :: Skip 1,2,3 for the recorder
1. recorder :: auto_append
1. recorder :: every function to recorder
1. recorder :: eof function to recorder
1. recorder :: write to file
1. recorder :: skip white lines: this flag will not append empty lines during auto_append.