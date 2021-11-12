# spiderman - An event based string parser.
2021-11-12
---
> Purpose

---
> The API
---
- SPIDERMSN
<!-- The lib is rdesigned based on line by line reading. -->
- Event
    - new  :method (event_name)
    - kill :method ()
    - life  :property (till_end,once,x_times())
    - execute: *This is the fn pointer*
    <!-- // isnt it anti patter ? -->
    - fire() 
- Event Types    
    - look_for_event: property (line_begin,line_end,line_mid, anywhere)
    -  every_line_begin    
    -  every_line_end
    -  file_end
    -  half_done
    -  percent_done(x)
            <!-- no more -->
- Recordings
    - new
    - start
    - stop/stop
    <!-- - save_to_file
    - read_from_file -->
    - clear (clear the recording buffer)
    - clear_recent: waste the latest buffer values
    - kill
    - append 
    - prepend
    - write (write another data to this buffer)
    - read (read from this buffer)->return string
        <!-- stitch with another recording -->
    - stitch_record_prepend ()
    - stitch_record_postpend ()
    - stitch_file_prepend ()
    - stitch_file_postpend ()

<!-- Next feature -->
- Process
    - new(process_name)  
    - start  
    - stop  
    - kill  
    - execute : *This is the fn pointer*  
---
