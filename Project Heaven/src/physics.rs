const G = 10_f64;
//Sort all objects
//Called when rend3 puts object into active rendering
pub fn sortandstack(num: f32/*f32 so that we can show sub objects (ex satelite around planet, planet around star etc)*/, pos: Veec3, vel: Vec3, mass: f64) {
    //Read json into stack of tuple vec3 if file exists

    //Add new variables to thhe stack

    //Sort stack by number

    //Print stack to file
    //let writer_pos = BufWriter::new(File::create(file_path_pos).expect("path invalid"));
    //serde_json::to_writer_pretty(writer_pos, &pos_data).unwrap();
}

//Called every 1/60 second
fn calcnew(framerate: u8) {
    //Read json into stack of tuple vec3

    //Calculate new pos and vel for the first object with all the others
    //Put the new pos and vel for object 1 in new stack and 
    //move one down the stack and repeat
    let pos_new = ((pos.1 + ((1/framerate) * vel.1)), (pos.2 + ((1/framerate) * vel.2)), (pos.3 + ((1/framerate) * vel.3)));
    let vel_new = ((G*(mass.n/(pos.(n+1)-pos.n))*pos.1), (), ());

    //Write all new values to file and exit
}

fn splitstack(num: f32) -> (f32, Vec3, Vec3) {
    //read json with new values

    //take the nth var from the stack
    let placeholder = read_stack.num
    
    let pos = placeholder.1;
    let vel = placeholder.2;

    //Double check num
    if num == num_new {
        //return the value
        return (num, pos, vel);
    }
}