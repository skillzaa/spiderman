# API.
2021-11-12
---
- SPIDERMSN
 Trigger Syntax --- Builder Pattern
 let trig = spider.events.add("event_name)
 handler().look_for("##").at().line_end();  
 
- triggers
    - add/new  :method (event_name,event_handler)->Self
    - event_handler *This is the fn pointer*
    - kill :method (event_name)->bool
    - life  :property (till_end,once,x_times())
    - look_for :property &str
    - at :property
        - line_begin
        - line_end
        - line_mid
        - anywhere
       <!-- //already exists by default  -->
    -  line_begin     
    -  line_end    
    -  file_begin    
    -  file_end    
    <!-- -  half_done--later -->
    <!-- -  percent_done(x)--later -->
            <!-- no more -->

- Recordings
    - add (name)
    - kill
    - get 
    - Record
        - start (name)
        - stop/stop(name)
        <!-- - save_to_file
        - read_from_file -->
        - clear (clear the recording buffer)(name)
        - clear_recent: waste the latest buffer values(name)
        <!-- - append_text : before current buffer-later -->
        <!-- - prepend_text --later -->
        - write (write another data to this buffer)
        - read (read from this buffer)->return string
            <!-- stitch with another recording -->
        <!-- - stitch_record_prepend () -->
        <!-- - stitch_record_postpend () -->
        <!-- - stitch_file_prepend () -->
        <!-- - stitch_file_postpend () -->

<!-- Next feature -->
- Process
    - add(process_name,event_handler)  
    - start  
    - stop  
    - kill  
    - event_handler : *This is the fn pointer*  
---

// pub fn welcome(f: fn() -> String)->Option<u32>{

// pub fn welcome(){
// let name = "abc";    
// let mut spiderman = SpiderMan::new("sssewsaz".to_string());
// let tf = spiderman.recordings.add(name);
// let rec = spiderman.recordings.get(name)
// .unwrap();
// rec.start();
// rec.start();
// rec.start();
// rec.start();
// rec.start();
// println!("{:?}",rec.read());
// }
