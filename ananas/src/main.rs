/*
    The ananas application will transfer the data given from a file to
    NaN format in order to obscure it.
*/
use std::io::stdin;
use ananas::*;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::fs::OpenOptions;

/*
    Unsafe function that will convert a vector of type f32 to
    an array of tybe &[u8]
*/
fn vf_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

/*
    Converts a u8 to slice of f32
*/
fn u8_to_vf(u: &[u8]) -> &[f32] {
   unsafe{ let (_, ints, _) = u.align_to::<f32>(); return ints; }
}

fn main() {
    println!("Input file for conversion: ");
    let mut file_str = String::new();
    match stdin().read_line(&mut file_str) {
	Ok(input) => input ,
	Err(err) => {
			println!("Error reading file input. Error {}",err);
			return;
		    }
    };

    let mut file = match File::open(file_str.trim()) { 
	Ok(input) => input,
	Err(err) => {
			println!("Unable to open specified file. Error : {}",err);
			return;
		    }
    };

    let mut exten = ""; 
    for (pos,ele) in (file_str.trim()).chars().enumerate() {
	if ele == '.' {
	    exten = &file_str[pos..];

	}
    }

    if exten.trim() == ".nan" {
	let mut reread_data = Vec::new();
	let mut out_file = match File::create("data.str") {
	    			Ok(input) => input,
	    			Err(err) => {
			    			println!("Unable to create file data.str. Error : {}",err);
			    			return;
				            }
			   };
	match file.read_to_end(&mut reread_data) {
	    Ok(input) => input,
	    Err(err) => {
			    println!("error reading data to vector. Error: {}",err);
			    return;
			}
	};
	let nan_str = match nan_to_str(u8_to_vf(&reread_data)) {
		          Ok(input) => input,
			  Err(err) => {
				          println!("Unable to convert from NaN to string. Error: {}",err);
					  return;
				      }
        };
	match out_file.write_all(nan_str.as_bytes()) {
	    Ok(input) => input,
	    Err(err) => {
			    println!("Unable to write to file. Error: {}",err);
			    return;
			}
	};
    }
    else {
	let mut file_data = String::new();
    	match file.read_to_string(&mut file_data) {
		                Ok(input) => input ,
				Err(_err) => {
						println!("Error reading file.");
						return;
		    			    }
    	};
   
 
	let mut options = OpenOptions::new();
	let mut out_file = match options.read(true)
				        .write(true)
				  	.create(true)
				  	.truncate(true)
				  	.open("str2nan.nan") {
							         Ok(input) => input,
								 Err(err) => {
									         println!("Unable to create str2nan.nan. Error : {}", err);
										 return;
									     }
							     };

	let filedata_nans = bytes_to_nan(&file_data.as_bytes());
	match out_file.write_all(vf_to_u8(&filedata_nans)) {
	    Ok(input) => input,
	    Err(err) => {
			    println!("Unable to write to file. Error: {}",err);
			    return;
			}
	};

	let mut _file = match File::open("str2nan.nan") {
	    Ok(input) => input,
	    Err(err) => {
			    println!("unable to open specified file. Error : {}",err);
			    return;
			}
	};
	
    }

}

