use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile> <inputfile>", args[0]); 
    } else {
        let fname = args[1].clone();
        let path = Path::new(fname.clone());
        let msg_file = File::open(&path);

        let tname = args[2].clone();
        let path2 = Path::new(tname.clone());
        let msg_file2 = File::open(&path2);

        match (msg_file, msg_file2) {
            (Some(mut msg_file), Some(mut msg_file2)) => {
                let msg_bytes: ~[u8] = msg_file.read_to_end();
                let msg_bytes2: ~[u8] = msg_file2.read_to_end();
                println!("{:?}", msg_bytes2);
                println!("{:?}", msg_bytes);
                let share_file 
                       = File::create(&Path::new(fname + tname + ".combo"));
                
                match (share_file) {
                    Some(share) => { 
                        split(msg_bytes, msg_bytes2, share); 
                        } ,
                    _c => fail!("Error opening output files!"),
                }
            } ,
            (Some(msg_file), _) => { fail!("Error opening message file: {:s}", tname); },
            (_, Some(msg_file2)) => { fail!("Error opening message file: {:s}", fname); },
            (None, None) => fail!("Error opening message file: {:s} {:s}", fname, tname)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
		ret.push(a[i] ^ b[i]);
    }
    ret
}

fn split(msg_bytes: &[u8], msg_bytes2: &[u8], mut share: File) {    
    let unencrypted_bytes = xor(msg_bytes, msg_bytes2);
    share.write(unencrypted_bytes);
}